//! PubChem PUG-REST: search suggestions, name / CAS / CID lookup, and the
//! molecule header (title, formula, molar mass, synonyms).

use std::sync::LazyLock;

use regex::Regex;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::http::{Host, Http};

pub const BASE: &str = "https://pubchem.ncbi.nlm.nih.gov/rest";

/// How many synonyms the header shows.
pub const SYNONYMS: usize = 5;
/// Longer synonyms are systematic names, not something a person would say.
const MAX_SYNONYM_CHARS: usize = 30;

static CAS: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\d{2,7}-\d{2}-\d$").unwrap());

/// What the molecule header shows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compound {
    pub cid: u64,
    pub title: String,
    /// Plain text, like `C8H10N4O2`; the UI adds subscripts.
    pub formula: String,
    /// In g/mol, as PubChem formats it, like `194.19`.
    pub molar_mass: String,
    pub iupac_name: Option<String>,
    pub cas: Option<String>,
    pub synonyms: Vec<String>,
}

pub use crate::error::Error;

#[derive(Clone)]
pub struct PubChem {
    http: Http,
    base: Url,
}

impl PubChem {
    pub fn new(http: Http) -> Self {
        Self::with_base(http, BASE)
    }

    pub fn with_base(http: Http, base: &str) -> Self {
        Self {
            http,
            base: Url::parse(base).expect("valid base URL"),
        }
    }

    /// `base/a/b/...`, each segment percent-encoded.
    fn url(&self, segments: &[&str]) -> Url {
        let mut url = self.base.clone();
        url.path_segments_mut()
            .expect("base URL has a path")
            .pop_if_empty()
            .extend(segments);
        url
    }

    async fn json(&self, url: Url) -> Result<Value, Error> {
        let body = self.http.get(Host::PubChem, url.as_str()).await?;
        serde_json::from_slice(&body).map_err(|_| Error::Format("PubChem JSON reply"))
    }

    /// Compound names that start like `query`, best first.
    pub async fn autocomplete(&self, query: &str, limit: usize) -> Result<Vec<String>, Error> {
        let query = query.trim();
        if query.is_empty() {
            return Ok(vec![]);
        }
        let mut url = self.url(&["autocomplete", "compound", query, "json"]);
        url.query_pairs_mut()
            .append_pair("limit", &limit.to_string());
        Ok(parse_autocomplete(&self.json(url).await?))
    }

    /// The CID for a name, CAS number or CID; `None` if PubChem knows no match.
    pub async fn resolve(&self, query: &str) -> Result<Option<u64>, Error> {
        let query = query.trim();
        if let Ok(cid) = query.parse::<u64>() {
            return Ok(Some(cid));
        }
        let url = self.url(&["pug", "compound", "name", query, "cids", "JSON"]);
        match self.json(url).await {
            Ok(v) => parse_cids(&v).map(|cids| cids.first().copied()),
            Err(Error::Http(e)) if e.is_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// The molecule header for `cid`. Two requests: properties, then synonyms.
    pub async fn compound(&self, cid: u64) -> Result<Compound, Error> {
        let cid_s = cid.to_string();
        let properties = self
            .json(self.url(&[
                "pug",
                "compound",
                "cid",
                &cid_s,
                "property",
                "Title,MolecularFormula,MolecularWeight,IUPACName",
                "JSON",
            ]))
            .await?;
        let mut compound = parse_properties(&properties)?;
        let synonyms = self
            .json(self.url(&["pug", "compound", "cid", &cid_s, "synonyms", "JSON"]))
            .await;
        match synonyms {
            Ok(v) => {
                let all = parse_synonyms(&v);
                compound.cas = cas_number(&all);
                compound.synonyms = pick_synonyms(&compound.title, &all, SYNONYMS);
            }
            // Some compounds have no synonyms; the header still works.
            Err(Error::Http(e)) if e.is_not_found() => {}
            Err(e) => return Err(e),
        }
        Ok(compound)
    }

    /// The PUG-View reply for one annotation heading, or `None` when the
    /// compound has nothing under it.
    pub async fn view(&self, cid: u64, heading: &str) -> Result<Option<Value>, Error> {
        let mut url = self.url(&["pug_view", "data", "compound", &cid.to_string(), "JSON"]);
        url.query_pairs_mut().append_pair("heading", heading);
        match self.json(url).await {
            Ok(v) => Ok(Some(v)),
            Err(Error::Http(e)) if e.is_not_found() => Ok(None),
            Err(e) => Err(e),
        }
    }

    /// PubChem's computed 3D conformer as an SDF record, or `None` when it has
    /// none (very large or flexible molecules, metal complexes, C60).
    pub async fn sdf_3d(&self, cid: u64) -> Result<Option<String>, Error> {
        let mut url = self.url(&["pug", "compound", "cid", &cid.to_string(), "record", "SDF"]);
        url.query_pairs_mut().append_pair("record_type", "3d");
        match self.http.get_text(Host::PubChem, url.as_str()).await {
            Ok(sdf) if is_sdf(&sdf) => Ok(Some(sdf)),
            Ok(_) => Err(Error::Format("PubChem SDF record")),
            Err(e) if e.is_not_found() => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// A 500 × 500 2D drawing, for molecules without a 3D conformer.
    pub async fn png_2d(&self, cid: u64) -> Result<Vec<u8>, Error> {
        let mut url = self.url(&["pug", "compound", "cid", &cid.to_string(), "PNG"]);
        url.query_pairs_mut()
            .append_pair("image_size", &format!("{PNG_SIZE}x{PNG_SIZE}"));
        let png = self.http.get(Host::PubChem, url.as_str()).await?;
        if !png.starts_with(PNG_SIGNATURE) {
            return Err(Error::Format("PubChem PNG image"));
        }
        Ok(png)
    }

    /// What the viewer shows: the 3D conformer if there is one, else the 2D drawing.
    pub async fn structure(&self, cid: u64) -> Result<Structure, Error> {
        match self.sdf_3d(cid).await? {
            Some(sdf) => Ok(Structure::ThreeD(sdf)),
            None => Ok(Structure::TwoD(self.png_2d(cid).await?)),
        }
    }
}

/// Side of the 2D drawing in pixels; sharp at the panel size on Retina screens.
pub const PNG_SIZE: u32 = 500;
const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Structure {
    /// SDF text for 3Dmol.js.
    ThreeD(String),
    /// PNG bytes.
    TwoD(Vec<u8>),
}

/// A molfile record: a counts line in V2000 or V3000 format and an end marker.
fn is_sdf(text: &str) -> bool {
    (text.contains("V2000") || text.contains("V3000")) && text.contains("M  END")
}

/// Autocomplete returns registry codes too (`Y2T8N94ZQQ`); keep names only.
fn parse_autocomplete(v: &Value) -> Vec<String> {
    v.pointer("/dictionary_terms/compound")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .filter(|t| !is_code(t))
        .map(str::to_string)
        .collect()
}

fn parse_cids(v: &Value) -> Result<Vec<u64>, Error> {
    v.pointer("/IdentifierList/CID")
        .and_then(Value::as_array)
        .map(|a| a.iter().filter_map(Value::as_u64).collect())
        .ok_or(Error::Format("PubChem CID list"))
}

fn parse_properties(v: &Value) -> Result<Compound, Error> {
    let p = v
        .pointer("/PropertyTable/Properties/0")
        .ok_or(Error::Format("PubChem property table"))?;
    let text = |key: &str| p.get(key).and_then(Value::as_str).map(str::to_string);
    Ok(Compound {
        cid: p
            .get("CID")
            .and_then(Value::as_u64)
            .ok_or(Error::Format("PubChem property table"))?,
        title: text("Title").ok_or(Error::Format("PubChem property table"))?,
        formula: text("MolecularFormula").unwrap_or_default(),
        // Usually a string ("194.19"), occasionally a number.
        molar_mass: text("MolecularWeight")
            .or_else(|| p.get("MolecularWeight").map(Value::to_string))
            .unwrap_or_default(),
        iupac_name: text("IUPACName"),
        cas: None,
        synonyms: vec![],
    })
}

fn parse_synonyms(v: &Value) -> Vec<String> {
    v.pointer("/InformationList/Information/0/Synonym")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

/// A code rather than a name: one word of at least 6 characters, with digits and
/// no lowercase letters, like `Y2T8N94ZQQ` or `CHEMBL113`. Short ones like
/// `5-HT` are names people use.
fn is_code(term: &str) -> bool {
    term.chars().count() >= 6
        && !term.contains(' ')
        && term.chars().any(|c| c.is_ascii_digit())
        && !term.chars().any(char::is_lowercase)
}

/// PubChem lists the CAS number among the synonyms, usually second.
fn cas_number(synonyms: &[String]) -> Option<String> {
    synonyms.iter().find(|s| CAS.is_match(s)).cloned()
}

/// Names a person would recognize, in PubChem's order (most common first):
/// no digits (CAS numbers, codes, systematic names), short, and not the title
/// or an earlier pick again.
fn pick_synonyms(title: &str, synonyms: &[String], n: usize) -> Vec<String> {
    let mut picked: Vec<String> = Vec::new();
    for s in synonyms {
        let fresh =
            !s.eq_ignore_ascii_case(title) && !picked.iter().any(|p| p.eq_ignore_ascii_case(s));
        if fresh && s.chars().count() <= MAX_SYNONYM_CHARS && !s.chars().any(|c| c.is_ascii_digit())
        {
            picked.push(s.clone());
            if picked.len() == n {
                break;
            }
        }
    }
    picked
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use super::*;
    use crate::http::Settings;
    use crate::http::test_server::{response, serve};

    fn fixture(name: &str) -> Vec<u8> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/http/pubchem")
            .join(name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("missing {}", path.display()))
    }

    fn json(name: &str) -> Value {
        serde_json::from_slice(&fixture(name)).unwrap()
    }

    /// A local PubChem that serves recorded replies by path.
    async fn local_pubchem() -> (PubChem, crate::http::test_server::Log) {
        let (base, log) = serve(|_, path| {
            let file = match path {
                "/rest/autocomplete/compound/caff/json?limit=10" => "autocomplete-caff.json",
                "/rest/pug/compound/name/caffeine/cids/JSON"
                | "/rest/pug/compound/name/58-08-2/cids/JSON" => "cids-caffeine.json",
                "/rest/pug/compound/cid/2519/property/Title,MolecularFormula,MolecularWeight,IUPACName/JSON" => {
                    "properties-2519.json"
                }
                "/rest/pug/compound/cid/2519/synonyms/JSON" => "synonyms-2519.json",
                "/rest/pug/compound/cid/2519/record/SDF?record_type=3d" => "sdf-3d-2519.sdf",
                "/rest/pug/compound/cid/123591/record/SDF?record_type=3d" => {
                    return response("404 Not Found", fixture("sdf-3d-not-found.txt"));
                }
                "/rest/pug/compound/cid/123591/PNG?image_size=500x500" => "png-123591.png",
                _ => return response("404 Not Found", fixture("cids-not-found.json")),
            };
            response("200 OK", fixture(file))
        })
        .await;
        let http = Http::with_settings(Settings {
            backoff: Duration::from_millis(10),
            ..Settings::default()
        });
        (PubChem::with_base(http, &format!("{base}/rest")), log)
    }

    #[test]
    fn autocomplete_keeps_names_and_drops_codes() {
        let caff = parse_autocomplete(&json("autocomplete-caff.json"));
        assert_eq!(caff[..2], ["caffeine", "caffeic acid"]);
        assert!(parse_autocomplete(&json("autocomplete-zzzzqq.json")).is_empty());
        assert!(parse_autocomplete(&serde_json::json!({"status": {"code": 1}})).is_empty());
    }

    #[test]
    fn properties_and_synonyms() {
        let c = parse_properties(&json("properties-2519.json")).unwrap();
        assert_eq!(
            (
                c.cid,
                c.title.as_str(),
                c.formula.as_str(),
                c.molar_mass.as_str()
            ),
            (2519, "Caffeine", "C8H10N4O2", "194.19")
        );
        assert_eq!(
            c.iupac_name.as_deref(),
            Some("1,3,7-trimethylpurine-2,6-dione")
        );

        let caffeine = parse_synonyms(&json("synonyms-2519.json"));
        assert_eq!(cas_number(&caffeine).as_deref(), Some("58-08-2"));
        assert_eq!(
            pick_synonyms("Caffeine", &caffeine, 5),
            [
                "Guaranine",
                "Methyltheobromine",
                "Theine",
                "Thein",
                "Cafeina"
            ]
        );
        let quercetin = parse_synonyms(&json("synonyms-5280343.json"));
        assert_eq!(cas_number(&quercetin).as_deref(), Some("117-39-5"));
        assert_eq!(
            pick_synonyms("Quercetin", &quercetin, 5),
            [
                "Sophoretin",
                "Meletin",
                "Quercetol",
                "Xanthaurine",
                "Quercetine"
            ]
        );
    }

    #[test]
    fn codes_are_not_names() {
        for code in ["Y2T8N94ZQQ", "CHEMBL113", "58-08-2"] {
            assert!(is_code(code), "{code}");
        }
        for name in ["caffeine", "Vitamin B12", "5-HT", "Caffeic acid"] {
            assert!(!is_code(name), "{name}");
        }
    }

    #[tokio::test]
    async fn resolves_names_cas_numbers_and_cids() {
        let (pubchem, log) = local_pubchem().await;
        assert_eq!(pubchem.resolve("caffeine").await.unwrap(), Some(2519));
        assert_eq!(pubchem.resolve(" 58-08-2 ").await.unwrap(), Some(2519));
        assert_eq!(pubchem.resolve("notamolecule123").await.unwrap(), None);
        let requests = log.lock().unwrap().len();
        assert_eq!(pubchem.resolve("2519").await.unwrap(), Some(2519));
        assert_eq!(
            log.lock().unwrap().len(),
            requests,
            "a CID needs no request"
        );
    }

    #[tokio::test]
    async fn names_are_percent_encoded() {
        let (pubchem, log) = local_pubchem().await;
        let _ = pubchem.resolve("vitamin b12/x?").await;
        let head = log.lock().unwrap()[0].clone();
        assert!(
            head.starts_with("GET /rest/pug/compound/name/vitamin%20b12%2Fx%3F/cids/JSON "),
            "{head}"
        );
    }

    #[tokio::test]
    async fn structure_is_3d_when_pubchem_has_it_else_2d() {
        let (pubchem, _) = local_pubchem().await;
        match pubchem.structure(2519).await.unwrap() {
            Structure::ThreeD(sdf) => {
                assert!(sdf.starts_with("2519\n"));
                assert!(sdf.trim_end().ends_with("$$$$"));
            }
            other => panic!("caffeine has a 3D conformer: {other:?}"),
        }
        // Buckminsterfullerene has no 3D conformer in PubChem.
        assert_eq!(pubchem.sdf_3d(123591).await.unwrap(), None);
        match pubchem.structure(123591).await.unwrap() {
            Structure::TwoD(png) => assert!(png.starts_with(PNG_SIGNATURE)),
            other => panic!("expected the 2D drawing: {other:?}"),
        }
    }

    #[test]
    fn sdf_check() {
        assert!(is_sdf(
            &String::from_utf8(fixture("sdf-3d-2519.sdf")).unwrap()
        ));
        assert!(!is_sdf("Status: 404"));
    }

    /// Against the real PubChem: `cargo test -p valence-core -- --ignored live`.
    #[tokio::test]
    #[ignore = "network"]
    async fn live_lookup() {
        let pubchem = PubChem::new(Http::new());
        let cid = pubchem.resolve("capsaicin").await.unwrap().unwrap();
        let c = pubchem.compound(cid).await.unwrap();
        println!("{c:#?}");
        assert_eq!(c.formula, "C18H27NO3");
        assert!(!pubchem.autocomplete("capsa", 10).await.unwrap().is_empty());
        assert!(matches!(
            pubchem.structure(cid).await.unwrap(),
            Structure::ThreeD(_)
        ));
        assert!(matches!(
            pubchem.structure(123591).await.unwrap(),
            Structure::TwoD(_)
        ));
    }

    #[tokio::test]
    async fn header_for_a_cid() {
        let (pubchem, _) = local_pubchem().await;
        let c = pubchem.compound(2519).await.unwrap();
        assert_eq!(c.title, "Caffeine");
        assert_eq!(c.cas.as_deref(), Some("58-08-2"));
        assert_eq!(c.synonyms.len(), SYNONYMS);
        assert_eq!(
            pubchem.autocomplete("caff", 10).await.unwrap()[0],
            "caffeine"
        );
        assert!(pubchem.autocomplete("  ", 10).await.unwrap().is_empty());
        // Cubane has no natural sources section.
        assert!(
            pubchem
                .view(136090, "Natural Pollution Sources")
                .await
                .unwrap()
                .is_none()
        );
    }
}

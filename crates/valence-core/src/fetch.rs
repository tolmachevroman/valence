//! Fetch everything for a molecule, streaming each part as it arrives, and
//! serve it from the cache while fresh.

use std::sync::Mutex;
use std::time::SystemTime;

use tokio::sync::mpsc::UnboundedSender;

use crate::cache::{Cache, Molecule, unix_seconds};
use crate::card::Snippet;
use crate::error::Error;
use crate::ghs::Ghs;
use crate::http::Http;
use crate::pubchem::{Compound, PubChem, Structure};
use crate::pug_view::{annotations, ghs_classification};
use crate::select::scrub;
use crate::wikipedia::{Wikipedia, wikipedia_snippets};

/// Every data source, sharing one HTTP client and its rate limit.
#[derive(Clone)]
pub struct Sources {
    pub pubchem: PubChem,
    pub wikipedia: Wikipedia,
}

impl Sources {
    pub fn new(http: Http) -> Self {
        Self {
            pubchem: PubChem::new(http.clone()),
            wikipedia: Wikipedia::new(http),
        }
    }
}

/// One part of a molecule, sent as soon as it is available.
#[derive(Debug, Clone, PartialEq)]
pub enum Progress {
    Compound(Compound),
    Structure(Structure),
    /// `None` when PubChem has no hazard classification.
    Hazards(Option<Ghs>),
    /// Snippets from one source, scrubbed; more may follow.
    Snippets(Vec<Snippet>),
}

/// Scrub dose and method sentences; drop snippets left empty.
fn prepare(snippets: &[Snippet]) -> Vec<Snippet> {
    snippets.iter().filter_map(scrub).collect()
}

/// Fetch a molecule from PubChem and Wikipedia. The header and structure come
/// first; then annotations, hazards and Wikipedia are fetched together. Any
/// failure fails the whole fetch, so a partial molecule is never cached.
pub async fn fetch(
    sources: &Sources,
    cid: u64,
    now: SystemTime,
    progress: &UnboundedSender<Progress>,
) -> Result<Molecule, Error> {
    // The receiver may have gone (the user moved on); keep fetching for the cache.
    let send = |p: Progress| {
        let _ = progress.send(p);
    };
    let pubchem = &sources.pubchem;

    let (compound, structure) = tokio::join!(
        async {
            let c = pubchem.compound(cid).await?;
            send(Progress::Compound(c.clone()));
            Ok::<_, Error>(c)
        },
        async {
            let s = pubchem.structure(cid).await?;
            send(Progress::Structure(s.clone()));
            Ok::<_, Error>(s)
        },
    );
    let (compound, structure) = (compound?, structure?);

    let (pubchem_snippets, ghs, wiki) = tokio::join!(
        async {
            let s = prepare(&annotations(pubchem, cid).await?);
            send(Progress::Snippets(s.clone()));
            Ok::<_, Error>(s)
        },
        async {
            let g = ghs_classification(pubchem, cid).await?;
            send(Progress::Hazards(g.as_ref().map(|(ghs, _)| ghs.clone())));
            Ok::<_, Error>(g)
        },
        async {
            let s = prepare(&wikipedia_snippets(pubchem, &sources.wikipedia, cid).await?);
            send(Progress::Snippets(s.clone()));
            Ok::<_, Error>(s)
        },
    );
    let (pubchem_snippets, ghs, wiki) = (pubchem_snippets?, ghs?, wiki?);

    // Card-input order, as in the spike: the record description, Wikipedia,
    // the other PubChem sections, then GHS. The same text can appear under
    // several headings; keep the first.
    let (description, other): (Vec<_>, Vec<_>) = pubchem_snippets
        .into_iter()
        .partition(|s| s.section == "Record Description");
    let (ghs, ghs_snippet) = ghs.unzip();
    let mut snippets: Vec<Snippet> = Vec::new();
    for s in description
        .into_iter()
        .chain(wiki)
        .chain(other)
        .chain(ghs_snippet)
    {
        if !snippets.iter().any(|kept| kept.text == s.text) {
            snippets.push(s);
        }
    }

    let (sdf_3d, png_2d) = match structure {
        Structure::ThreeD(sdf) => (Some(sdf), None),
        Structure::TwoD(png) => (None, Some(png)),
    };
    Ok(Molecule {
        compound,
        sdf_3d,
        png_2d,
        ghs,
        snippets,
        fetched_at: unix_seconds(now),
    })
}

/// The molecule from the cache while fresh, else fetched and cached. Cached
/// parts are sent as progress too, so the UI handles both paths the same way.
/// If a refetch fails, stale cached data is still better than nothing.
pub async fn load(
    sources: &Sources,
    cache: &Mutex<Cache>,
    cid: u64,
    now: SystemTime,
    progress: &UnboundedSender<Progress>,
) -> Result<Molecule, Error> {
    let cached = cache.lock().expect("cache lock").molecule(cid)?;
    let replay = |m: &Molecule| {
        for p in parts(m) {
            let _ = progress.send(p);
        }
    };
    if let Some(m) = cached.as_ref().filter(|m| m.is_fresh(now)) {
        replay(m);
        return Ok(m.clone());
    }
    match (fetch(sources, cid, now, progress).await, cached) {
        (Ok(m), _) => {
            let mut cache = cache.lock().expect("cache lock");
            cache.save(&m)?;
            // Reload so snippets carry their row ids.
            Ok(cache.molecule(cid)?.expect("just saved"))
        }
        (Err(_), Some(stale)) => {
            replay(&stale);
            Ok(stale)
        }
        (Err(e), None) => Err(e),
    }
}

/// A cached molecule as the progress messages a fetch would have sent.
fn parts(m: &Molecule) -> Vec<Progress> {
    let structure = match (&m.sdf_3d, &m.png_2d) {
        (Some(sdf), _) => Some(Structure::ThreeD(sdf.clone())),
        (None, Some(png)) => Some(Structure::TwoD(png.clone())),
        (None, None) => None,
    };
    let mut out = vec![Progress::Compound(m.compound.clone())];
    out.extend(structure.map(Progress::Structure));
    out.push(Progress::Hazards(m.ghs.clone()));
    out.push(Progress::Snippets(m.snippets.clone()));
    out
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

    use tokio::sync::mpsc::unbounded_channel;

    use super::*;
    use crate::http::Settings;
    use crate::http::test_server::{Log, response, serve};

    /// Molecules recorded by `scripts/record_http_fixtures.py`.
    const RECORDED: &[(&str, u64)] = &[
        ("caffeine", 2519),
        ("capsaicin", 1548943),
        ("menthol", 1254),
        ("buckminsterfullerene", 123591),
        ("ascorbic-acid", 54670067),
    ];

    fn slug(text: &str) -> String {
        text.to_lowercase().replace(' ', "-")
    }

    /// The recorded file for a request, named as the recorder names it; `None`
    /// means the real service answered 404. Wikipedia requests carry no CID, so
    /// their title is looked up in every molecule's folder.
    fn recorded(path: &str) -> Option<std::path::PathBuf> {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/http/molecules");
        let url = reqwest::Url::parse(&format!("http://local{path}")).ok()?;
        let query = |key: &str| {
            url.query_pairs()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.into_owned())
        };
        if let Some(title) = query("titles") {
            let file = format!("wikipedia-{}.json", slug(&title));
            return RECORDED
                .iter()
                .map(|(name, _)| root.join(name).join(&file))
                .find(|p| p.exists());
        }
        let segments: Vec<&str> = url.path_segments()?.collect();
        // `/pug/compound/cid/<cid>/…` or `/pug_view/data/compound/<cid>/JSON`.
        let cid_at = segments
            .iter()
            .position(|s| *s == "cid")
            .or_else(|| segments.iter().position(|s| *s == "compound"))?
            + 1;
        let cid: u64 = segments.get(cid_at)?.parse().ok()?;
        let (name, _) = RECORDED.iter().find(|(_, c)| *c == cid)?;
        let file = match segments.get(cid_at + 1..)? {
            ["property", ..] => "properties.json".to_string(),
            ["synonyms", ..] => "synonyms.json".to_string(),
            ["record", "SDF"] => "sdf-3d.sdf".to_string(),
            ["PNG"] => "png.png".to_string(),
            ["JSON"] => format!("view-{}.json", slug(&query("heading")?)),
            _ => return None,
        };
        Some(root.join(name).join(file)).filter(|p| p.exists())
    }

    /// Local PubChem and Wikipedia. While `down` is set, every request fails.
    async fn local_sources(down: Arc<AtomicBool>) -> (Sources, Log) {
        let (base, log) = serve(move |_, path| {
            if down.load(Ordering::Relaxed) {
                return response("500 Internal Server Error", "");
            }
            match recorded(path) {
                Some(file) => response("200 OK", std::fs::read(file).unwrap()),
                None => response("404 Not Found", ""),
            }
        })
        .await;
        let http = Http::with_settings(Settings {
            backoff: Duration::from_millis(1),
            pubchem_per_second: 1_000,
            ..Settings::default()
        });
        let sources = Sources {
            pubchem: PubChem::with_base(http.clone(), &format!("{base}/rest")),
            wikipedia: Wikipedia::with_api(http, &format!("{base}/w/api.php")),
        };
        (sources, log)
    }

    fn drain(rx: &mut tokio::sync::mpsc::UnboundedReceiver<Progress>) -> Vec<Progress> {
        std::iter::from_fn(|| rx.try_recv().ok()).collect()
    }

    #[tokio::test]
    async fn fetches_and_streams_every_part() {
        let (sources, _) = local_sources(Arc::default()).await;
        let (tx, mut rx) = unbounded_channel();
        let m = fetch(&sources, 2519, SystemTime::now(), &tx).await.unwrap();

        assert_eq!(m.compound.title, "Caffeine");
        assert!(m.sdf_3d.is_some() && m.png_2d.is_none());
        assert_eq!(m.ghs.as_ref().unwrap().signal.as_deref(), Some("Warning"));
        let sections: Vec<&str> = m.snippets.iter().map(|s| s.section.as_str()).collect();
        assert_eq!(sections.first(), Some(&"Record Description"));
        assert_eq!(sections.last(), Some(&"GHS Classification"));
        let first_wiki = sections.iter().position(|s| *s == "Introduction").unwrap();
        let first_uses = sections.iter().position(|s| *s == "Uses").unwrap();
        assert!(first_wiki < first_uses, "{sections:?}");
        // Scrubbed: caffeine's toxicity summary is full of doses.
        assert!(m.snippets.iter().all(|s| !s.text.contains("mg/kg")));
        let texts: std::collections::HashSet<_> = m.snippets.iter().map(|s| &s.text).collect();
        assert_eq!(texts.len(), m.snippets.len());

        let parts = drain(&mut rx);
        assert!(matches!(
            parts[0],
            Progress::Compound(_) | Progress::Structure(_)
        ));
        assert_eq!(
            parts
                .iter()
                .filter(|p| matches!(p, Progress::Snippets(_)))
                .count(),
            2
        );
        assert_eq!(parts.len(), 5);
    }

    /// What each recorded molecule is there to cover.
    #[tokio::test]
    async fn recorded_molecules() {
        let (sources, _) = local_sources(Arc::default()).await;
        let (tx, _rx) = unbounded_channel();
        let get = |cid| fetch(&sources, cid, SystemTime::now(), &tx);
        let has = |m: &Molecule, section: &str| m.snippets.iter().any(|s| s.section == section);
        let codes = |m: &Molecule| -> Vec<String> {
            m.ghs
                .iter()
                .flat_map(|g| &g.statements)
                .map(|s| s.code.clone())
                .collect()
        };

        for (name, cid) in RECORDED {
            let m = get(*cid).await.unwrap();
            assert!(
                m.snippets.len() >= 20,
                "{name}: {} snippets",
                m.snippets.len()
            );
            for section in [
                "Record Description",
                "Introduction",
                "History",
                "GHS Classification",
            ] {
                assert!(has(&m, section), "{name} lacks {section}");
            }
        }

        let capsaicin = get(1548943).await.unwrap();
        assert_eq!(
            capsaicin.ghs.as_ref().unwrap().signal.as_deref(),
            Some("Danger")
        );
        assert!(has(&capsaicin, "Natural function"), "the birds");

        let menthol = get(1254).await.unwrap();
        assert_eq!(codes(&menthol), ["H315", "H319"]);

        let c60 = get(123591).await.unwrap();
        assert!(c60.sdf_3d.is_none() && c60.png_2d.is_some());
        assert!(!has(&c60, "Toxicity Summary"), "PubChem has none");

        let vitamin_c = get(54670067).await.unwrap();
        let wiki = vitamin_c
            .snippets
            .iter()
            .find(|s| s.source == "wikipedia")
            .unwrap();
        assert_eq!(wiki.url, "https://en.wikipedia.org/wiki/Vitamin_C");
        assert!(codes(&vitamin_c).is_empty(), "not classified");
    }

    #[tokio::test]
    async fn serves_fresh_data_from_the_cache_and_stale_data_when_offline() {
        let down = Arc::new(AtomicBool::new(false));
        let (sources, log) = local_sources(down.clone()).await;
        let cache = Mutex::new(Cache::in_memory().unwrap());
        let (tx, mut rx) = unbounded_channel();
        let t0 = SystemTime::now();

        let first = load(&sources, &cache, 2519, t0, &tx).await.unwrap();
        let requests = log.lock().unwrap().len();
        assert!(requests >= 10);
        assert!(first.snippets.iter().all(|s| s.id.parse::<i64>().is_ok()));
        drain(&mut rx);

        let again = load(&sources, &cache, 2519, t0 + Duration::from_secs(60), &tx)
            .await
            .unwrap();
        assert_eq!(again, first);
        assert_eq!(log.lock().unwrap().len(), requests, "served from the cache");
        assert_eq!(drain(&mut rx).len(), 4, "cached parts are replayed");

        down.store(true, Ordering::Relaxed);
        let later = t0 + crate::cache::FRESH_FOR + Duration::from_secs(1);
        let stale = load(&sources, &cache, 2519, later, &tx).await.unwrap();
        assert_eq!(stale, first, "stale data when the refetch fails");
        assert!(log.lock().unwrap().len() > requests, "a refetch was tried");

        let missing = load(&sources, &cache, 1, later, &tx).await;
        assert!(missing.is_err());
    }
}

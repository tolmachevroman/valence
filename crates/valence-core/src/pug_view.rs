//! PubChem PUG-View annotations: the text sections cards are written from.
//! Manufacturing sections are never fetched, so they never reach a card.

use serde_json::Value;

use crate::card::Snippet;
use crate::pubchem::{Error, PubChem};
use crate::text::{MIN_CHARS, clean};

/// Headings in the order they are listed, and how many entries to keep from
/// each. The same choice as the M0 spike fixtures, which the cards were
/// measured on. GHS Classification is fetched separately (`ghs`).
pub const SECTIONS: [(&str, usize); 4] = [
    ("Record Description", 5),
    ("Uses", 4),
    ("Natural Pollution Sources", 3),
    ("Toxicity Summary", 2),
];

/// One text entry under a heading.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// The entry's label, like `Signal` or `See Also`; often absent.
    pub name: Option<String>,
    pub text: String,
    /// The data provider, like `HSDB` or `ChEBI`.
    pub origin: String,
}

/// Every text entry under a PUG-View reply, depth first, as PubChem lists them.
pub fn entries(view: &Value) -> Vec<Entry> {
    let record = &view["Record"];
    let origin = |info: &Value| -> String {
        let number = info.get("ReferenceNumber");
        record["Reference"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|r| r.get("ReferenceNumber") == number)
            .and_then(|r| r["SourceName"].as_str())
            .unwrap_or_default()
            .to_string()
    };
    let mut out = Vec::new();
    let mut stack: Vec<&Value> = record["Section"]
        .as_array()
        .into_iter()
        .flatten()
        .rev()
        .collect();
    while let Some(section) = stack.pop() {
        for info in section["Information"].as_array().into_iter().flatten() {
            let text: Vec<&str> = info["Value"]["StringWithMarkup"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|p| p["String"].as_str().unwrap_or_default())
                .collect();
            out.push(Entry {
                name: info["Name"].as_str().map(str::to_string),
                text: text.join(" "),
                origin: origin(info),
            });
        }
        stack.extend(section["Section"].as_array().into_iter().flatten().rev());
    }
    out
}

/// The first `limit` usable entries under `heading` as snippets. Ids are left
/// empty; they are numbered when a molecule's snippets are put together.
pub fn snippets(cid: u64, heading: &str, limit: usize, view: &Value) -> Vec<Snippet> {
    let url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/compound/{cid}#section={}",
        heading.replace(' ', "-")
    );
    let mut out: Vec<Snippet> = Vec::new();
    for entry in entries(view) {
        if entry.name.as_deref() == Some("See Also") {
            continue;
        }
        let text = clean(&entry.text);
        if text.chars().count() < MIN_CHARS || out.iter().any(|s| s.text == text) {
            continue;
        }
        out.push(Snippet {
            id: String::new(),
            source: "pubchem_view".into(),
            section: heading.into(),
            url: url.clone(),
            origin: entry.origin,
            text,
        });
        if out.len() == limit {
            break;
        }
    }
    out
}

/// Snippets from every heading in `SECTIONS` that the compound has.
pub async fn annotations(pubchem: &PubChem, cid: u64) -> Result<Vec<Snippet>, Error> {
    let mut out = Vec::new();
    for (heading, limit) in SECTIONS {
        if let Some(view) = pubchem.view(cid, heading).await? {
            out.extend(snippets(cid, heading, limit, &view));
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn view(name: &str) -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/http/pubchem")
            .join(name);
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }

    #[test]
    fn walks_nested_sections_with_their_sources() {
        let uses = entries(&view("view-2519-uses.json"));
        assert_eq!(
            uses[0].name.as_deref(),
            Some("Cosmetic Ingredient Review Link")
        );
        assert!(
            uses.iter()
                .any(|e| e.text.starts_with("Natural products -> Stimulants"))
        );
        assert!(
            uses.iter()
                .any(|e| e.origin.starts_with("California Safe Cosmetics"))
        );
    }

    #[test]
    fn keeps_the_first_usable_entries() {
        let s = snippets(
            2519,
            "Record Description",
            5,
            &view("view-2519-record-description.json"),
        );
        assert_eq!(s.len(), 5);
        assert!(s.iter().all(|s| s.text.chars().count() >= MIN_CHARS));
        assert_eq!(
            s[0].url,
            "https://pubchem.ncbi.nlm.nih.gov/compound/2519#section=Record-Description"
        );
        assert_eq!(s[0].source, "pubchem_view");
        let toxicity = snippets(
            2519,
            "Toxicity Summary",
            2,
            &view("view-2519-toxicity-summary.json"),
        );
        assert_eq!(toxicity.len(), 2);
    }

    /// Same input, same snippets as the Python script that made the spike
    /// fixtures, so the app's cards match what M0 measured.
    #[test]
    fn matches_the_spike_fixture_for_caffeine() {
        let fixture: Value = serde_json::from_slice(
            &std::fs::read(
                Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/spike/caffeine.json"),
            )
            .unwrap(),
        )
        .unwrap();
        for (heading, limit) in SECTIONS {
            let file = format!(
                "view-2519-{}.json",
                heading.to_lowercase().replace(' ', "-")
            );
            let ours: Vec<String> = snippets(2519, heading, limit, &view(&file))
                .into_iter()
                .map(|s| s.text)
                .collect();
            let spike: Vec<&str> = fixture["snippets"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|s| s["source"] == "pubchem_view" && s["section"] == heading)
                .map(|s| s["text"].as_str().unwrap())
                .collect();
            // The spike also dropped texts repeated across headings.
            assert!(
                spike.iter().all(|t| ours.iter().any(|o| o == t)),
                "{heading}: {spike:#?}\nvs\n{ours:#?}"
            );
        }
    }
}

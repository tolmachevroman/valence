//! The "Surprise me" list (M4.1).
//!
//! `valence curate` loads every name in `fixtures/curated/candidates.txt`
//! through the cache and keeps the molecules with an origin story and input
//! for at least `MIN_KINDS` cards, written to `app/src/curated.json` with
//! their CIDs so a pick needs no name lookup. Run again, it is served from
//! the cache.

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::SystemTime;

use serde::Serialize;
use tokio::sync::mpsc::unbounded_channel;
use valence_core::cache::Cache;
use valence_core::card::CardKind;
use valence_core::cards::has_input;
use valence_core::fetch::{Sources, load};
use valence_core::http::Http;

use crate::Result;

/// Card kinds with input, Danger included; the PRD's target is 3+ cards.
const MIN_KINDS: usize = 3;

#[derive(Serialize)]
struct Entry {
    cid: u64,
    name: String,
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

pub async fn run(cache_path: &Path) -> Result<()> {
    if let Some(dir) = cache_path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let list = std::fs::read_to_string(root().join("fixtures/curated/candidates.txt"))?;
    let names = list
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'));
    let cache = Mutex::new(Cache::open(cache_path)?);
    let sources = Sources::new(Http::new());
    let (tx, _rx) = unbounded_channel();

    let mut kept: Vec<Entry> = Vec::new();
    let mut rejected = 0;
    for name in names {
        let cid = match sources.pubchem.resolve(name).await {
            Ok(Some(cid)) => cid,
            Ok(None) => {
                eprintln!("skip  {name}: not on PubChem");
                rejected += 1;
                continue;
            }
            Err(e) => {
                eprintln!("skip  {name}: {e}");
                rejected += 1;
                continue;
            }
        };
        if kept.iter().any(|e| e.cid == cid) {
            eprintln!("skip  {name}: same compound as another name");
            continue;
        }
        let m = match load(&sources, &cache, cid, SystemTime::now(), &tx).await {
            Ok(m) => m,
            Err(e) => {
                eprintln!("skip  {name}: {e}");
                rejected += 1;
                continue;
            }
        };
        let c = &m.compound;
        let kinds: Vec<CardKind> = CardKind::ALL
            .into_iter()
            .filter(|&k| has_input(k, &c.title, &c.synonyms, &m.snippets))
            .collect();
        let listed = kinds
            .iter()
            .map(|k| k.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        if kinds.contains(&CardKind::Origin) && kinds.len() >= MIN_KINDS {
            eprintln!("keep  {name} → {} ({cid}): {listed}", c.title);
            kept.push(Entry {
                cid,
                name: c.title.clone(),
            });
        } else {
            eprintln!("skip  {name} ({cid}): only {listed}");
            rejected += 1;
        }
    }

    let out = root().join("app/src/curated.json");
    let mut json = serde_json::to_string_pretty(&kept)?;
    json.push('\n');
    std::fs::write(&out, json)?;
    eprintln!(
        "{} kept, {rejected} skipped; wrote {}",
        kept.len(),
        out.display()
    );
    Ok(())
}

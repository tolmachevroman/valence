//! Tauri shell: exposes `valence-core` to the frontend as commands and events.
//! The frontend never talks to PubChem, Wikipedia or the model directly.
//!
//! Types sent to the frontend are mirrored in `app/src/core.ts`.

use std::sync::Mutex;
use std::time::SystemTime;

use serde::Serialize;
use tauri::Manager;
use tauri::ipc::Channel;
use valence_core::cache::Cache;
use valence_core::fetch::{Progress, Sources, load};
use valence_core::ghs::Ghs;
use valence_core::http::Http;
use valence_core::pubchem::{Compound, Structure};

struct AppState {
    sources: Sources,
    cache: Mutex<Cache>,
}

/// One part of a molecule, as it arrives.
#[derive(Clone, Serialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum MoleculeEvent {
    Header {
        compound: Compound,
    },
    /// SDF text for the 3D view, or a PNG data URL when there is no 3D conformer.
    Structure {
        sdf: Option<String>,
        png: Option<String>,
    },
    Hazards {
        ghs: Option<Ghs>,
    },
    /// Everything has arrived and is cached.
    Done {
        snippet_count: usize,
        /// Unix seconds.
        fetched_at: u64,
    },
}

impl MoleculeEvent {
    /// Snippets go to the card pipeline, not the UI, so they map to nothing.
    fn from_progress(p: Progress) -> Option<Self> {
        Some(match p {
            Progress::Compound(compound) => Self::Header { compound },
            Progress::Structure(Structure::ThreeD(sdf)) => Self::Structure {
                sdf: Some(sdf),
                png: None,
            },
            Progress::Structure(Structure::TwoD(png)) => Self::Structure {
                sdf: None,
                png: Some(format!("data:image/png;base64,{}", base64(&png))),
            },
            Progress::Hazards(ghs) => Self::Hazards { ghs },
            Progress::Snippets(_) => return None,
        })
    }
}

fn base64(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = chunk
            .iter()
            .enumerate()
            .fold(0u32, |n, (i, b)| n | u32::from(*b) << (16 - 8 * i));
        for i in 0..4 {
            if i <= chunk.len() {
                out.push(ALPHABET[(n >> (18 - 6 * i) & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
    }
    out
}

#[tauri::command]
fn app_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Compound names starting like `query`.
#[tauri::command]
async fn autocomplete(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<Vec<String>, String> {
    state
        .sources
        .pubchem
        .autocomplete(&query, 10)
        .await
        .map_err(|e| e.to_string())
}

/// Find a molecule by name, CAS number or CID and stream its parts to
/// `on_event`, from the cache when fresh. Returns its CID.
#[tauri::command]
async fn open_molecule(
    state: tauri::State<'_, AppState>,
    query: String,
    on_event: Channel<MoleculeEvent>,
) -> Result<u64, String> {
    let cid = state
        .sources
        .pubchem
        .resolve(&query)
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("PubChem has no compound called \"{}\"", query.trim()))?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let (molecule, ()) = tokio::join!(
        async {
            let m = load(&state.sources, &state.cache, cid, SystemTime::now(), &tx).await;
            drop(tx);
            m
        },
        async {
            while let Some(p) = rx.recv().await {
                if let Some(event) = MoleculeEvent::from_progress(p) {
                    // The window may have moved on to another molecule.
                    let _ = on_event.send(event);
                }
            }
        },
    );
    let molecule = molecule.map_err(|e| e.to_string())?;
    let _ = on_event.send(MoleculeEvent::Done {
        snippet_count: molecule.snippets.len(),
        fetched_at: molecule.fetched_at,
    });
    Ok(cid)
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let cache = Cache::open(&dir.join("cache.sqlite"))?;
            app.manage(AppState {
                sources: Sources::new(Http::new()),
                cache: Mutex::new(cache),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_version,
            autocomplete,
            open_molecule
        ])
        .run(tauri::generate_context!())
        .expect("error while running Valence");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_matches_rfc_4648() {
        for (input, expected) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(base64(input.as_bytes()), expected);
        }
    }

    #[test]
    fn events_are_tagged_camel_case() {
        let done = MoleculeEvent::Done {
            snippet_count: 3,
            fetched_at: 7,
        };
        assert_eq!(
            serde_json::to_string(&done).unwrap(),
            r#"{"kind":"done","snippetCount":3,"fetchedAt":7}"#
        );
    }
}

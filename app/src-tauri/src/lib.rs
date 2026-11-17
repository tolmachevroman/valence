//! Tauri shell: exposes `valence-core` to the frontend as commands and events.
//! The frontend never talks to PubChem, Wikipedia or the model directly.
//!
//! Types sent to the frontend are mirrored in `app/src/core.ts`.

use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};
use std::time::{Instant, SystemTime};

use serde::Serialize;
use tauri::Manager;
use tauri::async_runtime::JoinHandle;
use tauri::ipc::Channel;
use tokio::sync::watch;
use valence_core::cache::{Cache, Molecule, unix_seconds};
use valence_core::card::CardKind;
use valence_core::cards::{self, Cached, Card, Event, Hazards, Outcome};
use valence_core::download::{self, DEFAULT_MODEL, part_path};
use valence_core::fetch::{Progress, Sources, load};
use valence_core::ghs::Ghs;
use valence_core::ghs::HazardStatement;
use valence_core::http::Http;
use valence_core::model::{LoadError, LocalModel, MODEL_FILE};
use valence_core::pubchem::{Compound, Structure};

struct AppState {
    sources: Sources,
    cache: Mutex<Cache>,
    model: Model,
    /// Writes the open molecule's cards, one at a time: the only user of the
    /// model, so this is the generation queue. Opening another molecule aborts it.
    writing: Mutex<Option<JoinHandle<()>>>,
}

/// The local model, loaded in the background at launch.
struct Model {
    /// The model's file name; cards are cached per model.
    name: String,
    path: PathBuf,
    status: watch::Sender<ModelStatus>,
    loaded: OnceLock<LocalModel>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(
    tag = "state",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum ModelStatus {
    /// No model file yet: the welcome screen offers the download.
    NotDownloaded {
        /// Bytes of the whole file.
        size: u64,
        /// Bytes kept from an earlier attempt, resumed by the next.
        downloaded: u64,
        /// Why the last attempt stopped.
        error: Option<String>,
    },
    Downloading {
        done: u64,
        total: u64,
    },
    Loading,
    Ready {
        load_seconds: f64,
    },
    /// mistral.rs refused: too little free memory for the model and its cache.
    NotEnoughMemory,
    Failed {
        message: String,
    },
}

impl ModelStatus {
    /// Whether the model may still become ready without the user's help.
    fn is_coming(&self) -> bool {
        matches!(self, Self::Loading | Self::Downloading { .. })
    }

    fn not_downloaded(path: &Path, error: Option<String>) -> Self {
        Self::NotDownloaded {
            size: DEFAULT_MODEL.size,
            downloaded: std::fs::metadata(part_path(path)).map_or(0, |m| m.len()),
            error,
        }
    }
}

/// `VALENCE_MODEL` when set (development), else the app's models folder.
fn model_path(data_dir: &Path) -> PathBuf {
    std::env::var_os("VALENCE_MODEL")
        .map(PathBuf::from)
        .unwrap_or_else(|| data_dir.join("models").join(MODEL_FILE))
}

async fn load_model(state: &AppState, path: &Path) {
    let started = Instant::now();
    let status = match LocalModel::load(path).await {
        Ok(model) => {
            let _ = state.model.loaded.set(model);
            ModelStatus::Ready {
                load_seconds: started.elapsed().as_secs_f64(),
            }
        }
        Err(LoadError::Missing) => ModelStatus::not_downloaded(path, None),
        Err(LoadError::NotEnoughMemory(e)) => {
            eprintln!("model refused to load: {e}");
            ModelStatus::NotEnoughMemory
        }
        Err(LoadError::Failed(message)) => ModelStatus::Failed { message },
    };
    state.model.status.send_replace(status);
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
    /// Cards to expect, in the order they will be written.
    CardsPlanned {
        cards: Vec<CardKind>,
    },
    CardWriting {
        card: CardKind,
    },
    /// The card so far: the source sentences chosen.
    CardDraft {
        card: CardKind,
        body: String,
    },
    CardRetrying {
        card: CardKind,
    },
    CardReady {
        card: CardView,
    },
    /// No card of this kind: skipped, failed its checks, or no model.
    CardGone {
        card: CardKind,
    },
}

/// A card that passed every check.
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct CardView {
    kind: CardKind,
    /// "Where you meet it".
    label: &'static str,
    title: String,
    body: String,
    sources: Vec<SourceView>,
    /// Danger only: official statements to list verbatim.
    hazards: Option<HazardsView>,
}

#[derive(Clone, Serialize)]
struct HazardsView {
    pictograms: Vec<String>,
    statements: Vec<HazardStatement>,
}

#[derive(Clone, Serialize)]
struct SourceView {
    label: String,
    url: String,
}

impl From<Card> for CardView {
    /// Two paragraphs of one section share a link; it is shown once.
    fn from(c: Card) -> Self {
        let mut sources: Vec<SourceView> = Vec::new();
        for s in c.sources {
            if !sources.iter().any(|v| v.url == s.url) {
                sources.push(SourceView {
                    label: s.label,
                    url: s.url,
                });
            }
        }
        Self {
            kind: c.kind,
            label: c.kind.label(),
            title: c.title,
            body: c.body,
            sources,
            hazards: c.hazards.map(
                |Hazards {
                     pictograms,
                     statements,
                 }| HazardsView {
                    pictograms,
                    statements,
                },
            ),
        }
    }
}

impl MoleculeEvent {
    fn from_card_event(card: CardKind, e: Event) -> Self {
        match e {
            Event::Writing => Self::CardWriting { card },
            Event::Draft { body } => Self::CardDraft { card, body },
            Event::Retrying { .. } => Self::CardRetrying { card },
        }
    }
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

/// Show the molecule's cards: the Danger card and cached cards at once, then
/// the missing ones as the model writes them, in order, each stored as it
/// finishes. Model cards wait for the model to load and are dropped when it can't.
async fn write_cards(state: &AppState, m: &Molecule, on_event: &Channel<MoleculeEvent>) {
    let name = &m.compound.title;
    let cid = m.compound.cid;
    let model_name = &state.model.name;
    let send = |e| {
        let _ = on_event.send(e);
    };
    let order = cards::order(&m.snippets);
    send(MoleculeEvent::CardsPlanned {
        cards: order.clone(),
    });

    let mut missing = Vec::new();
    for &kind in &order {
        let cached = if kind == CardKind::Danger {
            Ok(cards::danger(name, &m.snippets).map_or(Cached::NoCard, Cached::Card))
        } else {
            let cache = state.cache.lock().expect("not poisoned");
            cards::cached(&cache, cid, kind, &m.snippets, model_name)
        };
        match cached {
            Ok(Cached::Card(card)) => send(MoleculeEvent::CardReady { card: card.into() }),
            Ok(Cached::NoCard) => send(MoleculeEvent::CardGone { card: kind }),
            Ok(Cached::Missing) => missing.push(kind),
            Err(e) => {
                eprintln!("{name}: reading cached {} card: {e}", kind.as_str());
                missing.push(kind);
            }
        }
    }

    let mut status = state.model.status.subscribe();
    for kind in missing {
        let _ = status.wait_for(|s| !s.is_coming()).await;
        let Some(model) = state.model.loaded.get() else {
            send(MoleculeEvent::CardGone { card: kind });
            continue;
        };
        let mut on_card_event = |e| send(MoleculeEvent::from_card_event(kind, e));
        let outcome = cards::card(
            model,
            kind,
            name,
            &m.compound.synonyms,
            &m.snippets,
            &mut on_card_event,
        )
        .await;
        let outcome = match outcome {
            Ok(outcome) => outcome,
            Err(e) => {
                eprintln!("{name}: {} card failed: {e}", kind.as_str());
                send(MoleculeEvent::CardGone { card: kind });
                continue;
            }
        };
        if let Outcome::Dropped(problems) = &outcome {
            eprintln!(
                "{name}: {} card dropped: {}",
                kind.as_str(),
                problems.join("; ")
            );
        }
        let now = unix_seconds(SystemTime::now());
        let mut cache = state.cache.lock().expect("not poisoned");
        if let Err(e) = cards::remember(
            &mut cache,
            cid,
            kind,
            &m.snippets,
            model_name,
            &outcome,
            now,
        ) {
            eprintln!("{name}: storing the {} card: {e}", kind.as_str());
        }
        drop(cache);
        match outcome {
            Outcome::Card(card) => send(MoleculeEvent::CardReady { card: card.into() }),
            _ => send(MoleculeEvent::CardGone { card: kind }),
        }
    }
}

/// Find a molecule by name, CAS number or CID and stream its parts to
/// `on_event`, from the cache when fresh, then its cards as they are written.
/// Returns its CID once the data is in; cards keep arriving after that.
#[tauri::command]
async fn open_molecule(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    query: String,
    on_event: Channel<MoleculeEvent>,
) -> Result<u64, String> {
    // The window has moved on: stop writing the previous molecule's cards.
    if let Some(previous) = state.writing.lock().expect("not poisoned").take() {
        previous.abort();
    }
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
    let task = tauri::async_runtime::spawn(async move {
        write_cards(&app.state::<AppState>(), &molecule, &on_event).await;
    });
    if let Some(previous) = state.writing.lock().expect("not poisoned").replace(task) {
        previous.abort();
    }
    Ok(cid)
}

/// Open a source link in the default browser. Only https links: card sources
/// are PubChem and Wikipedia pages.
#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err(format!("not an https link: {url}"));
    }
    std::process::Command::new("/usr/bin/open")
        .arg(&url)
        .status()
        .map_err(|e| e.to_string())
        .and_then(|s| {
            s.success()
                .then_some(())
                .ok_or_else(|| format!("open failed for {url}"))
        })
}

/// Send the model status to `on_change` now and on every change, until the
/// model is ready or the window is gone.
#[tauri::command]
async fn model_status(
    state: tauri::State<'_, AppState>,
    on_change: Channel<ModelStatus>,
) -> Result<(), String> {
    let mut rx = state.model.status.subscribe();
    loop {
        let status = rx.borrow_and_update().clone();
        let ready = matches!(status, ModelStatus::Ready { .. });
        if on_change.send(status).is_err() || ready || rx.changed().await.is_err() {
            return Ok(());
        }
    }
}

/// Download the default model, resuming what an earlier attempt kept, then
/// load it. Does nothing unless the model is missing.
#[tauri::command]
fn download_model(app: tauri::AppHandle, state: tauri::State<'_, AppState>) {
    let start = ModelStatus::Downloading {
        done: 0,
        total: DEFAULT_MODEL.size,
    };
    let started = state.model.status.send_if_modified(|s| {
        let missing = matches!(s, ModelStatus::NotDownloaded { .. });
        if missing {
            *s = start;
        }
        missing
    });
    if !started {
        return;
    }
    let path = state.model.path.clone();
    tauri::async_runtime::spawn(async move {
        let state = app.state::<AppState>();
        let status = &state.model.status;
        let result = download::download(
            &download::client(),
            DEFAULT_MODEL,
            &path,
            &mut |done, total| {
                status.send_replace(ModelStatus::Downloading { done, total });
            },
        )
        .await;
        match result {
            Ok(()) => {
                status.send_replace(ModelStatus::Loading);
                load_model(&state, &path).await;
            }
            Err(e) => {
                eprintln!("model download stopped: {e}");
                status.send_replace(ModelStatus::not_downloaded(&path, Some(e.to_string())));
            }
        }
    });
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&dir)?;
            let cache = Cache::open(&dir.join("cache.sqlite"))?;
            let path = model_path(&dir);
            app.manage(AppState {
                sources: Sources::new(Http::new()),
                cache: Mutex::new(cache),
                model: Model {
                    name: path
                        .file_name()
                        .map_or_else(|| MODEL_FILE.into(), |f| f.to_string_lossy().into_owned()),
                    path: path.clone(),
                    status: watch::Sender::new(ModelStatus::Loading),
                    loaded: OnceLock::new(),
                },
                writing: Mutex::new(None),
            });
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                load_model(&handle.state::<AppState>(), &path).await;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            app_version,
            autocomplete,
            open_molecule,
            model_status,
            download_model,
            open_url
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
        let missing = ModelStatus::NotDownloaded {
            size: 9,
            downloaded: 4,
            error: None,
        };
        assert_eq!(
            serde_json::to_string(&missing).unwrap(),
            r#"{"state":"notDownloaded","size":9,"downloaded":4,"error":null}"#
        );
        let ready = ModelStatus::Ready { load_seconds: 1.5 };
        assert_eq!(
            serde_json::to_string(&ready).unwrap(),
            r#"{"state":"ready","loadSeconds":1.5}"#
        );
    }
}

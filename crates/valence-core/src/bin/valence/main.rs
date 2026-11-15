//! `valence snippets <name | CAS | CID>`: print a molecule's header and every
//! snippet a card could use, fetching it once and then serving it from the cache.
//!
//! `valence cards <name | CAS | CID>` (feature `model`): write its cards with
//! the local model, as the app does, with timings.
//!
//! `valence review fetch | run`: the 50-molecule review set (see `review`).

mod review;

use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;
use std::time::{Instant, SystemTime};

use tokio::sync::mpsc::unbounded_channel;
use valence_core::cache::{Cache, Molecule};
use valence_core::fetch::{Progress, Sources, load};
use valence_core::http::Http;
use valence_core::pubchem::Structure;

const USAGE: &str = "usage: valence snippets <name | CAS number | CID> [--cache <file>]
       valence cards <name | CAS number | CID> [--cache <file>] [--model <file.gguf>]
       valence review fetch
       valence review run [--model <file.gguf>]

The cache defaults to the app's: ~/Library/Application Support/io.github.tolmachevroman.valence/cache.sqlite
The model defaults to $VALENCE_MODEL, then the app's models folder.";

const APP_ID: &str = "io.github.tolmachevroman.valence";

fn default_cache() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(
        PathBuf::from(home)
            .join("Library/Application Support")
            .join(APP_ID)
            .join("cache.sqlite"),
    )
}

#[cfg(feature = "model")]
fn app_dir() -> Option<PathBuf> {
    default_cache()?.parent().map(PathBuf::from)
}

struct Args {
    command: String,
    query: String,
    cache: Option<PathBuf>,
    #[cfg_attr(not(feature = "model"), allow(dead_code))]
    model: Option<PathBuf>,
}

fn parse_args() -> Option<Args> {
    let mut args = std::env::args().skip(1);
    let command = args
        .next()
        .filter(|c| ["snippets", "cards", "review"].contains(&c.as_str()))?;
    // For `review`, the subcommand.
    let query = args
        .next()
        .filter(|q| command != "review" || q == "fetch" || q == "run")?;
    let (mut cache, mut model) = (None, None);
    while let Some(flag) = args.next() {
        let value = PathBuf::from(args.next()?);
        match flag.as_str() {
            "--cache" => cache = Some(value),
            "--model" if command != "snippets" => model = Some(value),
            _ => return None,
        }
    }
    Some(Args {
        command,
        query,
        cache,
        model,
    })
}

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> ExitCode {
    let Some(args) = parse_args() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let Some(cache_path) = args.cache.clone().or_else(default_cache) else {
        eprintln!("HOME is not set; pass --cache <file>");
        return ExitCode::from(2);
    };
    let result = match args.command.as_str() {
        "snippets" => snippets(&args.query, cache_path).await,
        "review" if args.query == "fetch" => review::fetch_all().await,
        "review" => review_run(&args).await,
        _ => cards(&args, cache_path).await,
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

/// Resolve and load a molecule, logging each part as it arrives.
async fn open(query: &str, cache_path: &Path, start: Instant) -> Result<Molecule> {
    if let Some(dir) = cache_path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let cache = Mutex::new(Cache::open(cache_path)?);
    let sources = Sources::new(Http::new());

    let Some(cid) = sources.pubchem.resolve(query).await? else {
        return Err(format!("PubChem has no compound named \"{query}\"").into());
    };
    eprintln!("{:6.2} s  CID {cid}", start.elapsed().as_secs_f64());

    let (tx, mut rx) = unbounded_channel();
    let progress = tokio::spawn(async move {
        while let Some(p) = rx.recv().await {
            let what = match p {
                Progress::Compound(c) => format!("header: {}", c.title),
                Progress::Structure(Structure::ThreeD(_)) => "structure: 3D".into(),
                Progress::Structure(Structure::TwoD(_)) => "structure: 2D drawing".into(),
                Progress::Hazards(g) => format!(
                    "hazards: {}",
                    g.and_then(|g| g.signal).unwrap_or_else(|| "none".into())
                ),
                Progress::Snippets(s) => format!("{} snippets", s.len()),
            };
            eprintln!("{:6.2} s  {what}", start.elapsed().as_secs_f64());
        }
    });
    let m = load(&sources, &cache, cid, SystemTime::now(), &tx).await?;
    drop(tx);
    progress.await?;
    Ok(m)
}

async fn snippets(query: &str, cache_path: PathBuf) -> Result<()> {
    let start = Instant::now();
    let m = open(query, &cache_path, start).await?;

    let c = &m.compound;
    println!("{} (CID {})", c.title, c.cid);
    println!("Formula     {}", c.formula);
    println!("Molar mass  {} g/mol", c.molar_mass);
    if let Some(cas) = &c.cas {
        println!("CAS         {cas}");
    }
    if !c.synonyms.is_empty() {
        println!("Also called {}", c.synonyms.join(", "));
    }
    println!(
        "Structure   {}",
        if m.sdf_3d.is_some() {
            "3D conformer"
        } else {
            "2D drawing only"
        }
    );
    match &m.ghs {
        Some(g) if !g.statements.is_empty() => {
            println!(
                "Hazards     {}",
                g.signal.as_deref().unwrap_or("no signal word")
            );
            for s in &g.statements {
                let share = s.percent.map(|p| format!(" ({p}%)")).unwrap_or_default();
                println!("            {}{share}: {}", s.code, s.text);
            }
        }
        Some(_) => println!("Hazards     not classified"),
        None => println!("Hazards     no GHS data"),
    }
    println!();
    for (i, s) in m.snippets.iter().enumerate() {
        let source = if s.source == "wikipedia" {
            "Wikipedia"
        } else {
            "PubChem"
        };
        let origin = if s.origin.is_empty() || s.origin == source {
            String::new()
        } else {
            format!(" via {}", s.origin)
        };
        println!(
            "[{}] {source}, {}{origin}\n    {}\n    {}\n",
            i + 1,
            s.section,
            s.url,
            s.text
        );
    }
    eprintln!(
        "{} snippets, {:.2} s, cache {}",
        m.snippets.len(),
        start.elapsed().as_secs_f64(),
        cache_path.display()
    );
    Ok(())
}

#[cfg(not(feature = "model"))]
async fn cards(_: &Args, _: PathBuf) -> Result<()> {
    Err("built without the model; run with --features model".into())
}

#[cfg(not(feature = "model"))]
async fn review_run(_: &Args) -> Result<()> {
    Err("built without the model; run with --features model".into())
}

#[cfg(feature = "model")]
async fn review_run(args: &Args) -> Result<()> {
    review::run(&model_path(args)?).await
}

/// `--model`, else `VALENCE_MODEL`, else the app's models folder.
#[cfg(feature = "model")]
fn model_path(args: &Args) -> Result<PathBuf> {
    use valence_core::model::MODEL_FILE;
    args.model
        .clone()
        .or_else(|| std::env::var_os("VALENCE_MODEL").map(PathBuf::from))
        .or_else(|| Some(app_dir()?.join("models").join(MODEL_FILE)))
        .ok_or_else(|| "HOME is not set; pass --model <file>".into())
}

#[cfg(feature = "model")]
async fn cards(args: &Args, cache_path: PathBuf) -> Result<()> {
    use valence_core::card::CardKind;
    use valence_core::cards::{self, Event, Outcome};
    use valence_core::model::LocalModel;

    let model_path = model_path(args)?;
    let start = Instant::now();
    // One after the other: run concurrently in one task, the model load
    // starved the fetch and both hung.
    let m = open(&args.query, &cache_path, start).await?;
    let model = LocalModel::load(&model_path).await?;
    let t = || start.elapsed().as_secs_f64();
    eprintln!("{:6.2} s  model loaded", t());

    let name = &m.compound.title;
    for kind in cards::order(&m.snippets) {
        let began = t();
        let mut first_text = None;
        let outcome = if kind == CardKind::Danger {
            cards::danger(name, &m.snippets).map_or(Outcome::NoInput, Outcome::Card)
        } else {
            cards::card(
                &model,
                kind,
                name,
                &m.compound.synonyms,
                &m.snippets,
                &mut |e| match e {
                    Event::Draft { .. } => {
                        first_text.get_or_insert_with(t);
                    }
                    Event::Retrying { problems } => eprintln!(
                        "{:6.2} s  {} retrying: {}",
                        t(),
                        kind.as_str(),
                        problems.join("; ")
                    ),
                    _ => {}
                },
            )
            .await?
        };
        let first = first_text.map_or(String::new(), |f| format!(", first text at {f:.2} s"));
        let took = format!("{:.1} s{first}", t() - began);
        match outcome {
            Outcome::Card(card) => {
                eprintln!("{:6.2} s  {} ready ({took})", t(), kind.as_str());
                println!("## {}: {}\n{}", kind.label(), card.title, card.body);
                for s in card.sources {
                    println!("  - {} <{}>", s.label, s.url);
                }
                println!();
            }
            Outcome::Dropped(problems) => {
                eprintln!(
                    "{:6.2} s  {} dropped ({took}): {}",
                    t(),
                    kind.as_str(),
                    problems.join("; ")
                );
            }
            Outcome::Skipped => eprintln!(
                "{:6.2} s  {} skipped by the model ({took})",
                t(),
                kind.as_str()
            ),
            Outcome::NoInput => eprintln!("{:6.2} s  {} no input", t(), kind.as_str()),
        }
    }
    Ok(())
}

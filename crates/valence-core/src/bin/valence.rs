//! `valence snippets <name | CAS | CID>`: print a molecule's header and every
//! snippet a card could use, fetching it once and then serving it from the cache.

use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::Mutex;
use std::time::{Instant, SystemTime};

use tokio::sync::mpsc::unbounded_channel;
use valence_core::cache::Cache;
use valence_core::fetch::{Progress, Sources, load};
use valence_core::http::Http;
use valence_core::pubchem::Structure;

const USAGE: &str = "usage: valence snippets <name | CAS number | CID> [--cache <file>]

The cache defaults to the app's: ~/Library/Application Support/io.github.tolmachevroman.valence/cache.sqlite";

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

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (query, cache_path) = match args.as_slice() {
        [cmd, query] if cmd == "snippets" => (query.clone(), default_cache()),
        [cmd, query, flag, path] if cmd == "snippets" && flag == "--cache" => {
            (query.clone(), Some(PathBuf::from(path)))
        }
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };
    let Some(cache_path) = cache_path else {
        eprintln!("HOME is not set; pass --cache <file>");
        return ExitCode::from(2);
    };
    match run(&query, cache_path).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

async fn run(query: &str, cache_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(dir) = cache_path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let cache = Mutex::new(Cache::open(&cache_path)?);
    let sources = Sources::new(Http::new());
    let start = Instant::now();

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

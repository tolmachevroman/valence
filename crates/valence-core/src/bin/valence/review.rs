//! The 50-molecule review set (M3.7).
//!
//! `valence review fetch` saves each molecule in `fixtures/review/molecules.txt`
//! as the snippets the core extracts, one JSON file each. `valence review run`
//! (feature `model`) writes every card for them and reports against the PRD
//! targets, with the cards themselves for review.

use std::path::{Path, PathBuf};
use std::time::{Instant, SystemTime};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::unbounded_channel;
use valence_core::card::Snippet;
use valence_core::fetch::{Sources, fetch};
use valence_core::ghs::Ghs;
use valence_core::http::Http;
use valence_core::pubchem::Compound;
use valence_core::text::clean;

use crate::Result;

/// One molecule of the review set, as fetched.
#[derive(Serialize, Deserialize)]
pub struct Fixture {
    /// The name searched for.
    pub query: String,
    pub compound: Compound,
    pub ghs: Option<Ghs>,
    pub has_3d: bool,
    /// Ids are positions, `1..`.
    pub snippets: Vec<Snippet>,
}

pub fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review")
}

pub fn slug(query: &str) -> String {
    query.to_lowercase().replace([' ', ','], "-")
}

fn names() -> Result<Vec<String>> {
    let list = std::fs::read_to_string(dir().join("molecules.txt"))?;
    Ok(list
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect())
}

#[cfg_attr(not(feature = "model"), allow(dead_code))]
pub fn load_all() -> Result<Vec<Fixture>> {
    names()?
        .iter()
        .map(|name| {
            let path = dir().join(format!("{}.json", slug(name)));
            let text = std::fs::read_to_string(&path)
                .map_err(|e| format!("{}: {e}; run `valence review fetch`", path.display()))?;
            let mut f: Fixture = serde_json::from_str(&text)?;
            // Saved with an older cleanup, perhaps; cleaning twice changes nothing.
            for s in &mut f.snippets {
                s.text = clean(&s.text);
            }
            Ok(f)
        })
        .collect()
}

/// Fetch every molecule in the list that isn't saved yet. Delete a file to
/// fetch that molecule again.
pub async fn fetch_all() -> Result<()> {
    let sources = Sources::new(Http::new());
    let (tx, _rx) = unbounded_channel();
    for name in names()? {
        let path = dir().join(format!("{}.json", slug(&name)));
        if path.exists() {
            continue;
        }
        let started = Instant::now();
        let Some(cid) = sources.pubchem.resolve(&name).await? else {
            return Err(format!("PubChem has no compound named \"{name}\"").into());
        };
        let m = fetch(&sources, cid, SystemTime::now(), &tx).await?;
        let snippets = m
            .snippets
            .into_iter()
            .enumerate()
            .map(|(i, s)| Snippet {
                id: (i + 1).to_string(),
                ..s
            })
            .collect::<Vec<_>>();
        eprintln!(
            "{name:24} CID {cid:<9} {:2} snippets  {:.1} s",
            snippets.len(),
            started.elapsed().as_secs_f64()
        );
        let fixture = Fixture {
            query: name.clone(),
            compound: m.compound,
            ghs: m.ghs,
            has_3d: m.sdf_3d.is_some(),
            snippets,
        };
        std::fs::write(path, serde_json::to_string_pretty(&fixture)? + "\n")?;
    }
    Ok(())
}

#[cfg(feature = "model")]
pub use run::run;

#[cfg(feature = "model")]
mod run {
    use std::collections::BTreeMap;
    use std::fmt::Write as _;
    use std::path::Path;
    use std::time::Instant;

    use serde::Serialize;
    use valence_core::card::CardKind;
    use valence_core::cards::{self, Event, Outcome};
    use valence_core::model::LocalModel;

    use super::{Fixture, load_all};
    use crate::Result;

    /// PRD targets (success metrics and non-functional requirements).
    const FIRST_CARD_S: f64 = 10.0;
    const STREAM_START_S: f64 = 3.0;
    const MIN_CARDS: usize = 3;

    #[derive(Serialize)]
    struct CardRun {
        molecule: String,
        kind: CardKind,
        /// written, untitled, retried, skipped, dropped, no input, from data, error.
        outcome: &'static str,
        title: Option<String>,
        body: Option<String>,
        /// Cited snippets: label, url, text.
        cited: Vec<(String, String, String)>,
        problems: Vec<String>,
        /// Problems of attempts that were retried.
        rejected: Vec<String>,
        /// Seconds from the start of the molecule's cards.
        first_text: Option<f64>,
        done: f64,
        seconds: f64,
    }

    impl CardRun {
        fn shown(&self) -> bool {
            matches!(
                self.outcome,
                "written" | "untitled" | "retried" | "from data"
            )
        }
    }

    async fn molecule(model: &LocalModel, f: &Fixture) -> Vec<CardRun> {
        let start = Instant::now();
        let name = &f.compound.title;
        let mut runs = Vec::new();
        for kind in cards::order(&f.snippets) {
            let began = start.elapsed().as_secs_f64();
            let (mut first_text, mut retried, mut rejected) = (None, false, Vec::new());
            let outcome = if kind == CardKind::Danger {
                Ok(cards::danger(name, &f.snippets).map_or(Outcome::NoInput, Outcome::Card))
            } else {
                cards::card(
                    model,
                    kind,
                    name,
                    &f.compound.synonyms,
                    &f.snippets,
                    &mut |e| match e {
                        Event::Draft { .. } => {
                            first_text.get_or_insert_with(|| start.elapsed().as_secs_f64());
                        }
                        Event::Retrying { problems } => {
                            retried = true;
                            rejected.extend(problems);
                        }
                        _ => {}
                    },
                )
                .await
            };
            let done = start.elapsed().as_secs_f64();
            let mut run = CardRun {
                molecule: f.query.clone(),
                kind,
                outcome: "error",
                title: None,
                body: None,
                cited: vec![],
                problems: vec![],
                rejected: vec![],
                first_text,
                done,
                seconds: done - began,
            };
            match outcome {
                Ok(Outcome::Card(card)) => {
                    run.outcome = if kind == CardKind::Danger {
                        "from data"
                    } else if card.title.is_empty() {
                        "untitled"
                    } else if retried {
                        "retried"
                    } else {
                        "written"
                    };
                    run.cited = card
                        .sources
                        .iter()
                        .filter_map(|s| {
                            let snippet = f.snippets.iter().find(|x| x.id == s.snippet_id)?;
                            Some((s.label.clone(), s.url.clone(), snippet.text.clone()))
                        })
                        .collect();
                    run.title = Some(card.title);
                    run.body = Some(card.body);
                }
                Ok(Outcome::Skipped) => run.outcome = "skipped",
                Ok(Outcome::Dropped(problems)) => {
                    run.outcome = "dropped";
                    run.problems = problems;
                }
                Ok(Outcome::NoInput) => run.outcome = "no input",
                Err(e) => run.problems = vec![e.to_string()],
            }
            run.rejected = rejected;
            eprintln!(
                "{:24} {:7} {:10} {:5.1} s",
                f.query,
                kind.as_str(),
                run.outcome,
                run.seconds
            );
            runs.push(run);
        }
        runs
    }

    fn percentile(values: &[f64], p: f64) -> f64 {
        if values.is_empty() {
            return f64::NAN;
        }
        let mut v = values.to_vec();
        v.sort_by(f64::total_cmp);
        v[((v.len() - 1) as f64 * p).round() as usize]
    }

    fn spread(values: &[f64]) -> String {
        format!(
            "{:.1} s median, {:.1} s p90, {:.1} s max",
            percentile(values, 0.5),
            percentile(values, 0.9),
            percentile(values, 1.0)
        )
    }

    fn report(model_name: &str, load: f64, runs: &[CardRun]) -> String {
        let mut by_molecule: BTreeMap<&str, Vec<&CardRun>> = BTreeMap::new();
        for r in runs {
            by_molecule.entry(&r.molecule).or_default().push(r);
        }
        let n = by_molecule.len();
        let model_card = |r: &&&CardRun| r.shown() && r.kind != CardKind::Danger;
        let first_model_card: Vec<f64> = by_molecule
            .values()
            .filter_map(|c| c.iter().find(model_card).map(|r| r.done))
            .collect();
        let stream_start: Vec<f64> = by_molecule
            .values()
            .filter_map(|c| c.iter().find_map(|r| r.first_text))
            .collect();
        let card_time: Vec<f64> = runs
            .iter()
            .filter(|r| r.kind != CardKind::Danger && r.outcome != "no input")
            .map(|r| r.seconds)
            .collect();
        let shown = |c: &Vec<&CardRun>| c.iter().filter(|r| r.shown()).count();
        let enough = by_molecule
            .values()
            .filter(|c| shown(c) >= MIN_CARDS)
            .count();
        let within = |v: &[f64], limit: f64| v.iter().filter(|&&s| s < limit).count();

        let mut md = String::new();
        let _ = writeln!(md, "# Review run: {model_name}\n");
        let _ = writeln!(
            md,
            "{n} molecules from `fixtures/review`, cards written as the app writes them. \
             Times start when a molecule's snippets are in hand; fetching adds 1–3 s uncached. \
             The Danger card is built from data and appears at once, so first-card times count model cards.\n"
        );
        let _ = writeln!(md, "## Against the PRD targets\n");
        let _ = writeln!(md, "| Measure | Target | Result |\n| --- | --- | --- |");
        let _ = writeln!(
            md,
            "| First model card | under {FIRST_CARD_S:.0} s | {} of {} molecules; {} |",
            within(&first_model_card, FIRST_CARD_S),
            first_model_card.len(),
            spread(&first_model_card)
        );
        let _ = writeln!(
            md,
            "| Streaming starts | under {STREAM_START_S:.0} s | {} of {}; {} |",
            within(&stream_start, STREAM_START_S),
            stream_start.len(),
            spread(&stream_start)
        );
        let _ = writeln!(
            md,
            "| Molecules with {MIN_CARDS}+ cards | 90% | {enough} of {n} ({:.0}%) |",
            100.0 * enough as f64 / n.max(1) as f64
        );
        let _ = writeln!(
            md,
            "| Cards with a claim not in the sources | 0 | by review, below |"
        );
        let _ = writeln!(md, "| Model load | under 10 s | {load:.1} s |");
        let _ = writeln!(
            md,
            "| Model card, including retry | | {} |",
            spread(&card_time)
        );

        let _ = writeln!(
            md,
            "\n## Outcomes\n\n| Card | written | untitled | retried | skipped | dropped | no input | error |\n| --- | --- | --- | --- | --- | --- | --- | --- |"
        );
        for kind in [CardKind::Where, CardKind::Origin, CardKind::Weird] {
            let count = |o: &str| {
                runs.iter()
                    .filter(|r| r.kind == kind && r.outcome == o)
                    .count()
            };
            let _ = writeln!(
                md,
                "| {} | {} | {} | {} | {} | {} | {} | {} |",
                kind.label(),
                count("written"),
                count("untitled"),
                count("retried"),
                count("skipped"),
                count("dropped"),
                count("no input"),
                count("error")
            );
        }

        let _ = writeln!(
            md,
            "\n## Cards\n\nEach claim should be stated by the cited snippets, folded under the card."
        );
        for (molecule, cards) in &by_molecule {
            let _ = writeln!(md, "\n### {molecule} ({} cards)\n", shown(cards));
            for r in cards {
                let _ = writeln!(
                    md,
                    "**{}** · {} · {:.1} s\n",
                    r.kind.label(),
                    r.outcome,
                    r.seconds
                );
                if let (Some(title), Some(body)) = (&r.title, &r.body) {
                    let _ = writeln!(md, "> **{title}**  \n> {body}\n");
                    if r.kind != CardKind::Danger {
                        let _ = writeln!(md, "<details><summary>Sources</summary>\n");
                        for (label, url, text) in &r.cited {
                            let _ = writeln!(md, "- [{label}]({url}): {text}");
                        }
                        let _ = writeln!(md, "\n</details>\n");
                    }
                }
                if !r.problems.is_empty() {
                    let _ = writeln!(md, "Problems: {}\n", r.problems.join("; "));
                }
            }
        }
        md
    }

    pub async fn run(model_path: &Path) -> Result<()> {
        let fixtures = load_all()?;
        let started = Instant::now();
        let model = LocalModel::load(model_path).await?;
        let load = started.elapsed().as_secs_f64();
        eprintln!("model loaded in {load:.1} s");

        let mut runs = Vec::new();
        for f in &fixtures {
            runs.extend(molecule(&model, f).await);
        }

        let model_name = model_path
            .file_stem()
            .map_or_else(|| "model".into(), |s| s.to_string_lossy().into_owned());
        let out = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/review/runs");
        std::fs::create_dir_all(&out)?;
        let jsonl: String = runs
            .iter()
            .map(|r| serde_json::to_string(r).map(|s| s + "\n"))
            .collect::<std::result::Result<_, _>>()?;
        std::fs::write(out.join(format!("{model_name}.jsonl")), jsonl)?;
        let md = out.join(format!("{model_name}.md"));
        std::fs::write(&md, report(&model_name, load, &runs))?;
        eprintln!("wrote {}", md.display());
        Ok(())
    }
}

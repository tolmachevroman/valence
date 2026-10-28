//! M0.5: generate every card for every spike fixture with one model, with one
//! stricter retry and the validator, and write a report for hand review.
//!
//! Run with:
//! `cargo run --release -p valence-core --example spike -- ~/Models/valence/<file>.gguf [--only caffeine,capsaicin]`
//!
//! Writes `docs/spike/runs/<model>.md` and `.jsonl`.

mod common;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use mistralrs::{Constraint, Model, RequestBuilder, TextMessageRole};
use serde::{Deserialize, Serialize};
use valence_core::card::{CardDraft, CardKind, Snippet};
use valence_core::validate::{self, Rejection, Reply};
use valence_core::{ghs, prompt, select};

use common::{PeakMemory, gb, load_model, stream};

const MAX_NEW_TOKENS: usize = 300;
/// M0.6 used 0.6; lowered in M0.6b for faithfulness to the snippets.
const TEMPERATURE: f64 = 0.2;

#[derive(Deserialize)]
struct Fixture {
    molecule: String,
    cid: u64,
    snippets: Vec<Snippet>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Outcome {
    Passed,
    PassedOnRetry,
    Skipped,
    Rejected,
    NoInput,
    /// Built from data without the model (the Danger card).
    FromData,
}

impl Outcome {
    fn label(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::PassedOnRetry => "passed on retry",
            Self::Skipped => "skipped by model",
            Self::Rejected => "rejected",
            Self::NoInput => "no input",
            Self::FromData => "from GHS data",
        }
    }

    /// Whether the card would appear in the app.
    fn shown(self) -> bool {
        matches!(self, Self::Passed | Self::PassedOnRetry | Self::FromData)
    }
}

#[derive(Serialize)]
struct Attempt {
    output: String,
    problems: Vec<String>,
    /// Short problem kinds, for the tally.
    categories: Vec<&'static str>,
    first_token: Option<f64>,
    seconds: f64,
    prompt_tokens: Option<usize>,
    completion_tokens: Option<usize>,
}

#[derive(Serialize)]
struct Cited {
    id: String,
    fixture_id: String,
    label: String,
    url: String,
}

#[derive(Serialize)]
struct CardResult {
    molecule: String,
    kind: CardKind,
    snippets: usize,
    outcome: Outcome,
    card: Option<CardDraft>,
    cited: Vec<Cited>,
    /// First step of a two-step card: the model choosing its snippets.
    pick: Option<Attempt>,
    attempts: Vec<Attempt>,
    /// Seconds from the start of this molecule to the end of this card.
    done_at: f64,
}

impl CardResult {
    /// Model time for this card: pick step plus every attempt.
    fn model_seconds(&self) -> f64 {
        self.pick.as_ref().map_or(0.0, |p| p.seconds)
            + self.attempts.iter().map(|a| a.seconds).sum::<f64>()
    }
}

fn category(r: &Rejection) -> &'static str {
    match r {
        Rejection::NoSources | Rejection::UnknownSource(_) => "bad sources",
        Rejection::UnsupportedNumber(_) => "number not in sources",
        Rejection::UnsupportedCode(_) => "code not in sources",
        Rejection::UnsupportedName(_) => "name not in sources",
        Rejection::Sentences(_) => "sentence count",
        Rejection::TooLong(_) => "too long",
        Rejection::EmptyTitle | Rejection::TitleTooLong(_) => "title",
    }
}

fn label(s: &Snippet) -> String {
    let source = if s.source == "wikipedia" {
        "Wikipedia"
    } else {
        "PubChem"
    };
    format!("{source}, {}", s.section)
}

async fn attempt(
    model: &Model,
    kind: CardKind,
    molecule: &str,
    picked: &[&Snippet],
    retry: Option<&[String]>,
) -> Result<(Attempt, Option<Reply>)> {
    let p = prompt::build(kind, molecule, picked, retry);
    let schema = prompt::output_schema(kind, picked.len());
    let request = RequestBuilder::new()
        .add_message(TextMessageRole::System, p.system)
        .add_message(TextMessageRole::User, p.user)
        .enable_thinking(false)
        .set_constraint(Constraint::JsonSchema(schema))
        .set_sampler_max_len(MAX_NEW_TOKENS)
        .set_sampler_temperature(TEMPERATURE);
    let g = stream(model, request, |_| {}).await?;

    let (reply, problems, categories) = match validate::parse(&g.text) {
        Ok(Reply::Skip) => (Some(Reply::Skip), vec![], vec![]),
        Ok(Reply::Card(card)) => match validate::validate(&card, molecule, picked) {
            Ok(()) => (Some(Reply::Card(card)), vec![], vec![]),
            Err(rs) => (
                None,
                rs.iter().map(ToString::to_string).collect(),
                rs.iter().map(category).collect(),
            ),
        },
        Err(e) => (None, vec![e.to_string()], vec!["unparseable"]),
    };
    let attempt = Attempt {
        output: g.text,
        problems,
        categories,
        first_token: g.first_token,
        seconds: g.seconds,
        prompt_tokens: g.usage.as_ref().map(|u| u.prompt_tokens),
        completion_tokens: g.usage.as_ref().map(|u| u.completion_tokens),
    };
    Ok((attempt, reply))
}

async fn run_card(
    model: &Model,
    fixture: &Fixture,
    kind: CardKind,
    started: Instant,
) -> Result<CardResult> {
    let mut picked = select::select(kind, &fixture.snippets);
    let mut result = CardResult {
        molecule: fixture.molecule.clone(),
        kind,
        snippets: picked.len(),
        outcome: Outcome::NoInput,
        card: None,
        cited: vec![],
        pick: None,
        attempts: vec![],
        done_at: 0.0,
    };

    if kind == CardKind::Danger {
        if let Some(s) = picked.first() {
            let ghs = ghs::Ghs::from_snippet_text(&s.text);
            result.card = Some(ghs::danger_card(&fixture.molecule, &ghs));
            result.cited = vec![Cited {
                id: "s1".into(),
                fixture_id: s.id.clone(),
                label: label(s),
                url: s.url.clone(),
            }];
            result.outcome = Outcome::FromData;
        }
        result.done_at = started.elapsed().as_secs_f64();
        return Ok(result);
    }

    if let (Some(_), true) = (prompt::pick_limit(kind), picked.len() > 1) {
        let p = prompt::build_pick(kind, &fixture.molecule, &picked);
        let request = RequestBuilder::new()
            .add_message(TextMessageRole::System, p.system)
            .add_message(TextMessageRole::User, p.user)
            .enable_thinking(false)
            .set_constraint(Constraint::JsonSchema(prompt::pick_schema(
                kind,
                picked.len(),
            )))
            .set_sampler_max_len(40)
            .set_sampler_temperature(TEMPERATURE);
        let g = stream(model, request, |_| {}).await?;
        let chosen = prompt::parse_pick(&g.text, picked.len());
        result.pick = Some(Attempt {
            problems: if chosen.is_some() {
                vec![]
            } else {
                vec!["unparseable pick; wrote from all snippets".into()]
            },
            categories: vec![],
            output: g.text,
            first_token: g.first_token,
            seconds: g.seconds,
            prompt_tokens: g.usage.as_ref().map(|u| u.prompt_tokens),
            completion_tokens: g.usage.as_ref().map(|u| u.completion_tokens),
        });
        if let Some(chosen) = chosen {
            picked = chosen.into_iter().map(|i| picked[i]).collect();
            result.snippets = picked.len();
        }
    }

    if !picked.is_empty() {
        for retry in [false, true] {
            let problems = result.attempts.last().map(|a| a.problems.as_slice());
            let (a, reply) = attempt(model, kind, &fixture.molecule, &picked, problems).await?;
            result.attempts.push(a);
            match reply {
                Some(Reply::Skip) => result.outcome = Outcome::Skipped,
                Some(Reply::Card(card)) => {
                    result.outcome = if retry {
                        Outcome::PassedOnRetry
                    } else {
                        Outcome::Passed
                    };
                    result.cited = card
                        .sources
                        .iter()
                        .filter_map(|id| {
                            let n: usize = id.strip_prefix('s')?.parse().ok()?;
                            let s = picked.get(n.checked_sub(1)?)?;
                            Some(Cited {
                                id: id.clone(),
                                fixture_id: s.id.clone(),
                                label: label(s),
                                url: s.url.clone(),
                            })
                        })
                        .collect();
                    result.card = Some(card);
                }
                None => {
                    result.outcome = Outcome::Rejected;
                    continue;
                }
            }
            break;
        }
    }
    result.done_at = started.elapsed().as_secs_f64();
    Ok(result)
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

fn report(
    model_name: &str,
    kv_cache: &str,
    load: f64,
    peak: u64,
    results: &[CardResult],
) -> String {
    let count = |o: Outcome| results.iter().filter(|r| r.outcome == o).count();
    let with_input = results.len() - count(Outcome::NoInput);
    let pct = |n: usize| 100.0 * n as f64 / with_input.max(1) as f64;

    let mut by_molecule: BTreeMap<&str, Vec<&CardResult>> = BTreeMap::new();
    for r in results {
        by_molecule.entry(&r.molecule).or_default().push(r);
    }
    let three_plus = by_molecule
        .values()
        .filter(|cards| cards.iter().filter(|r| r.outcome.shown()).count() >= 3)
        .count();
    let first_card: Vec<f64> = by_molecule
        .values()
        .filter_map(|cards| cards.iter().find(|r| r.outcome.shown()).map(|r| r.done_at))
        .collect();
    let first_model_card: Vec<f64> = by_molecule
        .values()
        .filter_map(|cards| {
            cards
                .iter()
                .find(|r| r.outcome.shown() && r.outcome != Outcome::FromData)
                .map(|r| r.done_at)
        })
        .collect();
    // Time until the reader sees text: the pick step, then the first token of the card.
    let first_token: Vec<f64> = results
        .iter()
        .filter_map(|r| {
            let pick = r.pick.as_ref().map_or(0.0, |p| p.seconds);
            Some(pick + r.attempts.first()?.first_token?)
        })
        .collect();
    let card_time: Vec<f64> = results
        .iter()
        .filter(|r| !r.attempts.is_empty())
        .map(CardResult::model_seconds)
        .collect();
    let mut tally: BTreeMap<&str, usize> = BTreeMap::new();
    for a in results.iter().flat_map(|r| &r.attempts) {
        for c in &a.categories {
            *tally.entry(c).or_default() += 1;
        }
    }

    let mut md = String::new();
    let _ = writeln!(md, "# Spike run: {model_name}\n");
    let _ = writeln!(
        md,
        "{} molecules, {} cards with input. Temperature {TEMPERATURE}, JSON schema constraint, one stricter retry, shortest card first, pick-then-write for Weird and Where, Danger from GHS data, KV cache: {kv_cache}.\n",
        by_molecule.len(),
        with_input
    );
    let _ = writeln!(md, "## Summary\n\n| Measure | Value |\n| --- | --- |");
    for (name, o) in [
        ("From GHS data (no model)", Outcome::FromData),
        ("Passed first try", Outcome::Passed),
        ("Passed on retry", Outcome::PassedOnRetry),
        ("Skipped by model", Outcome::Skipped),
        ("Rejected twice (dropped)", Outcome::Rejected),
    ] {
        let _ = writeln!(md, "| {name} | {} ({:.0}%) |", count(o), pct(count(o)));
    }
    let _ = writeln!(md, "| No input (no card) | {} |", count(Outcome::NoInput));
    let _ = writeln!(
        md,
        "| Molecules with 3+ cards | {three_plus} of {} |",
        by_molecule.len()
    );
    let _ = writeln!(
        md,
        "| First text of a model card | {} |",
        spread(&first_token)
    );
    let _ = writeln!(
        md,
        "| Model card, including pick and retry | {} |",
        spread(&card_time)
    );
    let _ = writeln!(md, "| First card per molecule | {} |", spread(&first_card));
    let _ = writeln!(
        md,
        "| First model card per molecule | {} |",
        spread(&first_model_card)
    );
    let _ = writeln!(md, "| Model load | {load:.1} s |");
    let _ = writeln!(md, "| Peak memory | {:.2} GB |", gb(peak));
    let _ = writeln!(md, "\n### Problems found (all attempts)\n");
    if tally.is_empty() {
        let _ = writeln!(md, "None.");
    }
    for (c, n) in &tally {
        let _ = writeln!(md, "- {c}: {n}");
    }

    let _ = writeln!(md, "\n## Cards\n\nFun score is for hand review in M0.6.");
    for (molecule, cards) in &by_molecule {
        let _ = writeln!(md, "\n### {molecule}\n");
        for r in cards {
            let secs = r.model_seconds();
            let _ = writeln!(
                md,
                "**{}** · {} · {} snippets · {secs:.1} s\n",
                r.kind.label(),
                r.outcome.label(),
                r.snippets
            );
            if let Some(card) = &r.card {
                let _ = writeln!(md, "> **{}**  \n> {}\n", card.title, card.body);
                let sources: Vec<_> = r
                    .cited
                    .iter()
                    .map(|c| format!("{} ({})", c.id, c.label))
                    .collect();
                let _ = writeln!(md, "Sources: {}  \nFun: _ /5\n", sources.join(", "));
            }
            for (i, a) in r.attempts.iter().enumerate() {
                if a.problems.is_empty() {
                    continue;
                }
                let _ = writeln!(
                    md,
                    "<details><summary>Attempt {} rejected: {}</summary>\n\n```json\n{}\n```\n</details>\n",
                    i + 1,
                    a.problems.join("; "),
                    a.output.trim()
                );
            }
        }
    }
    md
}

fn load_fixtures(dir: &Path, only: &[String]) -> Result<Vec<Fixture>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.extension().is_some_and(|x| x == "json"))
        .collect();
    paths.sort();
    let mut fixtures = Vec::new();
    for path in paths {
        let fixture: Fixture = serde_json::from_str(&std::fs::read_to_string(&path)?)
            .with_context(|| format!("reading {}", path.display()))?;
        if only.is_empty() || only.contains(&fixture.molecule) {
            fixtures.push(fixture);
        }
    }
    Ok(fixtures)
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let model_path: PathBuf = args
        .next()
        .context("usage: spike <path/to/model.gguf> [--only a,b]")?
        .into();
    let mut only = Vec::new();
    while let Some(arg) = args.next() {
        if arg == "--only" {
            only = args
                .next()
                .context("--only needs a list")?
                .split(',')
                .map(str::to_string)
                .collect();
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixtures = load_fixtures(&root.join("fixtures/spike"), &only)?;
    let model_name = model_path
        .file_stem()
        .context("model path has no file name")?
        .to_string_lossy()
        .to_string();

    let memory = PeakMemory::start();
    let start = Instant::now();
    let (model, kv_cache) = load_model(&model_path).await?;
    let load = start.elapsed().as_secs_f64();
    eprintln!("loaded {model_name} in {load:.1} s");

    let mut results = Vec::new();
    for fixture in &fixtures {
        let started = Instant::now();
        // Shortest prompt first, so the first card appears as early as possible;
        // the app can still display cards in a fixed order.
        let mut kinds = CardKind::ALL;
        kinds.sort_by_key(|&k| {
            select::select(k, &fixture.snippets)
                .iter()
                .map(|s| s.text.len())
                .sum::<usize>()
        });
        for kind in kinds {
            let r = run_card(&model, fixture, kind, started).await?;
            eprintln!(
                "{:22} cid {:<9} {:7} {:16} {:5.1} s",
                fixture.molecule,
                fixture.cid,
                kind.as_str(),
                r.outcome.label(),
                r.model_seconds()
            );
            results.push(r);
        }
    }

    let out_dir = root.join("docs/spike/runs");
    std::fs::create_dir_all(&out_dir)?;
    let jsonl: String = results
        .iter()
        .map(|r| serde_json::to_string(r).map(|s| s + "\n"))
        .collect::<Result<_, _>>()?;
    std::fs::write(out_dir.join(format!("{model_name}.jsonl")), jsonl)?;
    let md = report(&model_name, kv_cache, load, memory.peak(), &results);
    std::fs::write(out_dir.join(format!("{model_name}.md")), &md)?;
    eprintln!(
        "wrote {}",
        out_dir.join(format!("{model_name}.md")).display()
    );
    Ok(())
}

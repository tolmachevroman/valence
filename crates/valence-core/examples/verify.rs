//! M0.6d: measure the claim check against the hand-reviewed M0.6b cards. For
//! every model-written card that was shown, run the check and compare its
//! verdict with the review's accuracy label.
//!
//! Run with:
//! `cargo run --release -p valence-core --example verify -- ~/Models/valence/<file>.gguf --style quote|verdict`
//!
//! Writes `docs/spike/runs/m0.6d/verify-<model>-<style>.md` and `.jsonl`.

mod common;

use std::collections::{BTreeMap, HashMap};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use mistralrs::{Constraint, RequestBuilder, TextMessageRole};
use serde::{Deserialize, Serialize};
use valence_core::card::{CardDraft, Snippet};
use valence_core::verify::{self, Style};

use common::{PeakMemory, gb, load_model, stream};

const REVIEWED_RUN: &str = "docs/spike/runs/m0.6b";

#[derive(Deserialize)]
struct Fixture {
    molecule: String,
    snippets: Vec<Snippet>,
}

#[derive(Deserialize)]
struct CitedRow {
    fixture_id: String,
}

#[derive(Deserialize)]
struct RunRow {
    molecule: String,
    kind: String,
    outcome: String,
    card: Option<CardDraft>,
    cited: Vec<CitedRow>,
}

#[derive(Deserialize)]
struct Review {
    molecule: String,
    kind: String,
    accuracy: String,
    issue: String,
}

#[derive(Serialize)]
struct Checked {
    writer: String,
    molecule: String,
    kind: String,
    accuracy: String,
    issue: String,
    claims: Vec<String>,
    output: String,
    /// One per claim; `None` if the reply didn't parse.
    verdicts: Option<Vec<bool>>,
    flagged: bool,
    first_token: Option<f64>,
    seconds: f64,
    prompt_tokens: Option<usize>,
    completion_tokens: Option<usize>,
}

fn read_jsonl<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<Vec<T>> {
    std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).with_context(|| format!("parsing {}", path.display())))
        .collect()
}

fn percentile(values: &[f64], p: f64) -> f64 {
    let mut v = values.to_vec();
    v.sort_by(f64::total_cmp);
    v.get(((v.len().max(1) - 1) as f64 * p).round() as usize)
        .copied()
        .unwrap_or(f64::NAN)
}

fn report(model_name: &str, style: &str, load: f64, peak: u64, rows: &[Checked]) -> String {
    let mut md = String::new();
    let _ = writeln!(md, "# Claim check: {model_name}, {style} style\n");
    let _ = writeln!(
        md,
        "{} model-written cards from the hand-reviewed run in `{REVIEWED_RUN}`. A card is flagged when any claim (title or sentence) is judged unsupported, or the reply doesn't parse.\n",
        rows.len()
    );

    let _ = writeln!(md, "## Flag rate by review label\n");
    let _ = writeln!(
        md,
        "| Writer | Review label | Cards | Flagged |\n| --- | --- | --- | --- |"
    );
    let mut groups: BTreeMap<(&str, &str), (usize, usize)> = BTreeMap::new();
    for r in rows {
        for writer in [r.writer.as_str(), "all"] {
            let g = groups.entry((writer, r.accuracy.as_str())).or_default();
            g.0 += 1;
            g.1 += usize::from(r.flagged);
        }
    }
    for ((writer, label), (n, flagged)) in &groups {
        let _ = writeln!(
            md,
            "| {writer} | {label} | {n} | {flagged} ({:.0}%) |",
            100.0 * *flagged as f64 / *n as f64
        );
    }

    let kept: Vec<_> = rows.iter().filter(|r| !r.flagged).collect();
    let unsupported = |rs: &[&Checked]| rs.iter().filter(|r| r.accuracy == "unsupported").count();
    let all: Vec<_> = rows.iter().collect();
    let _ = writeln!(
        md,
        "\nIf flagged cards were dropped: {} of {} cards kept; unsupported among shown cards {:.0}% → {:.0}%.\n",
        kept.len(),
        rows.len(),
        100.0 * unsupported(&all) as f64 / all.len().max(1) as f64,
        100.0 * unsupported(&kept) as f64 / kept.len().max(1) as f64,
    );

    let seconds: Vec<f64> = rows.iter().map(|r| r.seconds).collect();
    let _ = writeln!(md, "## Cost\n\n| Measure | Value |\n| --- | --- |");
    let _ = writeln!(
        md,
        "| Check time per card | {:.1} s median, {:.1} s p90 |",
        percentile(&seconds, 0.5),
        percentile(&seconds, 0.9)
    );
    let unparsed = rows.iter().filter(|r| r.verdicts.is_none()).count();
    let _ = writeln!(md, "| Unparseable replies | {unparsed} |");
    let _ = writeln!(md, "| Model load | {load:.1} s |");
    let _ = writeln!(md, "| Peak memory | {:.2} GB |", gb(peak));

    let _ = writeln!(md, "\n## Misses: unsupported cards that passed\n");
    for r in rows
        .iter()
        .filter(|r| r.accuracy == "unsupported" && !r.flagged)
    {
        let _ = writeln!(
            md,
            "- {} {} ({}): {}",
            r.molecule, r.kind, r.writer, r.issue
        );
    }
    let _ = writeln!(
        md,
        "\n## False alarms: cards reviewed ok that were flagged\n"
    );
    for r in rows.iter().filter(|r| r.accuracy == "ok" && r.flagged) {
        let flagged: Vec<_> = r
            .verdicts
            .iter()
            .flatten()
            .zip(&r.claims)
            .filter(|(ok, _)| !**ok)
            .map(|(_, c)| format!("\"{c}\""))
            .collect();
        let _ = writeln!(
            md,
            "- {} {} ({}): {}",
            r.molecule,
            r.kind,
            r.writer,
            if flagged.is_empty() {
                "unparseable".to_string()
            } else {
                flagged.join("; ")
            }
        );
    }
    md
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let usage = "usage: verify <path/to/model.gguf> --style quote|verdict";
    let model_path: PathBuf = args.next().context(usage)?.into();
    let mut style = Style::Quote;
    while let Some(arg) = args.next() {
        if arg == "--style" {
            style = match args.next().as_deref() {
                Some("quote") => Style::Quote,
                Some("verdict") => Style::Verdict,
                _ => bail!(usage),
            };
        }
    }
    let style_name = match style {
        Style::Quote => "quote",
        Style::Verdict => "verdict",
    };

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut fixtures: HashMap<String, HashMap<String, Snippet>> = HashMap::new();
    for entry in std::fs::read_dir(root.join("fixtures/spike"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|x| x == "json") {
            let f: Fixture = serde_json::from_str(&std::fs::read_to_string(&path)?)?;
            let by_id = f.snippets.into_iter().map(|s| (s.id.clone(), s)).collect();
            fixtures.insert(f.molecule, by_id);
        }
    }

    // (writer, molecule, kind, card, cited snippets, review)
    let mut cases = Vec::new();
    let mut runs: Vec<PathBuf> = std::fs::read_dir(root.join(REVIEWED_RUN))?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.to_string_lossy().ends_with(".review.jsonl"))
        .collect();
    runs.sort();
    for review_path in runs {
        let name = review_path.file_name().unwrap().to_string_lossy();
        let writer = name.trim_end_matches(".review.jsonl").to_string();
        let mut reviews: HashMap<(String, String), Review> = read_jsonl::<Review>(&review_path)?
            .into_iter()
            .map(|r| ((r.molecule.clone(), r.kind.clone()), r))
            .collect();
        for row in read_jsonl::<RunRow>(&review_path.with_file_name(format!("{writer}.jsonl")))? {
            let shown = matches!(row.outcome.as_str(), "passed" | "passed_on_retry");
            let (Some(card), true) = (row.card, shown) else {
                continue;
            };
            let Some(review) = reviews.remove(&(row.molecule.clone(), row.kind.clone())) else {
                continue;
            };
            let snippets = &fixtures[&row.molecule];
            let cited: Vec<Snippet> = row
                .cited
                .iter()
                .map(|c| snippets[&c.fixture_id].clone())
                .collect();
            cases.push((writer.clone(), row.molecule, row.kind, card, cited, review));
        }
    }
    eprintln!("{} reviewed model cards", cases.len());

    let model_name = model_path
        .file_stem()
        .context("model path has no file name")?
        .to_string_lossy()
        .to_string();
    let memory = PeakMemory::start();
    let start = Instant::now();
    let (model, _) = load_model(&model_path).await?;
    let load = start.elapsed().as_secs_f64();
    eprintln!("loaded {model_name} in {load:.1} s");

    let mut rows = Vec::new();
    for (writer, molecule, kind, card, cited, review) in cases {
        let refs: Vec<&Snippet> = cited.iter().collect();
        let claims: Vec<String> = verify::claims(&card)
            .into_iter()
            .map(str::to_string)
            .collect();
        let p = verify::build(style, &molecule, &card, &refs);
        let request = RequestBuilder::new()
            .add_message(TextMessageRole::System, p.system)
            .add_message(TextMessageRole::User, p.user)
            .enable_thinking(false)
            .set_constraint(Constraint::JsonSchema(verify::schema(style, claims.len())))
            .set_sampler_max_len(400)
            .set_sampler_temperature(0.0);
        let g = stream(&model, request, |_| {}).await?;
        let verdicts = verify::parse(&g.text, claims.len());
        let flagged = verdicts.as_ref().is_none_or(|v| v.contains(&false));
        eprintln!(
            "{writer:40} {molecule:22} {kind:7} {:12} {} {:4.1} s",
            review.accuracy,
            if flagged { "FLAG" } else { "ok  " },
            g.seconds
        );
        rows.push(Checked {
            writer,
            molecule,
            kind,
            accuracy: review.accuracy.clone(),
            issue: review.issue.clone(),
            claims,
            output: g.text,
            verdicts,
            flagged,
            first_token: g.first_token,
            seconds: g.seconds,
            prompt_tokens: g.usage.as_ref().map(|u| u.prompt_tokens),
            completion_tokens: g.usage.as_ref().map(|u| u.completion_tokens),
        });
    }

    let out_dir = root.join("docs/spike/runs/m0.6d");
    std::fs::create_dir_all(&out_dir)?;
    let stem = format!("verify-{model_name}-{style_name}");
    let jsonl: String = rows
        .iter()
        .map(|r| serde_json::to_string(r).map(|s| s + "\n"))
        .collect::<Result<_, _>>()?;
    std::fs::write(out_dir.join(format!("{stem}.jsonl")), jsonl)?;
    let md = report(&model_name, style_name, load, memory.peak(), &rows);
    std::fs::write(out_dir.join(format!("{stem}.md")), &md)?;
    eprintln!("wrote {}", out_dir.join(format!("{stem}.md")).display());
    Ok(())
}

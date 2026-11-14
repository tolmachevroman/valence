//! Snapshots of everything about the review set's cards that doesn't need the
//! model: the order cards are written in, the snippets each card is given,
//! and the Danger card. A change to extraction, scrubbing or selection shows
//! up here as a diff to review (`cargo insta review`). The model's cards are
//! reviewed from `valence review run` reports instead.

use std::fmt::Write as _;
use std::path::Path;

use serde::Deserialize;
use valence_core::card::{CardKind, Snippet};
use valence_core::{cards, select};

#[derive(Deserialize)]
struct Compound {
    title: String,
}

#[derive(Deserialize)]
struct Fixture {
    compound: Compound,
    snippets: Vec<Snippet>,
}

fn review_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/review")
}

fn names() -> Vec<String> {
    std::fs::read_to_string(review_dir().join("molecules.txt"))
        .unwrap()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(str::to_string)
        .collect()
}

/// A snippet in one line: id, where it is from, and how it starts.
fn line(s: &Snippet) -> String {
    let start: String = s.text.chars().take(70).collect();
    let more = if s.text.chars().count() > 70 {
        "…"
    } else {
        ""
    };
    format!("[{}] {} / {}: {start}{more}", s.id, s.source, s.section)
}

fn summary(f: &Fixture) -> String {
    let mut out = String::new();
    let order: Vec<&str> = cards::order(&f.snippets)
        .iter()
        .map(|k| k.as_str())
        .collect();
    let _ = writeln!(out, "order: {}", order.join(", "));
    for kind in CardKind::ALL {
        let _ = writeln!(out, "\n{}:", kind.as_str());
        if kind == CardKind::Danger {
            match cards::danger(&f.compound.title, &f.snippets) {
                Some(card) => {
                    let _ = writeln!(out, "  {}\n  {}", card.title, card.body);
                    for s in card.hazards.iter().flat_map(|h| &h.statements) {
                        let _ = writeln!(out, "  - {}: {}", s.code, s.text);
                    }
                }
                None => {
                    let _ = writeln!(out, "  (no GHS data)");
                }
            }
            continue;
        }
        let picked = select::select(kind, &f.snippets);
        if picked.is_empty() {
            let _ = writeln!(out, "  (no input)");
        }
        for s in picked {
            let _ = writeln!(out, "  {}", line(s));
        }
    }
    out
}

#[test]
fn review_set_selection() {
    let names = names();
    assert_eq!(names.len(), 50);
    for name in names {
        let slug = name.to_lowercase().replace([' ', ','], "-");
        let text = std::fs::read_to_string(review_dir().join(format!("{slug}.json")))
            .unwrap_or_else(|e| panic!("{slug}.json: {e}; run `valence review fetch`"));
        let f: Fixture = serde_json::from_str(&text).unwrap();
        insta::assert_snapshot!(slug, summary(&f));
    }
}

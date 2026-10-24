//! Card prompts: shared rules, a brief and one example per card type, and the
//! JSON schema that constrains the model's output.

use std::fmt::Write;

use serde_json::{Value, json};

use crate::card::{CardKind, Snippet};

/// A chat prompt for one card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prompt {
    pub system: String,
    pub user: String,
}

const RULES: &str = "\
You write fact cards for Valence, an app that makes chemistry fun.

Rules:
- Use only facts stated in the snippets. If a fact is not in them, leave it out, even if you are sure it is true.
- Copy every number, year and name exactly as the snippets give them.
- Be playful in tone but exact in content. Do not exaggerate danger or safety.
- Never describe how to make, extract or concentrate a substance.
- Cite every snippet the body relies on.

Reply with one JSON object and nothing else:
{\"title\": \"...\", \"body\": \"...\", \"sources\": [\"s1\", \"s2\"]}
- title: a hook of at most 8 words, also based only on the snippets
- body: 2 to 4 sentences, under 80 words
- sources: snippet ids written exactly as they appear, like \"s3\"";

const SKIP_RULE: &str =
    "If the snippets contain nothing that fits this card, reply {\"skip\": true} instead.";

const RETRY_RULE: &str = "\
Your previous answer was rejected. Write the card again, more carefully: \
use only words, numbers and names that appear in the snippets you cite, and cite each one.";

struct Example {
    molecule: &'static str,
    snippets: &'static [(&'static str, &'static str)],
    card: &'static str,
}

fn brief(kind: CardKind) -> &'static str {
    match kind {
        CardKind::Where => {
            "Card: Where you meet it. The everyday places this molecule shows up: foods, \
             products, medicines, nature. Prefer concrete, familiar examples."
        }
        CardKind::Origin => {
            "Card: Origin story. Who discovered or first made it, when, and how it happened. \
             Keep people, places and years exactly as given."
        }
        CardKind::Danger => {
            "Card: Danger level. Explain the official GHS hazard statements in plain language \
             for a curious adult. List no hazards beyond them, and never state doses, safe \
             amounts or treatments."
        }
        CardKind::Weird => {
            "Card: Weird fact. The single most surprising detail in the snippets, the kind that \
             makes a reader say \"wait, really?\". Focus on one fact and why it is surprising."
        }
    }
}

// Examples use molecules outside the spike set so they don't leak answers.
fn example(kind: CardKind) -> Example {
    match kind {
        CardKind::Where => Example {
            molecule: "acetic acid",
            snippets: &[
                (
                    "PubChem, Uses",
                    "Acetic acid is the main component of vinegar, apart from water.",
                ),
                (
                    "Wikipedia, Introduction",
                    "Vinegar is typically 4–8% acetic acid by volume.",
                ),
                (
                    "PubChem, Uses",
                    "Used as a food additive (E260) to regulate acidity.",
                ),
            ],
            card: r#"{"title": "The sour in your salad", "body": "Every splash of vinegar is mostly water plus acetic acid, which typically makes up 4–8% of it by volume. The same molecule appears on food labels as E260, where it keeps acidity in check.", "sources": ["s1", "s2", "s3"]}"#,
        },
        CardKind::Origin => Example {
            molecule: "saccharin",
            snippets: &[
                (
                    "Wikipedia, History",
                    "Saccharin was produced first in 1879 by Constantin Fahlberg, a chemist working on coal tar derivatives in Ira Remsen's laboratory at Johns Hopkins University.",
                ),
                (
                    "Wikipedia, History",
                    "Fahlberg noticed a sweet taste on his hands one evening and connected this with the compound benzoic sulfimide on which he had been working.",
                ),
            ],
            card: r#"{"title": "A sweet taste on his hands", "body": "In 1879, Constantin Fahlberg was studying coal tar derivatives in Ira Remsen's laboratory at Johns Hopkins University. One evening he noticed a sweet taste on his hands and traced it to benzoic sulfimide, the compound he had been working on. That compound was saccharin.", "sources": ["s1", "s2"]}"#,
        },
        CardKind::Danger => Example {
            molecule: "acetone",
            snippets: &[(
                "PubChem, GHS Classification",
                "Signal word: Danger. Pictograms: Flammable, Irritant. H225: Highly Flammable liquid and vapor [Danger Flammable liquids] H319: Causes serious eye irritation [Warning Serious eye damage/eye irritation] H336: May cause drowsiness or dizziness [Warning Specific target organ toxicity, single exposure; Narcotic effects]",
            )],
            card: r#"{"title": "Keep it away from flames and eyes", "body": "Acetone is officially rated highly flammable, both as a liquid and as a vapor. It can cause serious eye irritation, and breathing it may cause drowsiness or dizziness.", "sources": ["s1"]}"#,
        },
        CardKind::Weird => Example {
            molecule: "hydrogen sulfide",
            snippets: &[
                (
                    "Wikipedia, Introduction",
                    "Hydrogen sulfide is a colorless gas with the characteristic foul odor of rotten eggs.",
                ),
                (
                    "Wikipedia, Safety",
                    "Although the smell is easy to notice at first, at high concentrations hydrogen sulfide quickly deadens the sense of smell.",
                ),
            ],
            card: r#"{"title": "The smell that switches itself off", "body": "Hydrogen sulfide is famous for smelling like rotten eggs. The strange part: at high concentrations it quickly deadens the sense of smell, so the warning fades just when there is more of the gas around.", "sources": ["s1", "s2"]}"#,
        },
    }
}

fn source_label(snippet: &Snippet) -> String {
    let source = match snippet.source.as_str() {
        "pubchem_view" => "PubChem",
        "wikipedia" => "Wikipedia",
        other => other,
    };
    format!("{source}, {}", snippet.section)
}

fn write_task(out: &mut String, kind: CardKind, molecule: &str, snippets: &[(String, String)]) {
    let _ = writeln!(out, "Molecule: {molecule}\nSnippets:");
    for (i, (label, text)) in snippets.iter().enumerate() {
        let _ = writeln!(out, "[s{}] ({label}) {text}", i + 1);
    }
    let _ = write!(out, "Write the \"{}\" card.", kind.label());
}

/// Build the prompt for one card. Snippets are renumbered `s1..sN` in the
/// order given; callers map ids back with the same order. Pass `retry` after a
/// rejected attempt to get the stricter prompt.
pub fn build(kind: CardKind, molecule: &str, snippets: &[&Snippet], retry: bool) -> Prompt {
    let ex = example(kind);
    let ex_snippets: Vec<_> = ex
        .snippets
        .iter()
        .map(|(label, text)| (label.to_string(), text.to_string()))
        .collect();

    let mut system = String::from(RULES);
    let _ = write!(system, "\n\n{}", brief(kind));
    if kind.can_skip() {
        let _ = write!(system, " {SKIP_RULE}");
    }
    system.push_str("\n\nExample\n");
    write_task(&mut system, kind, ex.molecule, &ex_snippets);
    let _ = write!(system, "\n{}", ex.card);
    if retry {
        let _ = write!(system, "\n\n{RETRY_RULE}");
    }

    let snippets: Vec<_> = snippets
        .iter()
        .map(|s| (source_label(s), s.text.clone()))
        .collect();
    let mut user = String::new();
    write_task(&mut user, kind, molecule, &snippets);

    Prompt { system, user }
}

/// JSON schema for the model's reply, for constrained decoding.
pub fn output_schema(kind: CardKind) -> Value {
    let card = json!({
        "type": "object",
        "properties": {
            "title": { "type": "string" },
            "body": { "type": "string" },
            "sources": {
                "type": "array",
                "items": { "type": "string", "pattern": "^s[0-9]+$" },
                "minItems": 1
            }
        },
        "required": ["title", "body", "sources"],
        "additionalProperties": false
    });
    if !kind.can_skip() {
        return card;
    }
    let skip = json!({
        "type": "object",
        "properties": { "skip": { "const": true } },
        "required": ["skip"],
        "additionalProperties": false
    });
    json!({ "anyOf": [card, skip] })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::CardDraft;

    fn snippet(section: &str, text: &str) -> Snippet {
        Snippet {
            id: "ignored".into(),
            source: "wikipedia".into(),
            section: section.into(),
            url: "https://en.wikipedia.org/wiki/Capsaicin".into(),
            origin: "Wikipedia".into(),
            text: text.into(),
        }
    }

    #[test]
    fn numbers_snippets_in_order() {
        let a = snippet("History", "First extracted in 1816.");
        let b = snippet("Natural function", "Birds do not feel it.");
        let p = build(CardKind::Weird, "capsaicin", &[&a, &b], false);
        assert!(
            p.user
                .contains("[s1] (Wikipedia, History) First extracted in 1816.")
        );
        assert!(
            p.user
                .contains("[s2] (Wikipedia, Natural function) Birds do not feel it.")
        );
        assert!(p.user.starts_with("Molecule: capsaicin"));
        assert!(p.user.ends_with("Write the \"Weird fact\" card."));
    }

    #[test]
    fn skip_and_retry_rules_only_where_they_apply() {
        let s = snippet("History", "First extracted in 1816.");
        assert!(
            build(CardKind::Origin, "x", &[&s], false)
                .system
                .contains(SKIP_RULE)
        );
        assert!(
            !build(CardKind::Danger, "x", &[&s], false)
                .system
                .contains(SKIP_RULE)
        );
        assert!(
            !build(CardKind::Where, "x", &[&s], false)
                .system
                .contains(RETRY_RULE)
        );
        assert!(
            build(CardKind::Where, "x", &[&s], true)
                .system
                .contains(RETRY_RULE)
        );
    }

    #[test]
    fn examples_follow_the_output_format() {
        for kind in CardKind::ALL {
            let ex = example(kind);
            let card: CardDraft = serde_json::from_str(ex.card).expect("example card parses");
            assert!(!card.sources.is_empty());
            for id in &card.sources {
                let n: usize = id.strip_prefix('s').unwrap().parse().unwrap();
                assert!((1..=ex.snippets.len()).contains(&n), "{kind:?} cites {id}");
            }
            assert!(card.body.split_whitespace().count() < 80);
        }
    }

    #[test]
    fn schema_allows_skip_only_for_skippable_kinds() {
        assert!(output_schema(CardKind::Weird).get("anyOf").is_some());
        assert!(output_schema(CardKind::Danger).get("anyOf").is_none());
    }
}

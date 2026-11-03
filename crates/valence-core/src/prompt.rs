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
- Keep each fact attached to what the snippet says it is about. Do not combine facts from different sentences into a new claim.
- Keep the snippet's hedges: if it says may, probably, possibly, proposed or suspected, say so too.
- Name only the examples, places and people the snippets name.
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
use only words, numbers and names that appear in the snippets you cite, cite each one, \
and write 2 to 4 sentences.";

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
             for a curious adult. List no hazards beyond them. Describe, don't advise: no \
             doses, safe amounts, treatments, or handling instructions."
        }
        CardKind::Weird => {
            "Card: Weird fact. The single most surprising detail in the snippets, the kind that \
             makes a reader say \"wait, really?\". Focus on one fact. Explain why it matters \
             only if the snippets say so; never add a cause or consequence of your own."
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
                    "Although the smell is easy to notice at first, at high concentrations hydrogen sulfide quickly deadens the sense of smell, so people may not notice that the gas is still present.",
                ),
            ],
            card: r#"{"title": "The smell that switches itself off", "body": "Hydrogen sulfide is famous for smelling like rotten eggs. The strange part: at high concentrations it quickly deadens the sense of smell, so people may not notice that the gas is still present.", "sources": ["s1", "s2"]}"#,
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

fn write_task(out: &mut String, molecule: &str, snippets: &[(String, String)], ask: &str) {
    let _ = writeln!(out, "Molecule: {molecule}\nSnippets:");
    for (i, (label, text)) in snippets.iter().enumerate() {
        let _ = writeln!(out, "[s{}] ({label}) {text}", i + 1);
    }
    out.push_str(ask);
}

fn labelled(snippets: &[&Snippet]) -> Vec<(String, String)> {
    snippets
        .iter()
        .map(|s| (source_label(s), s.text.clone()))
        .collect()
}

/// Build the prompt for one card. Snippets are renumbered `s1..sN` in the
/// order given; callers map ids back with the same order. After a rejected
/// attempt, pass `retry` with the validator's problems for the stricter prompt.
pub fn build(
    kind: CardKind,
    molecule: &str,
    snippets: &[&Snippet],
    retry: Option<&[String]>,
) -> Prompt {
    let ex = example(kind);
    let ex_snippets: Vec<_> = ex
        .snippets
        .iter()
        .map(|(label, text)| (label.to_string(), text.to_string()))
        .collect();
    let ask = format!("Write the \"{}\" card.", kind.label());

    let mut system = String::from(RULES);
    let _ = write!(system, "\n\n{}", brief(kind));
    if kind.can_skip() {
        let _ = write!(system, " {SKIP_RULE}");
    }
    system.push_str("\n\nExample\n");
    write_task(&mut system, ex.molecule, &ex_snippets, &ask);
    let _ = write!(system, "\n{}", ex.card);
    if let Some(problems) = retry {
        let _ = write!(system, "\n\n{RETRY_RULE}");
        if !problems.is_empty() {
            let _ = write!(system, " Problems found: {}.", problems.join("; "));
        }
    }

    let mut user = String::new();
    write_task(&mut user, molecule, &labelled(snippets), &ask);

    Prompt { system, user }
}

/// JSON schema for the model's reply, for constrained decoding. Source ids are
/// limited to the `snippet_count` snippets the prompt lists.
pub fn output_schema(kind: CardKind, snippet_count: usize) -> Value {
    let ids: Vec<_> = (1..=snippet_count).map(|n| format!("s{n}")).collect();
    let card = json!({
        "type": "object",
        "properties": {
            "title": { "type": "string" },
            "body": { "type": "string" },
            "sources": {
                "type": "array",
                "items": { "type": "string", "enum": ids },
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

/// For card kinds written in two steps, how many snippets the first step may
/// pick. Weird and Where get the most snippets and, in M0.6, merged facts
/// across them most often; writing from only the picked ones leaves less to mix up.
pub fn pick_limit(kind: CardKind) -> Option<usize> {
    match kind {
        CardKind::Where => Some(3),
        CardKind::Weird => Some(2),
        CardKind::Origin | CardKind::Danger => None,
    }
}

fn pick_brief(kind: CardKind) -> &'static str {
    match kind {
        CardKind::Where => {
            "Pick the snippets that best show where a person meets this molecule in everyday \
             life: foods, products, medicines, nature. Prefer concrete, familiar examples."
        }
        _ => {
            "Pick the snippet with the single most surprising, specific fact, the kind that \
             makes a reader say \"wait, really?\". Add a second only if the fact needs it."
        }
    }
}

/// First step of a two-step card: choose the best snippets by id.
pub fn build_pick(kind: CardKind, molecule: &str, snippets: &[&Snippet]) -> Prompt {
    let limit = pick_limit(kind).unwrap_or(1);
    let system = format!(
        "You choose source snippets for a fact card in Valence, an app that makes chemistry fun.\n\n\
         {} Pick at most {limit}.\n\n\
         Reply with one JSON object and nothing else: {{\"pick\": [\"s1\"]}}",
        pick_brief(kind)
    );
    let mut user = String::new();
    let ask = format!("Pick snippets for the \"{}\" card.", kind.label());
    write_task(&mut user, molecule, &labelled(snippets), &ask);
    Prompt { system, user }
}

/// Schema for the pick step: 1 to `pick_limit` ids from the listed snippets.
/// llguidance doesn't support `uniqueItems`; `parse_pick` drops repeats.
pub fn pick_schema(kind: CardKind, snippet_count: usize) -> Value {
    let ids: Vec<_> = (1..=snippet_count).map(|n| format!("s{n}")).collect();
    json!({
        "type": "object",
        "properties": {
            "pick": {
                "type": "array",
                "items": { "type": "string", "enum": ids },
                "minItems": 1,
                "maxItems": pick_limit(kind).unwrap_or(1)
            }
        },
        "required": ["pick"],
        "additionalProperties": false
    })
}

/// Zero-based indices of the picked snippets, in the order the model gave them.
/// `None` if the reply isn't a valid pick.
pub fn parse_pick(output: &str, snippet_count: usize) -> Option<Vec<usize>> {
    let value: Value = serde_json::from_str(output.trim()).ok()?;
    let mut picked = Vec::new();
    for id in value.get("pick")?.as_array()? {
        let n: usize = id.as_str()?.strip_prefix('s')?.parse().ok()?;
        if !(1..=snippet_count).contains(&n) {
            return None;
        }
        if !picked.contains(&(n - 1)) {
            picked.push(n - 1);
        }
    }
    (!picked.is_empty()).then_some(picked)
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
        let p = build(CardKind::Weird, "capsaicin", &[&a, &b], None);
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
            build(CardKind::Origin, "x", &[&s], None)
                .system
                .contains(SKIP_RULE)
        );
        assert!(
            !build(CardKind::Danger, "x", &[&s], None)
                .system
                .contains(SKIP_RULE)
        );
        assert!(
            !build(CardKind::Where, "x", &[&s], None)
                .system
                .contains(RETRY_RULE)
        );
        let problems = ["body has 1 sentences, needs 2 to 4".to_string()];
        let retry = build(CardKind::Where, "x", &[&s], Some(&problems)).system;
        assert!(retry.contains(RETRY_RULE));
        assert!(retry.ends_with("Problems found: body has 1 sentences, needs 2 to 4."));
    }

    #[test]
    fn examples_pass_the_validator() {
        for kind in CardKind::ALL {
            let ex = example(kind);
            let card: CardDraft = serde_json::from_str(ex.card).expect("example card parses");
            let snippets: Vec<_> = ex
                .snippets
                .iter()
                .map(|(label, text)| {
                    let (source, section) = label.split_once(", ").unwrap();
                    Snippet {
                        id: String::new(),
                        source: source.to_lowercase(),
                        section: section.into(),
                        url: String::new(),
                        origin: source.into(),
                        text: text.to_string(),
                    }
                })
                .collect();
            let refs: Vec<_> = snippets.iter().collect();
            assert_eq!(
                crate::validate::validate(&card, ex.molecule, &refs),
                Ok(()),
                "{kind:?} example"
            );
        }
    }

    #[test]
    fn pick_step_prompt_schema_and_parse() {
        let a = snippet("History", "First extracted in 1816.");
        let b = snippet("Natural function", "Birds do not feel it.");
        let p = build_pick(CardKind::Weird, "capsaicin", &[&a, &b]);
        assert!(p.system.contains("Pick at most 2."));
        assert!(
            p.user
                .ends_with("Pick snippets for the \"Weird fact\" card.")
        );
        assert_eq!(
            pick_schema(CardKind::Where, 5)["properties"]["pick"]["maxItems"],
            3
        );

        assert_eq!(
            parse_pick(r#"{"pick": ["s2", "s1", "s2"]}"#, 2),
            Some(vec![1, 0])
        );
        assert_eq!(parse_pick(r#"{"pick": ["s3"]}"#, 2), None);
        assert_eq!(parse_pick(r#"{"pick": []}"#, 2), None);
        assert_eq!(pick_limit(CardKind::Origin), None);
    }

    #[test]
    fn schema_allows_skip_only_for_skippable_kinds() {
        assert!(output_schema(CardKind::Weird, 3).get("anyOf").is_some());
        let danger = output_schema(CardKind::Danger, 3);
        assert!(danger.get("anyOf").is_none());
        assert_eq!(
            danger["properties"]["sources"]["items"]["enum"],
            json!(["s1", "s2", "s3"])
        );
    }
}

//! Claim check: a second model pass that asks, claim by claim, whether the
//! cited snippets state what the card says. Catches the paraphrased and merged
//! claims the string-matching validator can't see (M0.6d).

use std::fmt::Write;

use serde_json::{Value, json};

use crate::card::{CardDraft, Snippet};
use crate::prompt::Prompt;
use crate::validate::sentences;

/// How the model reports each claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
    /// Only a verdict per claim.
    Verdict,
    /// The snippet words that state the claim, then the verdict.
    Quote,
}

const RULES: &str = "\
You check fact cards for Valence, an app that makes chemistry fun, against the snippets they cite.

For each numbered claim, decide whether the snippets state it.
A claim is supported only if every part of it is stated in the snippets: the same subject, \
numbers, names, places and strength.
It is not supported if it:
- adds a fact, example, cause or consequence the snippets don't give
- joins facts the snippets give about different things
- changes who did what, or what a number or date refers to
- states as fact what the snippets only suggest, or drops a hedge like may or possibly
Playful wording is fine when the meaning is unchanged. A title is a hook: it is supported \
if its point is stated in the snippets.";

/// The claims to check: the title, then each body sentence.
pub fn claims(card: &CardDraft) -> Vec<&str> {
    let mut out = vec![card.title.trim()];
    out.extend(sentences(&card.body));
    out
}

pub fn build(style: Style, molecule: &str, card: &CardDraft, snippets: &[&Snippet]) -> Prompt {
    let mut system = String::from(RULES);
    system.push_str("\n\nReply with one JSON object and nothing else:\n");
    system.push_str(match style {
        Style::Verdict => r#"{"claims": [{"supported": true}, {"supported": false}]}"#,
        Style::Quote => {
            r#"{"claims": [{"quote": "the snippet words that state it", "supported": true}, {"quote": "", "supported": false}]}"#
        }
    });
    system.push_str("\nOne entry per claim, in order.");
    if style == Style::Quote {
        system.push_str(
            " quote: the shortest snippet passage that states the claim, or \"\" if none does.",
        );
    }

    let mut user = format!("Molecule: {molecule}\nSnippets:\n");
    for (i, s) in snippets.iter().enumerate() {
        let _ = writeln!(user, "[s{}] {}", i + 1, s.text);
    }
    user.push_str("\nClaims:\n");
    for (i, c) in claims(card).iter().enumerate() {
        let _ = writeln!(user, "{}. {c}", i + 1);
    }
    user.push_str("\nCheck each claim.");
    Prompt { system, user }
}

/// Schema with exactly one entry per claim.
pub fn schema(style: Style, claim_count: usize) -> Value {
    let entry = match style {
        Style::Verdict => json!({
            "type": "object",
            "properties": { "supported": { "type": "boolean" } },
            "required": ["supported"],
            "additionalProperties": false
        }),
        Style::Quote => json!({
            "type": "object",
            "properties": {
                "quote": { "type": "string", "maxLength": 200 },
                "supported": { "type": "boolean" }
            },
            "required": ["quote", "supported"],
            "additionalProperties": false
        }),
    };
    json!({
        "type": "object",
        "properties": {
            "claims": {
                "type": "array",
                "items": entry,
                "minItems": claim_count,
                "maxItems": claim_count
            }
        },
        "required": ["claims"],
        "additionalProperties": false
    })
}

/// One verdict per claim, in order. `None` if the reply doesn't have exactly
/// `claim_count` verdicts.
pub fn parse(output: &str, claim_count: usize) -> Option<Vec<bool>> {
    let value: Value = serde_json::from_str(output.trim()).ok()?;
    let verdicts: Option<Vec<bool>> = value
        .get("claims")?
        .as_array()?
        .iter()
        .map(|c| c.get("supported")?.as_bool())
        .collect();
    verdicts.filter(|v| v.len() == claim_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn card() -> CardDraft {
        CardDraft {
            title: "Birds don't feel the burn".into(),
            body: "Birds do not feel capsaicin. Mammals do.".into(),
            sources: vec!["s1".into()],
        }
    }

    #[test]
    fn numbers_title_and_sentences_as_claims() {
        let s = Snippet {
            id: String::new(),
            source: "wikipedia".into(),
            section: "Natural function".into(),
            url: String::new(),
            origin: String::new(),
            text: "Birds are insensitive to capsaicin.".into(),
        };
        let p = build(Style::Quote, "capsaicin", &card(), &[&s]);
        assert!(p.user.contains("[s1] Birds are insensitive to capsaicin."));
        assert!(p.user.contains(
            "1. Birds don't feel the burn\n2. Birds do not feel capsaicin.\n3. Mammals do."
        ));
        assert!(p.system.contains("quote:"));
        assert!(
            !build(Style::Verdict, "capsaicin", &card(), &[&s])
                .system
                .contains("quote:")
        );
        assert_eq!(
            schema(Style::Verdict, 3)["properties"]["claims"]["maxItems"],
            3
        );
    }

    #[test]
    fn parses_only_complete_replies() {
        let reply = r#"{"claims": [{"supported": true}, {"quote": "", "supported": false}]}"#;
        assert_eq!(parse(reply, 2), Some(vec![true, false]));
        assert_eq!(parse(reply, 3), None);
        assert_eq!(parse(r#"{"claims": [{"quote": "x"}]}"#, 1), None);
    }
}

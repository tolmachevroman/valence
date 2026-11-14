//! Cards made of source sentences (M3.7). The model never writes a card's
//! text: it chooses up to three numbered sentences from the snippets and
//! writes a short title, and the card shows those sentences exactly as the
//! sources give them. The title may only use words from the chosen sentences.
//!
//! The first review (docs/review/m3.7-findings.md) found that freely written
//! cards from a 4B model invent details in about half the cards, and that the
//! model can't catch this in its own writing. Choosing rules that out.

use std::collections::HashSet;
use std::fmt::Write;

use serde_json::{Value, json};

use crate::card::{CardDraft, CardKind, Snippet};
use crate::prompt::Prompt;
use crate::validate::sentences;

/// Sentences per card.
pub const MAX_CHOSEN: usize = 3;
/// Longer sentences aren't offered: a card is a few lines.
pub const MAX_SENTENCE_WORDS: usize = 45;
const MIN_SENTENCE_WORDS: usize = 4;
pub const MAX_CARD_WORDS: usize = 90;
pub const MAX_TITLE_WORDS: usize = 8;

/// Sentence openers that point back at the sentence before.
const LEANING: [&str; 11] = [
    "It", "Its", "This", "These", "They", "Their", "He", "She", "That", "Those", "Such",
];

/// Words a title may use freely: small words, and framing that states no fact.
const SMALL_WORDS: &[&str] = &[
    "a",
    "an",
    "the",
    "and",
    "or",
    "but",
    "of",
    "in",
    "on",
    "at",
    "to",
    "for",
    "from",
    "by",
    "with",
    "as",
    "is",
    "are",
    "was",
    "were",
    "be",
    "it",
    "its",
    "this",
    "that",
    "into",
    "than",
    "how",
    "why",
    "what",
    "when",
    "who",
    "where",
    "your",
    "you",
    "s",
    // Framing for the card kinds: where it's found, how it was discovered.
    "found",
    "find",
    "nature",
    "natural",
    "everyday",
    "life",
    "discovered",
    "discovery",
];

/// A sentence the model may choose, with its id: `s2.3` is snippet 2,
/// sentence 3 of that snippet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offered<'a> {
    pub id: String,
    /// Index into the snippets the card was given.
    pub snippet: usize,
    /// Position of the sentence in its snippet.
    pub position: usize,
    pub text: &'a str,
}

fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// The sentences of `snippets` a card may use, in order.
pub fn offer<'a>(snippets: &[&'a Snippet]) -> Vec<Offered<'a>> {
    let mut out = Vec::new();
    for (i, s) in snippets.iter().enumerate() {
        for (j, text) in sentences(&s.text).into_iter().enumerate() {
            let words = word_count(text);
            if (MIN_SENTENCE_WORDS..=MAX_SENTENCE_WORDS).contains(&words) {
                out.push(Offered {
                    id: format!("s{}.{}", i + 1, j + 1),
                    snippet: i,
                    position: j,
                    text,
                });
            }
        }
    }
    out
}

const RULES: &str = "\
You make fact cards for Valence, an app that makes chemistry fun, by choosing \
sentences from source snippets. The app shows the sentences you choose exactly as \
written; you never write the card's text yourself.

The snippets are split into numbered sentences like [s2.3].
- Choose 1 to 3 sentences that together make the best card. One is enough when it says it all.
- A sentence that starts with \"It\", \"This\" or \"They\" needs the sentence before it.
- Never choose sentences that describe how to make, extract or concentrate a substance.
- Write a title: a hook of at most 8 words, using only words from the sentences you \
choose, apart from small words like \"the\" or \"of\". Keep their meaning: no \
exaggeration, and keep hedges like \"may\".

Reply with one JSON object and nothing else:
{\"title\": \"...\", \"sentences\": [\"s1.2\", \"s3.1\"]}";

const SKIP_RULE: &str = "If no sentence fits this card, reply {\"skip\": true} instead.";

const RETRY_RULE: &str = "Your previous answer was rejected. Choose again.";

fn brief(kind: CardKind) -> &'static str {
    match kind {
        CardKind::Where => {
            "Card: Where you meet it. The everyday places this molecule shows up: foods, \
             products, medicines, nature. Prefer concrete, familiar examples."
        }
        CardKind::Origin => {
            "Card: Origin story. Who discovered or first made it, when, and how it happened."
        }
        CardKind::Weird => {
            "Card: Weird fact. The single most surprising detail, the kind that makes a \
             reader say \"wait, really?\"."
        }
        CardKind::Danger => "Card: Danger level.",
    }
}

struct Example {
    molecule: &'static str,
    snippets: &'static [(&'static str, &'static [&'static str])],
    reply: &'static str,
}

// Molecules outside the review set, so examples don't leak answers.
fn example(kind: CardKind) -> Example {
    match kind {
        CardKind::Where | CardKind::Danger => Example {
            molecule: "acetic acid",
            snippets: &[
                (
                    "PubChem, Uses",
                    &[
                        "Acetic acid is the main component of vinegar, apart from water.",
                        "It is also used to make cellulose acetate for photographic film.",
                    ],
                ),
                (
                    "Wikipedia, Introduction",
                    &["Vinegar is typically 4 to 8% acetic acid by volume."],
                ),
            ],
            reply: r#"{"title": "The main component of vinegar", "sentences": ["s1.1", "s2.1"]}"#,
        },
        CardKind::Origin => Example {
            molecule: "saccharin",
            snippets: &[
                (
                    "Wikipedia, History",
                    &[
                        "Saccharin was produced first in 1879 by Constantin Fahlberg, a chemist working on coal tar derivatives in Ira Remsen's laboratory at Johns Hopkins University.",
                        "Fahlberg noticed a sweet taste on his hands one evening, and connected this with the compound benzoic sulfimide on which he had been working.",
                    ],
                ),
                (
                    "Wikipedia, Introduction",
                    &["Saccharin is about 300 to 500 times as sweet as sucrose."],
                ),
            ],
            reply: r#"{"title": "A sweet taste on his hands", "sentences": ["s1.1", "s1.2"]}"#,
        },
        CardKind::Weird => Example {
            molecule: "hydrogen sulfide",
            snippets: &[
                (
                    "Wikipedia, Introduction",
                    &[
                        "Hydrogen sulfide is a colorless gas with the characteristic foul odor of rotten eggs.",
                    ],
                ),
                (
                    "Wikipedia, Safety",
                    &[
                        "Although the smell is easy to notice at first, at high concentrations hydrogen sulfide quickly deadens the sense of smell, so people may not notice that the gas is still present.",
                    ],
                ),
            ],
            reply: r#"{"title": "It quickly deadens the sense of smell", "sentences": ["s1.1", "s2.1"]}"#,
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

fn write_task<'a>(
    out: &mut String,
    molecule: &str,
    snippets: impl Iterator<Item = (String, Vec<(String, &'a str)>)>,
    ask: &str,
) {
    let _ = writeln!(out, "Molecule: {molecule}\nSnippets:");
    for (label, sentences) in snippets {
        let _ = writeln!(out, "({label})");
        for (id, text) in sentences {
            let _ = writeln!(out, "  [{id}] {text}");
        }
    }
    out.push_str(ask);
}

/// The prompt for one card. After a rejected attempt, pass the problems.
pub fn build(
    kind: CardKind,
    molecule: &str,
    snippets: &[&Snippet],
    offered: &[Offered],
    retry: Option<&[String]>,
) -> Prompt {
    let ask = format!("Choose sentences for the \"{}\" card.", kind.label());
    let mut system = String::from(RULES);
    let _ = write!(system, "\n\n{}", brief(kind));
    if kind.can_skip() {
        let _ = write!(system, " {SKIP_RULE}");
    }
    let ex = example(kind);
    system.push_str("\n\nExample\n");
    write_task(
        &mut system,
        ex.molecule,
        ex.snippets
            .iter()
            .enumerate()
            .map(|(i, (label, sentences))| {
                let numbered = sentences
                    .iter()
                    .enumerate()
                    .map(|(j, t)| (format!("s{}.{}", i + 1, j + 1), *t))
                    .collect();
                (label.to_string(), numbered)
            }),
        &ask,
    );
    let _ = write!(system, "\n{}", ex.reply);
    if let Some(problems) = retry {
        let _ = write!(system, "\n\n{RETRY_RULE}");
        if !problems.is_empty() {
            let _ = write!(system, " Problems found: {}.", problems.join("; "));
        }
    }

    let mut user = String::new();
    write_task(
        &mut user,
        molecule,
        snippets.iter().enumerate().filter_map(|(i, s)| {
            let numbered: Vec<_> = offered
                .iter()
                .filter(|o| o.snippet == i)
                .map(|o| (o.id.clone(), o.text))
                .collect();
            (!numbered.is_empty()).then(|| (source_label(s), numbered))
        }),
        &ask,
    );
    Prompt { system, user }
}

/// JSON schema for the reply: a title and 1 to `MAX_CHOSEN` offered ids, or
/// a skip where the card kind allows one.
pub fn schema(kind: CardKind, offered: &[Offered]) -> Value {
    let ids: Vec<&str> = offered.iter().map(|o| o.id.as_str()).collect();
    let card = json!({
        "type": "object",
        "properties": {
            "title": { "type": "string", "maxLength": 70 },
            "sentences": {
                "type": "array",
                "items": { "type": "string", "enum": ids },
                "minItems": 1,
                "maxItems": MAX_CHOSEN
            }
        },
        "required": ["title", "sentences"],
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

/// What the model chose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Chosen { title: String, ids: Vec<String> },
    Skip,
}

pub fn parse(output: &str) -> Result<Reply, String> {
    let v: Value =
        serde_json::from_str(output.trim()).map_err(|e| format!("unparseable reply: {e}"))?;
    if v.get("skip") == Some(&Value::Bool(true)) {
        return Ok(Reply::Skip);
    }
    let title = v["title"]
        .as_str()
        .ok_or("reply has no title")?
        .trim()
        .to_string();
    let ids = v["sentences"]
        .as_array()
        .ok_or("reply has no sentences")?
        .iter()
        .map(|id| {
            id.as_str()
                .map(str::to_string)
                .ok_or("sentence ids must be strings")
        })
        .collect::<Result<_, _>>()?;
    Ok(Reply::Chosen { title, ids })
}

/// Sentence ids found so far in a reply that may be cut off anywhere.
pub fn partial_ids(output: &str) -> Vec<String> {
    let Some(at) = output.find("\"sentences\"") else {
        return vec![];
    };
    let mut pieces: Vec<&str> = output[at + 11..].split('"').collect();
    // The last piece has no closing quote yet.
    pieces.pop();
    pieces
        .into_iter()
        .skip(1)
        .step_by(2)
        .filter(|s| {
            s.strip_prefix('s')
                .and_then(|n| n.split_once('.'))
                .is_some_and(|(a, b)| a.parse::<usize>().is_ok() && b.parse::<usize>().is_ok())
        })
        .map(str::to_string)
        .collect()
}

fn stem(word: &str) -> String {
    let w = word.to_lowercase().replace('’', "'");
    let w = w.strip_suffix("'s").unwrap_or(&w).to_string();
    for suffix in ["ies", "es", "s", "ed", "ing", "ly"] {
        if w.len() > 4 && w.ends_with(suffix) {
            return w[..w.len() - suffix.len()].to_string();
        }
    }
    w
}

fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '-'))
        .map(|w| w.trim_matches(|c| c == '\'' || c == '-'))
        .filter(|w| !w.is_empty())
}

/// Title words, other than small words, that the chosen sentences and the
/// molecule's name don't contain.
pub fn foreign_title_words(title: &str, chosen: &[&str], molecule: &str) -> Vec<String> {
    let known: HashSet<String> = chosen
        .iter()
        .flat_map(|s| words(s))
        .chain(words(molecule))
        .map(stem)
        .collect();
    words(title)
        .filter(|w| !SMALL_WORDS.contains(&w.to_lowercase().as_str()))
        .filter(|w| !known.contains(&stem(w)))
        .map(str::to_string)
        .collect()
}

/// The chosen sentences in source order, or the problems with the choice.
pub fn chosen<'o, 'a>(
    ids: &[String],
    offered: &'o [Offered<'a>],
) -> Result<Vec<&'o Offered<'a>>, Vec<String>> {
    let mut problems = Vec::new();
    let mut picked: Vec<&Offered> = Vec::new();
    for id in ids {
        match offered.iter().find(|o| &o.id == id) {
            Some(o) if !picked.contains(&o) => picked.push(o),
            Some(_) => {}
            None => problems.push(format!("{id} is not one of the numbered sentences")),
        }
    }
    // A sentence that leans on the one before ("It was…") brings that one
    // along, as long as it was offered; still the sources' own words.
    loop {
        let missing = picked.iter().find_map(|o| {
            let leans = LEANING.contains(&o.text.split_whitespace().next().unwrap_or_default());
            let before = offered
                .iter()
                .find(|p| p.snippet == o.snippet && p.position + 1 == o.position)?;
            (leans && !picked.contains(&before)).then_some(before)
        });
        match missing {
            Some(before) => picked.push(before),
            None => break,
        }
    }
    picked.sort_by_key(|o| (o.snippet, o.position));
    for (i, o) in picked.iter().enumerate() {
        let first = o.text.split_whitespace().next().unwrap_or_default();
        let after_its_own =
            i > 0 && picked[i - 1].snippet == o.snippet && picked[i - 1].position + 1 == o.position;
        if LEANING.contains(&first) && !after_its_own {
            problems.push(format!(
                "{} starts with \"{first}\" but the sentence before it was not chosen",
                o.id
            ));
        }
    }
    let total: usize = picked.iter().map(|o| word_count(o.text)).sum();
    if total > MAX_CARD_WORDS {
        problems.push(format!(
            "the chosen sentences have {total} words; keep it under {MAX_CARD_WORDS}"
        ));
    }
    if picked.is_empty() && problems.is_empty() {
        problems.push("no sentence chosen".into());
    }
    if problems.is_empty() {
        Ok(picked)
    } else {
        Err(problems)
    }
}

/// Problems with a title for the chosen sentences.
pub fn title_problems(title: &str, chosen: &[&Offered], molecule: &str) -> Vec<String> {
    let mut problems = Vec::new();
    let n = word_count(title);
    if n == 0 {
        problems.push("the title is empty".into());
    } else if n > MAX_TITLE_WORDS {
        problems.push(format!(
            "the title has {n} words; keep it to {MAX_TITLE_WORDS}"
        ));
    }
    let texts: Vec<&str> = chosen.iter().map(|o| o.text).collect();
    let foreign = foreign_title_words(title, &texts, molecule);
    if !foreign.is_empty() {
        problems.push(format!(
            "the title uses words not in the chosen sentences: {}",
            foreign.join(", ")
        ));
    }
    problems
}

/// The card from a checked choice: the sentences joined, citing their snippets.
pub fn draft(title: &str, chosen: &[&Offered]) -> CardDraft {
    let mut sources: Vec<String> = Vec::new();
    for o in chosen {
        let id = format!("s{}", o.snippet + 1);
        if !sources.contains(&id) {
            sources.push(id);
        }
    }
    CardDraft {
        title: title.to_string(),
        body: chosen.iter().map(|o| o.text).collect::<Vec<_>>().join(" "),
        sources,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(text: &str) -> Snippet {
        Snippet {
            id: String::new(),
            source: "wikipedia".into(),
            section: "History".into(),
            url: String::new(),
            origin: String::new(),
            text: text.into(),
        }
    }

    fn ids(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    const HISTORY: &str = "In 1819, the German chemist Friedlieb Ferdinand Runge isolated relatively pure caffeine for the first time. He called it Kaffebase, a base that exists in coffee. Short one.";

    #[test]
    fn offers_card_sized_sentences_numbered_by_snippet() {
        let a = s(HISTORY);
        let b = s(&format!("{} words.", "many ".repeat(50)));
        let offered = offer(&[&a, &b]);
        let ids: Vec<&str> = offered.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(ids, ["s1.1", "s1.2"], "too short and too long are left out");
    }

    #[test]
    fn a_leaning_sentence_needs_the_one_before_it() {
        let a = s(HISTORY);
        let offered = offer(&[&a]);
        // The sentence before is brought along.
        let with_before = chosen(&ids(&["s1.2"]), &offered).unwrap();
        let got: Vec<&str> = with_before.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(got, ["s1.1", "s1.2"]);
        // Unless it wasn't offered (here: the snippet's first sentence leans).
        let leaning_first =
            s("It was isolated by Runge in Germany in 1819. Runge called it Kaffebase.");
        let offered_lf = offer(&[&leaning_first]);
        let err = chosen(&ids(&["s1.1"]), &offered_lf).unwrap_err();
        assert!(err[0].contains("starts with \"It\""), "{err:?}");
        // Chosen out of order: put back in source order, and then fine.
        let ok = chosen(&ids(&["s1.2", "s1.1"]), &offered).unwrap();
        assert_eq!(ok[0].id, "s1.1");
        assert!(chosen(&ids(&["s9.1"]), &offered).is_err());
    }

    #[test]
    fn titles_use_only_words_from_the_chosen_sentences() {
        let a = s(HISTORY);
        let offered = offer(&[&a]);
        let picked = chosen(&ids(&["s1.1", "s1.2"]), &offered).unwrap();
        assert!(title_problems("Runge isolated it first, in 1819", &picked, "caffeine").is_empty());
        assert!(title_problems("Caffeine's coffee base", &picked, "caffeine").is_empty());
        // Framing words are fine; facts from elsewhere are not.
        let p = title_problems("Discovered in a German kitchen", &picked, "caffeine");
        assert!(p[0].ends_with(": kitchen"), "{p:?}");
    }

    #[test]
    fn replies_and_partial_replies() {
        assert_eq!(
            parse(r#"{"title": "T", "sentences": ["s1.1", "s2.3"]}"#),
            Ok(Reply::Chosen {
                title: "T".into(),
                ids: ids(&["s1.1", "s2.3"])
            })
        );
        assert_eq!(parse(r#"{"skip": true}"#), Ok(Reply::Skip));
        assert_eq!(
            partial_ids(r#"{"title": "s1.1 x", "sentences": ["s1.1", "s2"#),
            ["s1.1"]
        );
        assert!(partial_ids(r#"{"title": "Ru"#).is_empty());
    }

    #[test]
    fn the_card_is_the_sentences_verbatim() {
        let a = s(HISTORY);
        let offered = offer(&[&a]);
        let picked = chosen(&ids(&["s1.1", "s1.2"]), &offered).unwrap();
        let d = draft("Runge isolated it first", &picked);
        assert_eq!(
            d.body,
            "In 1819, the German chemist Friedlieb Ferdinand Runge isolated relatively pure caffeine for the first time. He called it Kaffebase, a base that exists in coffee."
        );
        assert_eq!(d.sources, ["s1"]);
    }

    #[test]
    fn the_prompt_lists_numbered_sentences_and_the_schema_only_those_ids() {
        let a = s(HISTORY);
        let snippets = [&a];
        let offered = offer(&snippets);
        let p = build(CardKind::Origin, "caffeine", &snippets, &offered, None);
        assert!(p.user.contains("  [s1.2] He called it Kaffebase"));
        assert!(!p.user.contains("Short one"));
        let schema = schema(CardKind::Origin, &offered);
        let ids = &schema["anyOf"][0]["properties"]["sentences"]["items"]["enum"];
        assert_eq!(ids, &json!(["s1.1", "s1.2"]));
    }
}

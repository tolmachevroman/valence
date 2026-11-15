//! Cards made of source sentences (M3.7). The model never writes a card's
//! text: it chooses up to three numbered sentences from the snippets, then a
//! title from the excerpts of those sentences this module allows, and the
//! card shows the sentences exactly as the sources give them.
//!
//! The first review (docs/review/m3.7-findings.md) found that freely written
//! cards from a 4B model invent details in about half the cards, and that the
//! model can't catch this in its own writing. Choosing rules that out.

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
const LEANING: &[&str] = &[
    "It",
    "Its",
    "This",
    "These",
    "They",
    "Their",
    "He",
    "She",
    "His",
    "Her",
    "That",
    "Those",
    "Such",
    "However",
    "Thus",
    "Therefore",
    "Hence",
    "Also",
    "Moreover",
    "Furthermore",
    "Additionally",
    "Instead",
    "Consequently",
    "Nevertheless",
    "Nonetheless",
    "Similarly",
    "Likewise",
    "Otherwise",
    "Meanwhile",
    "Subsequently",
    "Both",
    "Then",
    "Results",
];

/// Words a sentence's meaning can hang on; a title copied from it must keep them.
const QUALIFIERS: &[&str] = &[
    "may",
    "might",
    "can",
    "could",
    "probably",
    "possibly",
    "presumably",
    "reportedly",
    "likely",
    "perhaps",
    "suggest",
    "suggests",
    "suggested",
    "thought",
    "believed",
    "associated",
    "claimed",
    "according",
    "not",
    "no",
    "never",
    "once",
    "formerly",
];

/// Whether a sentence leans on the one before: it opens with a pronoun or a
/// connective, or points back with "these", "those" or "such".
fn leans(text: &str) -> bool {
    let first = text
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_end_matches([',', ';', ':']);
    if LEANING.contains(&first) {
        return true;
    }
    let lower: Vec<String> = tokens(text);
    lower.iter().enumerate().any(|(i, w)| {
        w == "these" || w == "those" || (w == "such" && lower.get(i + 1).is_none_or(|n| n != "as"))
    })
}

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

/// The sentences of `snippets` a card may use, in order. A sentence that
/// appears in two snippets is offered once, and one that leans on the sentence
/// before it only if that one is offered too.
pub fn offer<'a>(snippets: &[&'a Snippet]) -> Vec<Offered<'a>> {
    let mut out: Vec<Offered> = Vec::new();
    for (i, s) in snippets.iter().enumerate() {
        for (j, text) in sentences(&s.text).into_iter().enumerate() {
            let words = word_count(text);
            let repeat = out.iter().any(|o| tokens(o.text) == tokens(text));
            let stranded =
                leans(text) && !out.iter().any(|o| o.snippet == i && o.position + 1 == j);
            if (MIN_SENTENCE_WORDS..=MAX_SENTENCE_WORDS).contains(&words) && !repeat && !stranded {
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
- Choose sentences about this molecule, and name it: at least one sentence must.
- A sentence that starts with \"It\", \"This\" or \"However\" needs the sentence before it.
- Never choose sentences that describe how to make, extract or concentrate a substance.

Reply with one JSON object and nothing else:
{\"sentences\": [\"s1.2\", \"s3.1\"]}";

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
            reply: r#"{"sentences": ["s1.1", "s2.1"]}"#,
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
            reply: r#"{"sentences": ["s1.1", "s1.2"]}"#,
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
            reply: r#"{"sentences": ["s1.1", "s2.1"]}"#,
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

/// JSON schema for the reply: 1 to `MAX_CHOSEN` offered ids, or a skip where
/// the card kind allows one.
pub fn schema(kind: CardKind, offered: &[Offered]) -> Value {
    let ids: Vec<&str> = offered.iter().map(|o| o.id.as_str()).collect();
    let card = json!({
        "type": "object",
        "properties": {
            "sentences": {
                "type": "array",
                "items": { "type": "string", "enum": ids },
                "minItems": 1,
                "maxItems": MAX_CHOSEN
            }
        },
        "required": ["sentences"],
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
    Chosen(Vec<String>),
    Skip,
}

pub fn parse(output: &str) -> Result<Reply, String> {
    let v: Value =
        serde_json::from_str(output.trim()).map_err(|e| format!("unparseable reply: {e}"))?;
    if v.get("skip") == Some(&Value::Bool(true)) {
        return Ok(Reply::Skip);
    }
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
    Ok(Reply::Chosen(ids))
}

/// Words a title may not start or end on.
const EDGE_WORDS: &[&str] = &[
    "a", "an", "the", "and", "or", "but", "of", "in", "on", "at", "to", "for", "by", "with", "as",
    "is", "are", "was", "were", "be", "that", "which", "from", "its", "their",
];
/// More titles than this make the schema slow to compile; the rest are dropped.
const MAX_TITLES: usize = 1000;

/// The titles a card may have: 2 to `MAX_TITLE_WORDS` consecutive words of
/// one chosen sentence, not starting or ending on a small word, with every
/// hedge and negation of that sentence. Each passes `title_problems`.
pub fn titles(chosen: &[&Offered]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for o in chosen {
        let words: Vec<&str> = o.text.split_whitespace().collect();
        let qualifiers: Vec<String> = tokens(o.text)
            .into_iter()
            .filter(|w| QUALIFIERS.contains(&w.as_str()))
            .collect();
        for n in 2..=MAX_TITLE_WORDS {
            for run in words.windows(n) {
                let text = run
                    .join(" ")
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string();
                let t = tokens(&text);
                let edge = |w: Option<&String>| w.is_none_or(|w| EDGE_WORDS.contains(&w.as_str()));
                let balanced = text.matches('(').count() == text.matches(')').count()
                    && text.matches('"').count() % 2 == 0;
                if t.len() == n
                    && !edge(t.first())
                    && !edge(t.last())
                    && balanced
                    && qualifiers.iter().all(|q| t.contains(q))
                    && !out.contains(&text)
                {
                    out.push(text);
                }
            }
        }
    }
    out.truncate(MAX_TITLES);
    out
}

/// The prompt for the title step: the card's sentences, and the brief.
pub fn build_title(kind: CardKind, molecule: &str, chosen: &[&Offered]) -> Prompt {
    let system = format!(
        "You choose titles for fact cards in Valence, an app that makes chemistry fun. \
         The title is a short phrase copied from the card: pick the most striking one, \
         the part that makes a reader curious. {}\n\n\
         Reply with one JSON object and nothing else: {{\"title\": \"...\"}}",
        brief(kind)
    );
    let mut user = format!("Molecule: {molecule}\nCard:\n");
    for o in chosen {
        let _ = writeln!(user, "{}", o.text);
    }
    user.push_str("Choose the title.");
    Prompt { system, user }
}

/// Schema for the title step: one of `titles`.
pub fn title_schema(titles: &[String]) -> Value {
    json!({
        "type": "object",
        "properties": { "title": { "type": "string", "enum": titles } },
        "required": ["title"],
        "additionalProperties": false
    })
}

/// The chosen title, if the reply holds one of `titles`.
pub fn parse_title(output: &str, titles: &[String]) -> Option<String> {
    let v: Value = serde_json::from_str(output.trim()).ok()?;
    let title = v["title"].as_str()?;
    titles.iter().find(|t| *t == title).cloned()
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

/// Lowercase words, for comparing text regardless of case and punctuation.
fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !(c.is_alphanumeric() || c == '\'' || c == '’' || c == '-'))
        .map(|w| {
            w.trim_matches(|c| c == '\'' || c == '’' || c == '-')
                .to_lowercase()
        })
        .filter(|w| !w.is_empty())
        .collect()
}

/// The molecule's names to look for in a card: each name, and each without a
/// short stereo prefix ("L-Ascorbic Acid" → "ascorbic acid").
fn name_forms(names: &[&str]) -> Vec<String> {
    let mut out = Vec::new();
    for name in names {
        let lower = name.to_lowercase();
        // "L-Ascorbic Acid", "2,4,6-Trinitrotoluene".
        if let Some((prefix, rest)) = lower.split_once('-')
            && (prefix.chars().count() <= 4
                || prefix.chars().all(|c| c.is_ascii_digit() || c == ','))
            && rest.chars().count() >= 3
        {
            out.push(rest.to_string());
        }
        // "Fullerenes" names a card about buckminsterfullerene.
        if let Some(singular) = lower.strip_suffix('s')
            && singular.chars().count() >= 5
        {
            out.push(singular.to_string());
        }
        out.push(lower);
    }
    out.retain(|n| n.chars().count() >= 3);
    out
}

/// The chosen sentences in source order, or the problems with the choice.
/// `names` are the molecule's title and synonyms; the card must use one.
pub fn chosen<'o, 'a>(
    ids: &[String],
    offered: &'o [Offered<'a>],
    names: &[&str],
) -> Result<Vec<&'o Offered<'a>>, Vec<String>> {
    let mut problems = Vec::new();
    let mut picked: Vec<&Offered> = Vec::new();
    let mut too_long = 0;
    for id in ids {
        let Some(o) = offered.iter().find(|o| &o.id == id) else {
            problems.push(format!("{id} is not one of the numbered sentences"));
            continue;
        };
        // A sentence that leans on the one before ("It was…") brings that
        // one along; still the sources' own words.
        let mut chain = vec![o];
        while let Some(first) = chain.last().filter(|c| leans(c.text))
            && let Some(before) = offered
                .iter()
                .find(|p| p.snippet == first.snippet && p.position + 1 == first.position)
        {
            chain.push(before);
        }
        chain.retain(|c| !picked.contains(c));
        // Picks are taken in the model's order while the card stays short.
        let words = |v: &[&Offered]| v.iter().map(|o| word_count(o.text)).sum::<usize>();
        if words(&picked) + words(&chain) <= MAX_CARD_WORDS {
            picked.extend(chain);
        } else {
            too_long += 1;
        }
    }
    if picked.is_empty() && too_long > 0 {
        problems.push(format!(
            "the chosen sentences are too long together; keep the card under {MAX_CARD_WORDS} words"
        ));
    }
    picked.sort_by_key(|o| (o.snippet, o.position));
    for (i, o) in picked.iter().enumerate() {
        let after_its_own =
            i > 0 && picked[i - 1].snippet == o.snippet && picked[i - 1].position + 1 == o.position;
        if leans(o.text) && !after_its_own {
            problems.push(format!(
                "{} refers back to a sentence that can't be included",
                o.id
            ));
        }
    }
    let forms = name_forms(names);
    let text = picked
        .iter()
        .map(|o| o.text.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    if !picked.is_empty() && !forms.iter().any(|n| text.contains(n.as_str())) {
        problems.push(format!(
            "no chosen sentence names {}",
            names.first().unwrap_or(&"the molecule")
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

/// Problems with a title for the chosen sentences, a check on `titles`: it must be 2 to
/// `MAX_TITLE_WORDS` consecutive words of one of them, keeping that
/// sentence's hedges and negations.
pub fn title_problems(title: &str, chosen: &[&Offered]) -> Vec<String> {
    let t = tokens(title);
    if t.len() < 2 || t.len() > MAX_TITLE_WORDS {
        return vec![format!(
            "the title has {} words; copy 2 to {MAX_TITLE_WORDS}",
            t.len()
        )];
    }
    let source = chosen.iter().find(|o| {
        let words = tokens(o.text);
        words.windows(t.len()).any(|w| w == t.as_slice())
    });
    let Some(source) = source else {
        return vec!["the title must be consecutive words copied from one chosen sentence".into()];
    };
    let dropped: Vec<String> = tokens(source.text)
        .into_iter()
        .filter(|w| QUALIFIERS.contains(&w.as_str()) && !t.contains(w))
        .collect();
    if dropped.is_empty() {
        vec![]
    } else {
        vec![format!(
            "the title leaves out \"{}\" from {}",
            dropped.join("\", \""),
            source.id
        )]
    }
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

    const NAMES: &[&str] = &["caffeine"];
    const HISTORY: &str = "In 1819, the German chemist Friedlieb Ferdinand Runge isolated relatively pure caffeine for the first time. He called it Kaffebase, a base that exists in coffee. Short one.";

    #[test]
    fn offers_card_sized_sentences_once() {
        let a = s(HISTORY);
        let b = s(&format!("{} words.", "many ".repeat(50)));
        let c = s("He called it Kaffebase, a base that exists in coffee.");
        let offered = offer(&[&a, &b, &c]);
        let ids: Vec<&str> = offered.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(
            ids,
            ["s1.1", "s1.2"],
            "too short, too long and repeats are left out"
        );
    }

    #[test]
    fn a_leaning_sentence_brings_the_one_before_it() {
        let a = s(HISTORY);
        let offered = offer(&[&a]);
        let with_before = chosen(&ids(&["s1.2"]), &offered, NAMES).unwrap();
        let got: Vec<&str> = with_before.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(got, ["s1.1", "s1.2"]);
        // A sentence leaning on nothing offered isn't offered itself.
        let first =
            s("It was isolated by Runge in Germany in 1819. Runge called caffeine Kaffebase.");
        let offered = offer(&[&first]);
        let got: Vec<&str> = offered.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(got, ["s1.2"]);
        assert!(chosen(&ids(&["s9.1"]), &offered, NAMES).is_err());

        for leaning in [
            "However, the tests were almost four times more likely to fail.",
            "Results were published in the Chinese Medical Journal in 1979.",
            "Although caffeine was discovered in these fish, it is found elsewhere.",
            "The Quechua used the bark to treat such shivering.",
        ] {
            assert!(leans(leaning), "{leaning}");
        }
        assert!(!leans("Caffeine is found in foods such as tea and coffee."));
    }

    #[test]
    fn picks_are_kept_in_order_while_the_card_is_short() {
        let long = |n: u32| format!("Caffeine {} end {n}.", "word ".repeat(40));
        let a = s(&format!("{} {} {}", long(1), long(2), long(3)));
        let offered = offer(&[&a]);
        let picked = chosen(&ids(&["s1.3", "s1.1", "s1.2"]), &offered, NAMES).unwrap();
        let got: Vec<&str> = picked.iter().map(|o| o.id.as_str()).collect();
        assert_eq!(got, ["s1.1", "s1.3"], "the third no longer fits");
    }

    #[test]
    fn the_card_must_name_its_molecule() {
        let a = s("Acetanilide was the first aniline derivative found to relieve pain in 1886.");
        let offered = offer(&[&a]);
        let err = chosen(&ids(&["s1.1"]), &offered, &["acetaminophen", "paracetamol"]).unwrap_err();
        assert_eq!(err, ["no chosen sentence names acetaminophen"]);
        let v = s("Ascorbic acid is found in citrus fruits and many green vegetables.");
        let offered = offer(&[&v]);
        assert!(chosen(&ids(&["s1.1"]), &offered, &["L-Ascorbic Acid"]).is_ok());
    }

    #[test]
    fn titles_are_copied_and_keep_their_hedges() {
        let a = s(
            "Carvone was probably not isolated as a pure compound until 1849. Caffeine helps premature babies gain weight in neonatal care.",
        );
        let offered = offer(&[&a]);
        let all = chosen(&ids(&["s1.1", "s1.2"]), &offered, &["carvone"]).unwrap();
        assert!(title_problems("premature babies gain weight", &all).is_empty());
        assert!(title_problems("Premature Babies, Gain Weight!", &all).is_empty());
        assert_eq!(
            title_problems("isolated as a pure compound", &all),
            ["the title leaves out \"probably\", \"not\" from s1.1"]
        );
        assert!(title_problems("probably not isolated as a pure compound", &all).is_empty());
        assert_eq!(
            title_problems("babies gain weight with caffeine", &all),
            ["the title must be consecutive words copied from one chosen sentence"]
        );
        assert!(!title_problems("Carvone", &all).is_empty(), "too short");
    }

    #[test]
    fn allowed_titles_are_clean_excerpts_that_keep_hedges() {
        let a = s(
            "Carvone was probably not isolated as a pure compound until 1849. Caffeine helps premature babies gain weight in neonatal care.",
        );
        let offered = offer(&[&a]);
        let all = chosen(&ids(&["s1.1", "s1.2"]), &offered, &["carvone"]).unwrap();
        let t = titles(&all);
        assert!(t.contains(&"premature babies gain weight".to_string()));
        assert!(t.contains(&"probably not isolated".to_string()));
        assert!(
            !t.iter().any(|t| t.starts_with("isolated")),
            "drops the hedge"
        );
        assert!(
            !t.iter()
                .any(|t| t.ends_with(" in") || t.starts_with("the "))
        );
        for title in &t {
            assert!(title_problems(title, &all).is_empty(), "{title}");
        }
        assert_eq!(
            parse_title(r#"{"title": "premature babies gain weight"}"#, &t).as_deref(),
            Some("premature babies gain weight")
        );
        assert_eq!(parse_title(r#"{"title": "invented"}"#, &t), None);
        assert!(title_schema(&t)["properties"]["title"]["enum"].is_array());
    }

    #[test]
    fn names_match_without_locants_or_plurals() {
        let forms = name_forms(&["2,4,6-Trinitrotoluene", "Fullerenes"]);
        assert!(forms.contains(&"trinitrotoluene".to_string()));
        assert!(forms.contains(&"fullerene".to_string()));
    }

    #[test]
    fn replies_and_partial_replies() {
        assert_eq!(
            parse(r#"{"sentences": ["s1.1", "s2.3"]}"#),
            Ok(Reply::Chosen(ids(&["s1.1", "s2.3"])))
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
        let picked = chosen(&ids(&["s1.1", "s1.2"]), &offered, NAMES).unwrap();
        let d = draft("isolated relatively pure caffeine", &picked);
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

    /// Every example reply obeys the rules it teaches.
    #[test]
    fn examples_follow_the_rules() {
        for kind in [CardKind::Where, CardKind::Origin, CardKind::Weird] {
            let ex = example(kind);
            let owned: Vec<Snippet> = ex.snippets.iter().map(|(_, t)| s(&t.join(" "))).collect();
            let refs: Vec<&Snippet> = owned.iter().collect();
            let offered = offer(&refs);
            let Ok(Reply::Chosen(ids)) = parse(ex.reply) else {
                panic!("{kind:?}")
            };
            let picked = chosen(&ids, &offered, &[ex.molecule]).unwrap();
            assert!(!titles(&picked).is_empty(), "{kind:?}");
        }
    }
}

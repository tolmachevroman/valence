//! Parses the model's reply and checks a card against the snippets it cites.
//!
//! The checks are deliberately simple string matching (PRD: "simple normalized
//! string matching"): every number, code and proper name in the body must
//! appear in a cited snippet. They catch invented years, people and figures,
//! not paraphrased claims that introduce no new names or numbers.

use std::collections::HashSet;
use std::fmt;
use std::sync::LazyLock;

use regex::Regex;

use crate::card::{CardDraft, Snippet};

pub const MIN_SENTENCES: usize = 2;
pub const MAX_SENTENCES: usize = 4;
/// Body must be under this many words.
pub const MAX_WORDS: usize = 80;
pub const MAX_TITLE_WORDS: usize = 8;

/// What the model answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reply {
    Card(CardDraft),
    /// The model said the snippets have nothing for this card.
    Skip,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError(pub String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unparseable reply: {}", self.0)
    }
}

impl std::error::Error for ParseError {}

/// Parse a reply, tolerating code fences or chatter around the JSON object.
pub fn parse(output: &str) -> Result<Reply, ParseError> {
    let (Some(start), Some(end)) = (output.find('{'), output.rfind('}')) else {
        return Err(ParseError("no JSON object".into()));
    };
    if end < start {
        return Err(ParseError("no JSON object".into()));
    }
    let json = &output[start..=end];
    let value: serde_json::Value =
        serde_json::from_str(json).map_err(|e| ParseError(e.to_string()))?;
    if value.get("skip") == Some(&serde_json::Value::Bool(true)) {
        return Ok(Reply::Skip);
    }
    serde_json::from_value(value)
        .map(Reply::Card)
        .map_err(|e| ParseError(e.to_string()))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rejection {
    NoSources,
    UnknownSource(String),
    UnsupportedNumber(String),
    UnsupportedCode(String),
    UnsupportedName(String),
    Sentences(usize),
    TooLong(usize),
    EmptyTitle,
    TitleTooLong(usize),
}

impl fmt::Display for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSources => write!(f, "cites no sources"),
            Self::UnknownSource(id) => write!(f, "cites unknown source {id}"),
            Self::UnsupportedNumber(n) => write!(f, "number {n} is not in the cited snippets"),
            Self::UnsupportedCode(c) => write!(f, "{c} is not in the cited snippets"),
            Self::UnsupportedName(n) => write!(f, "name {n} is not in the cited snippets"),
            Self::Sentences(n) => write!(
                f,
                "body has {n} sentences, needs {MIN_SENTENCES} to {MAX_SENTENCES}"
            ),
            Self::TooLong(n) => write!(f, "body has {n} words, must be under {MAX_WORDS}"),
            Self::EmptyTitle => write!(f, "title is empty"),
            Self::TitleTooLong(n) => {
                write!(f, "title has {n} words, at most {MAX_TITLE_WORDS} allowed")
            }
        }
    }
}

/// Common words that start a sentence capitalized without being names. Covers
/// contractions and casual openers the system word list may lack.
#[rustfmt::skip]
const SENTENCE_STARTERS: &[&str] = &[
    "a", "about", "after", "all", "also", "although", "an", "and", "around", "as", "at", "because",
    "before", "both", "but", "by", "despite", "during", "each", "even", "every", "for", "from",
    "he", "her", "here", "his", "how", "however", "if", "imagine", "in", "instead", "interestingly",
    "it", "its", "just", "later", "like", "many", "most", "next", "no", "not", "now", "odd",
    "oddly", "of", "on", "once", "one", "only", "or", "our", "over", "remarkably", "she", "since",
    "so", "some", "still", "strangely", "surprisingly", "that", "the", "their", "then", "there",
    "these", "they", "this", "those", "though", "thanks", "to", "today", "turns", "under", "unlike",
    "we", "what", "when", "where", "which", "while", "who", "why", "with", "without", "yet", "you",
    "your",
    // Seen in M0.5 ("Plus, ...", "Did you know ...") and their kin.
    "plus", "did", "do", "does", "can", "could", "would", "will", "is", "are", "was", "were", "has",
    "have", "had", "ever", "meet", "picture", "talk", "think", "guess", "wow", "well", "yes", "sure",
    "first", "best", "fun", "chances", "spoiler",
];

static NUMBER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d{1,3}(?:,\d{3})+(?:\.\d+)?|\d+(?:\.\d+)?").unwrap());
static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\p{L}\p{N}]+(?:['’][\p{L}]+)?").unwrap());
static ORDINAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)(?:st|nd|rd|th)$").unwrap());
// A sentence ends at . ! ? (plus closing quotes or brackets) before whitespace
// and an uppercase letter or digit, or at the end of the text.
static SENTENCE_END: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r#"[.!?]["'’”)\]]*(?:\s+[\p{Lu}\d]|\s*$)"#).unwrap());

fn canonical_number(raw: &str) -> String {
    let n = raw.replace(',', "");
    match n.split_once('.') {
        Some((int, frac)) => {
            let frac = frac.trim_end_matches('0');
            if frac.is_empty() {
                int.to_string()
            } else {
                format!("{int}.{frac}")
            }
        }
        None => n,
    }
}

/// Numbers that stand alone: not part of a code like H302 or C8H8.
fn numbers(text: &str) -> Vec<String> {
    NUMBER
        .find_iter(text)
        .filter(|m| {
            let before = text[..m.start()].chars().next_back();
            let after = text[m.end()..].chars().next();
            !before.is_some_and(char::is_alphabetic) && !after.is_some_and(char::is_alphabetic)
        })
        .map(|m| canonical_number(m.as_str()))
        .collect()
}

/// The word without a possessive or contraction: "Remsen's" → "Remsen", "It's" → "It".
fn base_word(word: &str) -> &str {
    word.split(['\'', '’']).next().unwrap_or(word)
}

/// Lowercase entries of the macOS system word list (public domain, always
/// present on macOS): ordinary words. Names are listed capitalized there, so
/// they stay out. Empty if the file is missing.
static COMMON_WORDS: LazyLock<HashSet<String>> = LazyLock::new(|| {
    std::fs::read_to_string("/usr/share/dict/words")
        .map(|list| {
            list.lines()
                .filter(|w| w.chars().next().is_some_and(char::is_lowercase))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
});

/// Whether a capitalized word opening a sentence is just an ordinary word.
fn is_ordinary_starter(word: &str) -> bool {
    let lower = base_word(word).to_lowercase();
    SENTENCE_STARTERS.contains(&lower.as_str()) || COMMON_WORDS.contains(&lower)
}

/// Everything a card may draw on: snippet text plus the source labels the model sees.
struct Vocabulary {
    words: HashSet<String>,
    numbers: HashSet<String>,
}

impl Vocabulary {
    fn new(molecule: &str, snippets: &[&Snippet]) -> Self {
        let mut words = HashSet::new();
        let mut nums = HashSet::new();
        for s in snippets {
            for text in [s.text.as_str(), s.section.as_str(), s.origin.as_str()] {
                for w in WORD.find_iter(text) {
                    words.insert(base_word(w.as_str()).to_lowercase());
                }
                nums.extend(numbers(text));
            }
            words.insert(
                match s.source.as_str() {
                    "pubchem_view" => "pubchem",
                    other => other,
                }
                .to_string(),
            );
        }
        // A card may always name its own molecule, even when a snippet says "the compound".
        for w in WORD.find_iter(molecule) {
            words.insert(w.as_str().to_lowercase());
        }
        Self {
            words,
            numbers: nums,
        }
    }

    /// Case-insensitive, ignoring possessives and a plural "s" or "es".
    fn has_word(&self, word: &str) -> bool {
        let w = base_word(word).to_lowercase();
        let singular = [w.strip_suffix("es"), w.strip_suffix('s')];
        self.words.contains(&w)
            || singular
                .into_iter()
                .flatten()
                .any(|s| self.words.contains(s))
    }
}

fn sentence_count(body: &str) -> usize {
    SENTENCE_END.find_iter(body.trim()).count()
}

/// Check numbers and alphanumeric codes in `text`.
fn check_numbers_and_codes(text: &str, vocab: &Vocabulary, out: &mut Vec<Rejection>) {
    for n in numbers(text) {
        if !vocab.numbers.contains(&n) {
            out.push(Rejection::UnsupportedNumber(n));
        }
    }
    for w in WORD.find_iter(text) {
        let w = base_word(w.as_str());
        let has_digit = w.chars().any(|c| c.is_ascii_digit());
        let has_letter = w.chars().any(char::is_alphabetic);
        if !(has_digit && has_letter) {
            continue;
        }
        if let Some(n) = ORDINAL.captures(w) {
            if !vocab.numbers.contains(&n[1]) && !vocab.has_word(w) {
                out.push(Rejection::UnsupportedNumber(w.to_string()));
            }
        } else if !vocab.has_word(w) {
            out.push(Rejection::UnsupportedCode(w.to_string()));
        }
    }
}

/// Check capitalized words, treating sentence-initial common words as ordinary.
fn check_names(body: &str, vocab: &Vocabulary, out: &mut Vec<Rejection>) {
    let mut sentence_start = true;
    let mut last_end = 0;
    for m in WORD.find_iter(body) {
        let gap = &body[last_end..m.start()];
        if last_end > 0 && gap.contains(['.', '!', '?', ':']) {
            sentence_start = true;
        }
        last_end = m.end();

        let word = m.as_str();
        let starts_upper = word.chars().next().is_some_and(char::is_uppercase);
        let is_code = word.chars().any(|c| c.is_ascii_digit());
        let starter = sentence_start;
        sentence_start = false;
        if !starts_upper || is_code {
            continue;
        }
        if starter && is_ordinary_starter(word) {
            continue;
        }
        if !vocab.has_word(word) {
            out.push(Rejection::UnsupportedName(base_word(word).to_string()));
        }
    }
}

/// Validate a card about `molecule` against the snippets the prompt listed, in
/// prompt order (`s1` is `snippets[0]`). Returns every problem found.
pub fn validate(
    card: &CardDraft,
    molecule: &str,
    snippets: &[&Snippet],
) -> Result<(), Vec<Rejection>> {
    let mut out = Vec::new();

    let mut cited = Vec::new();
    for id in &card.sources {
        let index = id
            .strip_prefix('s')
            .and_then(|n| n.parse::<usize>().ok())
            .filter(|&n| (1..=snippets.len()).contains(&n));
        match index {
            Some(n) => cited.push(snippets[n - 1]),
            None => out.push(Rejection::UnknownSource(id.clone())),
        }
    }
    if card.sources.is_empty() {
        out.push(Rejection::NoSources);
    }

    if !cited.is_empty() {
        let vocab = Vocabulary::new(molecule, &cited);
        check_numbers_and_codes(&card.body, &vocab, &mut out);
        check_numbers_and_codes(&card.title, &vocab, &mut out);
        check_names(&card.body, &vocab, &mut out);
    }

    let sentences = sentence_count(&card.body);
    if !(MIN_SENTENCES..=MAX_SENTENCES).contains(&sentences) {
        out.push(Rejection::Sentences(sentences));
    }
    let words = card.body.split_whitespace().count();
    if words >= MAX_WORDS {
        out.push(Rejection::TooLong(words));
    }
    let title_words = card.title.split_whitespace().count();
    if title_words == 0 {
        out.push(Rejection::EmptyTitle);
    } else if title_words > MAX_TITLE_WORDS {
        out.push(Rejection::TitleTooLong(title_words));
    }

    out.dedup();
    if out.is_empty() { Ok(()) } else { Err(out) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snippet(section: &str, text: &str) -> Snippet {
        Snippet {
            id: String::new(),
            source: "wikipedia".into(),
            section: section.into(),
            url: String::new(),
            origin: "Wikipedia".into(),
            text: text.into(),
        }
    }

    fn capsaicin() -> Vec<Snippet> {
        vec![
            snippet(
                "Introduction",
                "Capsaicin is the main active component of chili peppers. It is an irritant for mammals, including humans.",
            ),
            snippet(
                "Natural function",
                "The seeds of Capsicum plants are dispersed predominantly by birds. In birds, the TRPV1 channel does not respond to capsaicin.",
            ),
            snippet(
                "History",
                "The compound was first extracted in impure form in 1816 by Christian Friedrich Bucholz (1770–1818).",
            ),
            snippet(
                "Capsaicinoids",
                "Capsaicin and dihydrocapsaicin (both 16.0 million SHU) are the most pungent capsaicinoids; capsaicin makes up 69% of the mix, and peppers reach 2,000,000 SHU.",
            ),
        ]
    }

    fn card(title: &str, body: &str, sources: &[&str]) -> CardDraft {
        CardDraft {
            title: title.into(),
            body: body.into(),
            sources: sources.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn check(c: &CardDraft) -> Result<(), Vec<Rejection>> {
        let s = capsaicin();
        let refs: Vec<_> = s.iter().collect();
        validate(c, "capsaicin", &refs)
    }

    #[test]
    fn accepts_a_grounded_card() {
        let c = card(
            "Birds Don't Feel the Burn",
            "Capsaicin makes chili peppers burn for mammals, including humans. Birds, which spread Capsicum seeds, feel nothing: their TRPV1 channel does not respond to it.",
            &["s1", "s2"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    #[test]
    fn accepts_names_years_and_formatted_numbers() {
        let c = card(
            "First extracted in 1816",
            "Christian Friedrich Bucholz first extracted capsaicin in impure form in 1816. At full strength, capsaicin scores 16 million SHU, and some peppers reach 2,000,000 SHU.",
            &["s3", "s4"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    #[test]
    fn rejects_missing_and_unknown_sources() {
        let body = "Capsaicin is an irritant. It burns.";
        assert!(
            check(&card("Hot", body, &[]))
                .unwrap_err()
                .contains(&Rejection::NoSources)
        );
        let err = check(&card("Hot", body, &["s9", "2"])).unwrap_err();
        assert!(err.contains(&Rejection::UnknownSource("s9".into())));
        assert!(err.contains(&Rejection::UnknownSource("2".into())));
    }

    #[test]
    fn rejects_an_invented_year_and_a_substring_number() {
        let err = check(&card(
            "Old news",
            "Capsaicin was first extracted in 1818 by Bucholz. That was 18 years before anyone cared.",
            &["s3"],
        ))
        .unwrap_err();
        // 1818 is in the snippet (Bucholz's death), so it passes; 18 is not a
        // standalone number there, only part of 1816 and 1818.
        assert!(!err.contains(&Rejection::UnsupportedNumber("1818".into())));
        assert!(err.contains(&Rejection::UnsupportedNumber("18".into())));

        let err = check(&card(
            "Older news",
            "Capsaicin was first extracted in 1820. Bucholz did it.",
            &["s3"],
        ))
        .unwrap_err();
        assert_eq!(err, vec![Rejection::UnsupportedNumber("1820".into())]);
    }

    #[test]
    fn rejects_facts_from_an_uncited_snippet() {
        // 1816 is in s3, but the card only cites s1.
        let err = check(&card(
            "Hot since 1816",
            "Capsaicin is an irritant for mammals. It was first extracted in 1816.",
            &["s1"],
        ))
        .unwrap_err();
        assert!(err.contains(&Rejection::UnsupportedNumber("1816".into())));
    }

    #[test]
    fn rejects_invented_names_anywhere_in_a_sentence() {
        let err = check(&card(
            "A Hungarian find",
            "Capsaicin was first extracted by Bucholz in 1816. Later, Hungarian scientist Albert Szent-Györgyi studied it.",
            &["s3"],
        ))
        .unwrap_err();
        assert!(err.contains(&Rejection::UnsupportedName("Hungarian".into())));
        assert!(err.contains(&Rejection::UnsupportedName("Albert".into())));

        let err = check(&card(
            "Hot",
            "Capsaicin is an irritant for mammals. Scoville measured it first.",
            &["s1"],
        ))
        .unwrap_err();
        assert_eq!(err, vec![Rejection::UnsupportedName("Scoville".into())]);
    }

    #[test]
    fn codes_are_matched_whole_not_as_numbers() {
        let ok = card(
            "A receptor called TRPV1",
            "Birds spread Capsicum seeds. Their TRPV1 channel does not respond to capsaicin.",
            &["s2"],
        );
        assert_eq!(check(&ok), Ok(()));
        let err = check(&card(
            "A receptor called TRPA1",
            "Birds spread Capsicum seeds. Their TRPA1 channel ignores capsaicin.",
            &["s2"],
        ))
        .unwrap_err();
        assert!(err.contains(&Rejection::UnsupportedCode("TRPA1".into())));
    }

    #[test]
    fn title_numbers_are_checked_but_title_case_is_fine() {
        let err = check(&card(
            "Hot Since 1700",
            "Capsaicin is an irritant for mammals. It is in chili peppers.",
            &["s1"],
        ))
        .unwrap_err();
        assert_eq!(err, vec![Rejection::UnsupportedNumber("1700".into())]);
    }

    #[test]
    fn enforces_sentence_and_word_limits() {
        let one = check(&card(
            "Hot",
            "Capsaicin is an irritant for mammals.",
            &["s1"],
        ))
        .unwrap_err();
        assert_eq!(one, vec![Rejection::Sentences(1)]);

        let five =
            "Capsaicin burns. It is in chili peppers. It irritates mammals. Humans too. Ouch.";
        assert!(
            check(&card("Hot", five, &["s1"]))
                .unwrap_err()
                .contains(&Rejection::Sentences(5))
        );

        let long = format!(
            "Capsaicin is an irritant for mammals. {}.",
            format!("It {}", "burns ".repeat(80)).trim()
        );
        assert!(matches!(
            check(&card("Hot", &long, &["s1"])).unwrap_err()[..],
            [Rejection::TooLong(_)]
        ));

        let err = check(&card(
            "A title that goes on far longer than eight words allow",
            "Capsaicin is an irritant for mammals. It burns.",
            &["s1"],
        ))
        .unwrap_err();
        assert_eq!(err, vec![Rejection::TitleTooLong(11)]);
    }

    #[test]
    fn decimals_and_abbreviations_do_not_split_sentences() {
        assert_eq!(
            sentence_count("It scores 16.0 million SHU in C. annuum peppers. That is a lot."),
            2
        );
    }

    /// Known gap: a claim with no new names or numbers passes. This is the
    /// Qwen3 card from M0.1 ("evolutionary adaptation" is not in the snippets).
    #[test]
    fn known_gap_paraphrased_claims_pass() {
        let c = card(
            "Birds Don't Feel the Burn",
            "Birds are immune to capsaicin because their TRPV1 channel does not respond to it. This evolutionary adaptation helps spread chili plants.",
            &["s2"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    /// Ordinary words opening a sentence pass via the system word list, and
    /// contractions and plurals match their base word (M0.5 false rejections).
    #[test]
    fn ordinary_openers_contractions_and_plurals_pass() {
        let c = card(
            "Hot",
            "Pure capsaicin is the main active component of chili peppers. It's an irritant for mammals. Beyond that, Capsicum seeds are dispersed predominantly by birds.",
            &["s1", "s2"],
        );
        assert_eq!(check(&c), Ok(()));
        let c = card(
            "Hot",
            "Capsaicin is an irritant for mammals. They're also in chili peppers, and Humans feel it.",
            &["s1"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    /// Known gap: a name that is also an ordinary word passes when it opens a
    /// sentence ("japan" is a lacquer in the word list).
    #[test]
    fn known_gap_name_that_is_also_a_word_opening_a_sentence() {
        let c = card(
            "Hot",
            "Capsaicin is an irritant for mammals. Japan grows chili peppers.",
            &["s1"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    #[test]
    fn molecule_name_is_always_allowed() {
        // s3 says "The compound", never "capsaicin".
        let c = card(
            "Older than you think",
            "Capsaicin was first extracted in impure form in 1816. Christian Friedrich Bucholz did it.",
            &["s3"],
        );
        assert_eq!(check(&c), Ok(()));
    }

    #[test]
    fn parses_cards_skips_and_fenced_replies() {
        let fenced = "```json\n{\"title\": \"T\", \"body\": \"B.\", \"sources\": [\"s1\"]}\n```";
        assert!(matches!(parse(fenced), Ok(Reply::Card(c)) if c.sources == ["s1"]));
        assert_eq!(parse(r#"{"skip": true}"#), Ok(Reply::Skip));
        assert!(parse("I cannot help with that.").is_err());
        assert!(parse(r#"{"title": "T"}"#).is_err());
    }
}

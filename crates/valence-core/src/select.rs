//! Picks the snippets each card type gets. No snippets means no card.

use std::sync::LazyLock;

use regex::Regex;

use crate::card::{CardKind, Snippet};
use crate::validate::sentences;

/// Rough cap on snippet text per card (about 1,500 tokens). Prompt processing
/// runs at about 300 tokens/s on an M1 Pro (M0.5), so every 1,000 characters
/// here adds close to a second before the card starts streaming.
pub const INPUT_BUDGET_CHARS: usize = 6_000;

const GHS: &str = "GHS Classification";

static WHERE_SECTION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)introduction|record description|occurrence|natural|\buses?\b|application|food|culinary|dietary|commercial|consumption|medical|sources",
    )
    .unwrap()
});
static ORIGIN_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)history|discovery|etymology|naming").unwrap());
// How it's made. "Biosynthesis" (how a plant makes it) and "Discovery and
// isolation" (history) are not.
static HOW_MADE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)\bsynthesis|production|manufactur|preparation|extraction|isolation|purification",
    )
    .unwrap()
});

/// Whether a section is about how the molecule is made. No card draws on these.
pub fn is_how_made(section: &str) -> bool {
    HOW_MADE.is_match(section) && !ORIGIN_SECTION.is_match(section)
}

// Sentences no card may draw on. Doses: lethal or safe amounts, intake levels.
// Procedures: reaction equations and method steps, which come close to how-to;
// biology ("synthesized by enzymes") and history ("synthesized by Sobrero in
// 1846") stay.
static DOSE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:LD50|LD₅₀|LC50|LDLo|TDLo|(?:lethal|fatal|toxic) dose|[mµμ]?g/kg|mg/d(?:ay)?|g/day|per kilogram)\b").unwrap()
});
static AMOUNT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\d\s*(?:mg|g|µg|μg|mcg|ml|mL|grams?|milligrams?)\b").unwrap());
static INTAKE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:dos(?:e|es|age|ing)|daily|intake|ingest\w*|administ\w*|oral(?:ly)?|taken|consum\w*|swallow\w*)\b").unwrap()
});
static PROCEDURE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\+[^.]*[→⟶⇌]|[→⟶⇌][^.]*\+|\breacts? with\b|\bcan react\b|\breaction of\b[^.]*\bwith\b|\bare prepared by\b|\bby (?:first )?(?:heating|cooling|evaporating|distilling|boiling|treating|reacting|mixing|dissolving|impaling)\b|\byields? (?:of )?\d|\bsteam distillation\b").unwrap()
});
static BIOLOGY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)\b(?:pathway|enzymes?|in the body|naturally|metaboli\w*|biosynthe\w*)\b")
        .unwrap()
});

fn is_dose(sentence: &str) -> bool {
    DOSE.is_match(sentence) || (AMOUNT.is_match(sentence) && INTAKE.is_match(sentence))
}

fn is_procedure(sentence: &str) -> bool {
    PROCEDURE.is_match(sentence) && !BIOLOGY.is_match(sentence)
}

/// The snippet without dose and procedure sentences, or `None` if nothing is
/// left. GHS snippets are kept whole: the Danger card is built from them as data.
pub fn scrub(snippet: &Snippet) -> Option<Snippet> {
    if snippet.section == GHS {
        return Some(snippet.clone());
    }
    let kept: Vec<_> = sentences(&snippet.text)
        .into_iter()
        .filter(|s| !is_dose(s) && !is_procedure(s))
        .collect();
    (!kept.is_empty()).then(|| Snippet {
        text: kept.join(" "),
        ..snippet.clone()
    })
}

fn is_wikipedia(s: &Snippet) -> bool {
    s.source == "wikipedia"
}

/// Selection order for Weird: unusual Wikipedia sections first, where the
/// surprising details tend to live, then the intro, then PubChem.
fn weird_rank(s: &Snippet) -> u8 {
    match (is_wikipedia(s), s.section.as_str()) {
        (true, "Introduction") => 1,
        (true, _) => 0,
        (false, "Record Description") => 2,
        (false, "Natural Pollution Sources") => 3,
        (false, "Toxicity Summary") => 4,
        (false, _) => 5,
    }
}

fn within_budget(snippets: Vec<&Snippet>) -> Vec<&Snippet> {
    let mut used = 0;
    snippets
        .into_iter()
        .take_while(|s| {
            used += s.text.len();
            used <= INPUT_BUDGET_CHARS
        })
        .collect()
}

/// Snippets for one card, in the order the prompt should list them.
pub fn select(kind: CardKind, snippets: &[Snippet]) -> Vec<&Snippet> {
    // Fetching already leaves how-made sections out; snippets cached by older
    // versions may not.
    let snippets = snippets.iter().filter(|s| !is_how_made(&s.section));
    match kind {
        CardKind::Where => within_budget(
            snippets
                .filter(|s| s.section != GHS && WHERE_SECTION.is_match(&s.section))
                .filter(|s| is_wikipedia(s) || s.section != "Toxicity Summary")
                .collect(),
        ),
        CardKind::Origin => within_budget(
            snippets
                .filter(|s| is_wikipedia(s) && ORIGIN_SECTION.is_match(&s.section))
                .collect(),
        ),
        CardKind::Danger => snippets.filter(|s| s.section == GHS).collect(),
        CardKind::Weird => {
            let mut picked: Vec<_> = snippets.filter(|s| s.section != GHS).collect();
            picked.sort_by_key(|s| weird_rank(s));
            within_budget(picked)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(source: &str, section: &str, text: &str) -> Snippet {
        Snippet {
            id: String::new(),
            source: source.into(),
            section: section.into(),
            url: String::new(),
            origin: String::new(),
            text: text.into(),
        }
    }

    fn sections(picked: &[&Snippet]) -> Vec<String> {
        picked.iter().map(|s| s.section.clone()).collect()
    }

    fn record() -> Vec<Snippet> {
        vec![
            s("pubchem_view", "Record Description", "A purine alkaloid."),
            s("wikipedia", "Introduction", "Found in coffee."),
            s("wikipedia", "History", "Isolated in 1819."),
            s("wikipedia", "Natural function", "Birds ignore it."),
            s("pubchem_view", "Uses", "Food flavoring."),
            s("pubchem_view", "Toxicity Summary", "Toxic at high doses."),
            s("pubchem_view", GHS, "H302: Harmful if swallowed"),
        ]
    }

    #[test]
    fn each_kind_gets_its_sections() {
        let r = record();
        assert_eq!(
            sections(&select(CardKind::Where, &r)),
            [
                "Record Description",
                "Introduction",
                "Natural function",
                "Uses"
            ]
        );
        assert_eq!(sections(&select(CardKind::Origin, &r)), ["History"]);
        assert_eq!(sections(&select(CardKind::Danger, &r)), [GHS]);
        assert_eq!(
            sections(&select(CardKind::Weird, &r)),
            [
                "History",
                "Natural function",
                "Introduction",
                "Record Description",
                "Toxicity Summary",
                "Uses"
            ]
        );
    }

    #[test]
    fn missing_sections_mean_no_card() {
        let r = vec![s("wikipedia", "Introduction", "A cube of carbon atoms.")];
        assert!(select(CardKind::Origin, &r).is_empty());
        assert!(select(CardKind::Danger, &r).is_empty());
        assert_eq!(select(CardKind::Weird, &r).len(), 1);
    }

    #[test]
    fn scrub_drops_doses_and_procedures_only() {
        let text = |t: &str| scrub(&s("wikipedia", "Toxicity", t)).map(|s| s.text);
        assert_eq!(
            text("The LD50 in mice is 47.2 mg/kg. Birds do not feel it."),
            Some("Birds do not feel it.".into())
        );
        assert_eq!(
            text("A typical dose of caffeine is around 70 to 100 mg per drink."),
            None
        );
        assert_eq!(
            text(
                "Becher obtained it by heating ethanol with sulfuric acid; he named it. It is a gas."
            ),
            Some("It is a gas.".into())
        );
        assert_eq!(text("C60 reacts with lithium in liquid ammonia."), None);
        for kept in [
            "It was synthesized by the Italian chemist Ascanio Sobrero in 1846.",
            "Serotonin is synthesized from tryptophan by a short metabolic pathway.",
            "It is produced by bacteria, especially cyanobacteria.",
            "Morphine is obtained from opium.",
            "A 100 g bar of dark chocolate holds a lot of it.",
        ] {
            assert_eq!(text(kept).as_deref(), Some(kept));
        }
        let ghs = s("pubchem_view", GHS, "H301: Toxic if swallowed");
        assert_eq!(scrub(&ghs), Some(ghs));
    }

    #[test]
    fn how_made_sections_reach_no_card() {
        for made in ["Synthesis", "Production", "Chemical synthesis", "Isolation"] {
            assert!(is_how_made(made), "{made}");
        }
        for kept in ["Biosynthesis", "Discovery and isolation", "History", "Uses"] {
            assert!(!is_how_made(kept), "{kept}");
        }
        let r = vec![
            s(
                "wikipedia",
                "Production",
                "Made in large vats from petroleum.",
            ),
            s("wikipedia", "Uses", "Used in cough drops."),
        ];
        for kind in [CardKind::Where, CardKind::Weird] {
            assert_eq!(sections(&select(kind, &r)), ["Uses"]);
        }
    }

    #[test]
    fn input_is_capped() {
        let long = "x".repeat(INPUT_BUDGET_CHARS / 2);
        let r: Vec<_> = (0..5).map(|_| s("wikipedia", "Trivia", &long)).collect();
        assert_eq!(select(CardKind::Weird, &r).len(), 2);
    }
}

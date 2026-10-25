//! Picks the snippets each card type gets. No snippets means no card.

use std::sync::LazyLock;

use regex::Regex;

use crate::card::{CardKind, Snippet};

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
    match kind {
        CardKind::Where => within_budget(
            snippets
                .iter()
                .filter(|s| s.section != GHS && WHERE_SECTION.is_match(&s.section))
                .filter(|s| is_wikipedia(s) || s.section != "Toxicity Summary")
                .collect(),
        ),
        CardKind::Origin => within_budget(
            snippets
                .iter()
                .filter(|s| is_wikipedia(s) && ORIGIN_SECTION.is_match(&s.section))
                .collect(),
        ),
        CardKind::Danger => snippets.iter().filter(|s| s.section == GHS).collect(),
        CardKind::Weird => {
            let mut picked: Vec<_> = snippets.iter().filter(|s| s.section != GHS).collect();
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
    fn input_is_capped() {
        let long = "x".repeat(INPUT_BUDGET_CHARS / 2);
        let r: Vec<_> = (0..5).map(|_| s("wikipedia", "Trivia", &long)).collect();
        assert_eq!(select(CardKind::Weird, &r).len(), 2);
    }
}

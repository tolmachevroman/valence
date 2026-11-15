//! Cleanup shared by every snippet source.

use std::sync::LazyLock;

use regex::Regex;

/// Shorter passages carry no fact worth a card.
pub const MIN_CHARS: usize = 40;
/// Longer ones are cut at the last sentence end before this.
pub const MAX_CHARS: usize = 700;

static SPACE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\s+").unwrap());
// Wikipedia's plain-text extract drops IPA, leaving "(, commonly )" or "()".
static EMPTY_PARENS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s*\(\s*[,;][^()]*\)|\s*\(\s*\)").unwrap());

// Database leftovers: citation markers "(1)", "(1-4)", "[3]", "(L1069)";
// HSDB's slashes around inserted words ("congestive /heart/ failure", whose
// words stay); a leading "CHEMICAL PROFILE:" label.
static CITATION: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\s*(?:\((?:\d{1,3}(?:\s*[-–,]\s*\d{1,3})*|[A-Z]\d{2,})\)|\[\d{1,3}(?:[-–,]\d{1,3})*\])",
    )
    .unwrap()
});
static SLASH_TAG: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(^|\s)/([A-Za-z][^/\s]{0,40})/").unwrap());
static LABEL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[A-Z][A-Z ]{5,}:\s*").unwrap());

/// Collapse whitespace, drop emptied parentheses and database leftovers, and
/// cut to `MAX_CHARS`. Running it again changes nothing.
pub fn clean(text: &str) -> String {
    let text = SPACE.replace_all(text, " ");
    let text = EMPTY_PARENS.replace_all(text.trim(), "");
    let text = CITATION.replace_all(&text, "");
    let text = SLASH_TAG.replace_all(&text, "$1$2");
    let text = LABEL.replace(text.trim(), "").into_owned();
    if text.chars().count() <= MAX_CHARS {
        return text;
    }
    let cut: String = text.chars().take(MAX_CHARS).collect();
    match cut.rfind(". ") {
        Some(end) if cut[..end].chars().count() > MIN_CHARS => cut[..=end].to_string(),
        _ => format!("{}…", cut.trim_end()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_space_and_empty_parentheses() {
        assert_eq!(
            clean("Caffeine (, commonly )  is\n a stimulant ()."),
            "Caffeine is a stimulant."
        );
    }

    #[test]
    fn drops_database_leftovers() {
        assert_eq!(
            clean(
                "CHEMICAL PROFILE: Used as a flavor (1-4). Treats congestive /heart/ failure [3]. See (L1069)."
            ),
            "Used as a flavor. Treats congestive heart failure. See."
        );
        // Years and real parentheses stay.
        let kept = "Isolated in (1819) by Runge (a German chemist), and/or by others.";
        assert_eq!(clean(kept), kept);
        assert_eq!(clean(&clean("Uses (2).")), "Uses.");
    }

    #[test]
    fn cuts_long_text_at_a_sentence_end() {
        let sentence = "Caffeine is found in coffee. ";
        let long = sentence.repeat(40);
        let cut = clean(&long);
        assert!(cut.chars().count() <= MAX_CHARS);
        assert!(cut.ends_with("coffee."));

        let unbroken = "x".repeat(MAX_CHARS + 10);
        assert_eq!(clean(&unbroken).chars().count(), MAX_CHARS + 1);
        assert!(clean(&unbroken).ends_with('…'));
    }
}

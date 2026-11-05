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

/// Collapse whitespace, drop emptied parentheses, and cut to `MAX_CHARS`.
pub fn clean(text: &str) -> String {
    let text = SPACE.replace_all(text, " ");
    let text = EMPTY_PARENS.replace_all(text.trim(), "").into_owned();
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

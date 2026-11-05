//! GHS hazard data and the Danger card built from it without the model.
//!
//! Hazard statements are already plain language ("Causes serious eye
//! irritation"), and M0.6 showed small models misreading them: report
//! percentages as probabilities, "Suspected of" as "may", "Not Classified" as
//! "safe", plus unprompted advice. So the Danger card is assembled from the
//! data directly.

use std::sync::LazyLock;

use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::card::CardDraft;

/// Statements reported by fewer than this share of notifiers are counted, not listed.
const MAJORITY_PERCENT: f32 = 50.0;
const MAX_LISTED: usize = 5;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HazardStatement {
    /// Such as `H302` or `H300+H310`.
    pub code: String,
    /// Such as "Harmful if swallowed".
    pub text: String,
    /// Share of notifiers reporting it, when the source gives one (ECHA does).
    pub percent: Option<f32>,
    /// The statement's own signal level, "Danger" or "Warning", when given.
    pub level: Option<String>,
}

/// Stored as `ghs_json` in the cache.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Ghs {
    /// "Danger" or "Warning"; `None` when not classified.
    pub signal: Option<String>,
    pub pictograms: Vec<String>,
    pub statements: Vec<HazardStatement>,
}

// Code, optional "**" footnote marker, optional report share, optional
// "(Deleted)" label, text, then the bracketed level and class.
static STATEMENT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(H\d{3}\w*(?:\+H\d{3}\w*)*)(?:\s*\*+)?(?:\s*\((\d+(?:\.\d+)?)%\))?:\s*(?:\(Deleted\)\s*)?([^\[]+?)\s*\[(Danger|Warning)?",
    )
    .unwrap()
});
static SIGNAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"Signal word:\s*(\w+)").unwrap());
static PICTOGRAMS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"Pictograms:\s*([^.]+)\.").unwrap());

impl Ghs {
    /// Parse a GHS snippet's text, as built by `pug_view::ghs` (and earlier by
    /// `scripts/fetch_spike_fixtures.py`).
    pub fn from_snippet_text(text: &str) -> Self {
        let mut statements: Vec<HazardStatement> = Vec::new();
        for c in STATEMENT.captures_iter(text) {
            if statements.iter().any(|s| s.code == c[1]) {
                continue;
            }
            statements.push(HazardStatement {
                code: c[1].to_string(),
                text: c[3].trim().to_string(),
                percent: c.get(2).and_then(|p| p.as_str().parse().ok()),
                level: c.get(4).map(|l| l.as_str().to_string()),
            });
        }
        let classified = !statements.is_empty();
        Self {
            signal: SIGNAL
                .captures(text)
                .filter(|_| classified)
                .map(|c| c[1].to_string()),
            pictograms: PICTOGRAMS
                .captures(text)
                .filter(|_| classified)
                .map(|c| c[1].split(',').map(|p| p.trim().to_string()).collect())
                .unwrap_or_default(),
            statements,
        }
    }

    /// Statements listed on the card: those most notifiers report, plus, when
    /// none of those carries the signal word's level, the ones that do, so the
    /// card explains its own signal (benzylpenicillin's "Danger" comes from a
    /// statement 48% report). The rest are only counted.
    fn listed(&self) -> (Vec<&HazardStatement>, Vec<&HazardStatement>) {
        let majority = |s: &HazardStatement| s.percent.is_none_or(|p| p >= MAJORITY_PERCENT);
        let at_signal = |s: &HazardStatement| s.level.is_some() && s.level == self.signal;
        let explained = self.statements.iter().any(|s| majority(s) && at_signal(s));
        self.statements
            .iter()
            .partition(|s| majority(s) || (!explained && at_signal(s)))
    }
}

fn lower_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

fn join_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{}, and {last}", rest.join(", ")),
    }
}

/// The Danger card text. The UI shows pictograms and statements verbatim next to it.
pub fn danger_card(molecule: &str, ghs: &Ghs) -> CardDraft {
    let name = {
        let mut chars = molecule.chars();
        chars
            .next()
            .map(|c| c.to_uppercase().chain(chars).collect::<String>())
            .unwrap_or_default()
    };
    let Some(signal) = &ghs.signal else {
        return CardDraft {
            title: "Not classified as hazardous".into(),
            body: format!(
                "{name} is not classified as hazardous under the official GHS system in the data PubChem lists. That is a regulatory label, not a promise of safety."
            ),
            sources: vec![],
        };
    };

    let (common, rare) = ghs.listed();
    let listed: Vec<String> = common
        .iter()
        .take(MAX_LISTED)
        .map(|s| lower_first(&s.text))
        .collect();
    let unlisted = common.len().saturating_sub(MAX_LISTED);

    let mut body = format!("{name} carries the official GHS signal word \"{signal}\".");
    if !listed.is_empty() {
        body.push_str(&format!(" Its hazard statements: {}.", join_list(&listed)));
    }
    if unlisted > 0 {
        body.push_str(&format!(" The official list has {unlisted} more."));
    }
    if !rare.is_empty() {
        let n = rare.len();
        body.push_str(&format!(
            " A minority of reports list {n} other hazard{}.",
            if n == 1 { "" } else { "s" }
        ));
    }
    CardDraft {
        title: format!("Official signal word: {signal}"),
        body,
        sources: vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPSAICIN: &str = "Signal word: Danger. Pictograms: Corrosive, Acute Toxic, Irritant. H301 (21.9%): Toxic if swallowed [Danger Acute toxicity, oral] H302 (78%): Harmful if swallowed [Warning Acute toxicity, oral] H315 (99.5%): Causes skin irritation [Warning Skin corrosion/irritation] H317 (21.5%): May cause an allergic skin reaction [Warning Sensitization, Skin] H318 (75.3%): Causes serious eye damage [Danger Serious eye damage/eye irritation]";
    const MENTHOL: &str = "Signal word: Warning. Pictograms: Irritant. Not Classified Reported as not meeting GHS hazard criteria by 1 of 1 companies.";

    #[test]
    fn parses_statements_signal_and_pictograms() {
        let g = Ghs::from_snippet_text(CAPSAICIN);
        assert_eq!(g.signal.as_deref(), Some("Danger"));
        assert_eq!(g.pictograms, ["Corrosive", "Acute Toxic", "Irritant"]);
        assert_eq!(g.statements.len(), 5);
        assert_eq!(g.statements[0].code, "H301");
        assert_eq!(g.statements[0].text, "Toxic if swallowed");
        assert_eq!(g.statements[0].percent, Some(21.9));
    }

    #[test]
    fn combined_codes_and_no_percentages() {
        let g = Ghs::from_snippet_text(
            "Signal word: Danger. H300+H310: Fatal if swallowed or in contact with skin [Danger Acute toxicity] H300+H310: Fatal if swallowed or in contact with skin [Danger Acute toxicity]",
        );
        assert_eq!(g.statements.len(), 1);
        assert_eq!(g.statements[0].code, "H300+H310");
        assert_eq!(g.statements[0].percent, None);
    }

    #[test]
    fn card_lists_majority_statements_and_counts_the_rest() {
        let card = danger_card("capsaicin", &Ghs::from_snippet_text(CAPSAICIN));
        assert_eq!(card.title, "Official signal word: Danger");
        assert_eq!(
            card.body,
            "Capsaicin carries the official GHS signal word \"Danger\". Its hazard statements: harmful if swallowed, causes skin irritation, and causes serious eye damage. A minority of reports list 2 other hazards."
        );
    }

    /// M0.6b review: the "**" footnote marker hid H373, and "(Deleted)" leaked into the card.
    #[test]
    fn footnote_markers_and_deleted_labels() {
        let g = Ghs::from_snippet_text(
            "Signal word: Danger. H201: (Deleted) Explosive; mass explosion hazard [Danger Explosives] H300: Fatal if swallowed [Danger Acute toxicity, oral] H373 **: May causes damage to organs through prolonged or repeated exposure [Warning Specific target organ toxicity, repeated exposure]",
        );
        let codes: Vec<_> = g.statements.iter().map(|s| s.code.as_str()).collect();
        assert_eq!(codes, ["H201", "H300", "H373"]);
        assert_eq!(g.statements[0].text, "Explosive; mass explosion hazard");
        assert_eq!(g.statements[2].level.as_deref(), Some("Warning"));
    }

    #[test]
    fn card_explains_its_signal_word_even_from_a_minority_report() {
        let g = Ghs::from_snippet_text(
            "Signal word: Danger. H317 (100%): May cause an allergic skin reaction [Warning Sensitization, Skin] H334 (48%): May cause allergy or asthma symptoms or breathing difficulties if inhaled [Danger Sensitization, respiratory] H400 (46.7%): Very toxic to aquatic life [Warning Hazardous to the aquatic environment, acute hazard]",
        );
        let card = danger_card("benzylpenicillin", &g);
        assert!(card.body.contains("may cause allergy or asthma symptoms"));
        assert!(
            card.body
                .ends_with("A minority of reports list 1 other hazard.")
        );
    }

    /// PubChem can show an Irritant pictogram next to "Not Classified"; with no
    /// hazard statements, the card must not claim a hazard or claim safety.
    #[test]
    fn not_classified_is_neither_hazard_nor_safety() {
        let g = Ghs::from_snippet_text(MENTHOL);
        assert_eq!(g, Ghs::default());
        let card = danger_card("menthol", &g);
        assert_eq!(card.title, "Not classified as hazardous");
        assert!(card.body.contains("not a promise of safety"));
        assert!(!card.body.to_lowercase().contains("irritat"));
    }
}

//! Card types, source snippets, and the card shape the model writes.

use serde::{Deserialize, Serialize};

/// The four POC card types (F4–F7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardKind {
    Where,
    Origin,
    Danger,
    Weird,
}

impl CardKind {
    pub const ALL: [CardKind; 4] = [Self::Where, Self::Origin, Self::Danger, Self::Weird];

    /// Value stored in `card.kind`.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Where => "where",
            Self::Origin => "origin",
            Self::Danger => "danger",
            Self::Weird => "weird",
        }
    }

    /// Heading shown on the card.
    pub fn label(self) -> &'static str {
        match self {
            Self::Where => "Where you meet it",
            Self::Origin => "Origin story",
            Self::Danger => "Danger level",
            Self::Weird => "Weird fact",
        }
    }

    /// Whether the model may decline when the snippets have nothing that fits.
    /// Where-you-meet-it and Danger are skipped before generation instead,
    /// when no relevant snippets exist.
    pub fn can_skip(self) -> bool {
        matches!(self, Self::Origin | Self::Weird)
    }
}

/// A fetched source passage. The model sees only these, never an open question.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Snippet {
    /// Stable id: the cache row id, or `s3`-style in spike fixtures. Prompts
    /// number their snippets `s1..sN` themselves.
    pub id: String,
    /// `pubchem_view` or `wikipedia`.
    pub source: String,
    /// Section name, such as `Uses` or `History`.
    pub section: String,
    pub url: String,
    /// Original data provider, such as `ChEBI` or `Wikipedia`.
    #[serde(default)]
    pub origin: String,
    pub text: String,
}

/// A card as the model writes it, before validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardDraft {
    pub title: String,
    pub body: String,
    pub sources: Vec<String>,
}

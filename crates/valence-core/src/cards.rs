//! The card pipeline: select → offer numbered sentences → the model chooses
//! sentences and writes a title → check → one retry with the problems → card
//! or nothing. The card's text is always the sources' own sentences
//! (`extract`); accuracy first.

use crate::cache::{Cache, NewCard, StoredCard, text_hash};
use crate::card::{CardDraft, CardKind, Snippet};
use crate::generate::{Generate, GenerateError, Request};
use crate::{extract, ghs, select};

/// Up to three sentence ids.
const CHOICE_TOKENS: usize = 40;
/// Lowered from 0.6 in M0.6b for faithfulness to the snippets.
pub const TEMPERATURE: f64 = 0.2;

/// A snippet a card cites.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// `Snippet::id`: the cache row id.
    pub snippet_id: String,
    /// "Wikipedia, History", "PubChem, Uses".
    pub label: String,
    pub url: String,
}

/// A card that passed every check.
#[derive(Debug, Clone, PartialEq)]
pub struct Card {
    pub kind: CardKind,
    pub title: String,
    pub body: String,
    pub sources: Vec<Source>,
    /// Danger only: what to show verbatim under the text.
    pub hazards: Option<Hazards>,
}

/// The official GHS data a Danger card lists: the statements most reports
/// agree on, plus any that explain the signal word (`Ghs::listed`).
#[derive(Debug, Clone, PartialEq)]
pub struct Hazards {
    pub pictograms: Vec<String>,
    pub statements: Vec<ghs::HazardStatement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Card(Card),
    /// The model said the snippets have nothing for this card.
    Skipped,
    /// Failed twice; the problems of the last attempt.
    Dropped(Vec<String>),
    /// No snippets for this card.
    NoInput,
}

/// What the pipeline is doing, for the UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The model is choosing sentences.
    Writing,
    /// The card so far, unchecked: the sentences chosen so far.
    Draft { body: String },
    /// The choice was rejected; choosing again with these problems listed.
    Retrying { problems: Vec<String> },
}

fn label(s: &Snippet) -> String {
    let source = if s.source == "wikipedia" {
        "Wikipedia"
    } else {
        "PubChem"
    };
    format!("{source}, {}", s.section)
}

/// Snippet `sN` of the ones the prompt listed.
fn cited<'a>(id: &str, listed: &[&'a Snippet]) -> Option<&'a Snippet> {
    let n: usize = id.strip_prefix('s')?.parse().ok()?;
    listed.get(n.checked_sub(1)?).copied()
}

fn finished(kind: CardKind, draft: CardDraft, listed: &[&Snippet]) -> Card {
    let sources = draft
        .sources
        .iter()
        .filter_map(|id| cited(id, listed))
        .map(|s| Source {
            snippet_id: s.id.clone(),
            label: label(s),
            url: s.url.clone(),
        })
        .collect();
    Card {
        kind,
        title: draft.title,
        body: draft.body,
        sources,
        hazards: None,
    }
}

/// Cards are cached per model and per pipeline version: bump this when a
/// change should rewrite cards already stored.
const PIPELINE: &str = "extract-1";

fn cache_key(model: &str) -> String {
    format!("{model} {PIPELINE}")
}

/// What the cache holds for one card kind.
#[derive(Debug, Clone, PartialEq)]
pub enum Cached {
    Card(Card),
    /// Written before from these same snippets, and came out empty.
    NoCard,
    /// Never written from these snippets: generate it.
    Missing,
}

/// One hash for the snippets a card kind gets, so a remembered empty card
/// is retried once any of them changes.
fn input_hash(kind: CardKind, snippets: &[Snippet]) -> String {
    let joined: String = select::select(kind, snippets)
        .iter()
        .map(|s| text_hash(&s.text))
        .collect();
    text_hash(&joined)
}

/// The cached `kind` card for a molecule, written by `model` (a file name).
pub fn cached(
    cache: &Cache,
    cid: u64,
    kind: CardKind,
    snippets: &[Snippet],
    model: &str,
) -> rusqlite::Result<Cached> {
    let model = &cache_key(model);
    let stored = cache.cards(cid, model)?;
    if let Some(card) = stored.into_iter().find(|c| c.kind == kind.as_str()) {
        return Ok(Cached::Card(from_stored(kind, card, snippets)));
    }
    let no_card = cache.no_card(cid, kind.as_str(), model)?;
    Ok(
        if no_card.as_deref() == Some(input_hash(kind, snippets).as_str()) {
            Cached::NoCard
        } else {
            Cached::Missing
        },
    )
}

fn from_stored(kind: CardKind, stored: StoredCard, snippets: &[Snippet]) -> Card {
    let sources = stored
        .snippet_ids
        .iter()
        .filter_map(|id| snippets.iter().find(|s| s.id == id.to_string()))
        .map(|s| Source {
            snippet_id: s.id.clone(),
            label: label(s),
            url: s.url.clone(),
        })
        .collect();
    Card {
        kind,
        title: stored.title,
        body: stored.body,
        sources,
        hazards: None,
    }
}

/// Store what generating a `kind` card came to. Errors are not stored (the
/// next open tries again), nor is missing input (nothing to try).
pub fn remember(
    cache: &mut Cache,
    cid: u64,
    kind: CardKind,
    snippets: &[Snippet],
    model: &str,
    outcome: &Outcome,
    now: u64,
) -> rusqlite::Result<()> {
    let model = &cache_key(model);
    match outcome {
        Outcome::Card(card) => {
            let ids: Vec<i64> = card
                .sources
                .iter()
                .filter_map(|s| s.snippet_id.parse().ok())
                .collect();
            cache.save_card(&NewCard {
                cid,
                kind: kind.as_str(),
                title: &card.title,
                body: &card.body,
                model,
                snippet_ids: &ids,
                created_at: now,
            })?;
        }
        Outcome::Skipped | Outcome::Dropped(_) => {
            cache.save_no_card(cid, kind.as_str(), model, &input_hash(kind, snippets), now)?;
        }
        Outcome::NoInput => {}
    }
    Ok(())
}

/// Card kinds with input, in generation order: Danger first (built from data,
/// instant), then the shortest prompt first, so the first card appears early.
pub fn order(snippets: &[Snippet]) -> Vec<CardKind> {
    let mut kinds: Vec<(CardKind, usize)> = CardKind::ALL
        .into_iter()
        .map(|k| {
            let picked = select::select(k, snippets);
            let chars: usize = picked.iter().map(|s| s.text.len()).sum();
            (k, if picked.is_empty() { 0 } else { chars.max(1) })
        })
        .filter(|(_, chars)| *chars > 0)
        .collect();
    kinds.sort_by_key(|&(k, chars)| (k != CardKind::Danger, chars));
    kinds.into_iter().map(|(k, _)| k).collect()
}

/// The Danger card, built from the GHS data without the model; `None` when
/// PubChem has no GHS section.
pub fn danger(molecule: &str, snippets: &[Snippet]) -> Option<Card> {
    let picked = select::select(CardKind::Danger, snippets);
    let data = ghs::Ghs::from_snippet_text(&picked.first()?.text);
    let draft = CardDraft {
        sources: vec!["s1".into()],
        ..ghs::danger_card(molecule, &data)
    };
    Some(Card {
        hazards: Some(Hazards {
            pictograms: data.pictograms.clone(),
            statements: data.listed().0.into_iter().cloned().collect(),
        }),
        ..finished(CardKind::Danger, draft, &picked)
    })
}

/// Whether `card` would have anything to work with: GHS data for Danger,
/// sentences to offer the model for the others.
pub fn has_input(
    kind: CardKind,
    molecule: &str,
    also_called: &[String],
    snippets: &[Snippet],
) -> bool {
    if kind == CardKind::Danger {
        return danger(molecule, snippets).is_some();
    }
    let names: Vec<&str> = std::iter::once(molecule)
        .chain(also_called.iter().map(String::as_str))
        .collect();
    !extract::offer(&select::select(kind, snippets), &names).is_empty()
}

/// Generate one card about `molecule` from its snippets (`extract`): the
/// model chooses sentences, and the choice is checked and retried once with
/// its problems. A choice that fails twice drops the card.
pub async fn card<G: Generate>(
    model: &G,
    kind: CardKind,
    molecule: &str,
    also_called: &[String],
    snippets: &[Snippet],
    on_event: &mut (dyn FnMut(Event) + Send),
) -> Result<Outcome, GenerateError> {
    if kind == CardKind::Danger {
        return Ok(danger(molecule, snippets).map_or(Outcome::NoInput, Outcome::Card));
    }
    let names: Vec<&str> = std::iter::once(molecule)
        .chain(also_called.iter().map(String::as_str))
        .collect();
    let picked = select::select(kind, snippets);
    let offered = extract::offer(&picked, &names);
    if offered.is_empty() {
        return Ok(Outcome::NoInput);
    }

    on_event(Event::Writing);
    let mut problems: Option<Vec<String>> = None;
    let mut chosen = None;
    for _ in 0..2 {
        if let Some(p) = &problems {
            on_event(Event::Retrying {
                problems: p.clone(),
            });
        }
        let request = Request {
            prompt: extract::build(kind, molecule, &picked, &offered, problems.as_deref()),
            schema: extract::schema(kind, &offered),
            max_tokens: CHOICE_TOKENS,
            temperature: TEMPERATURE,
        };
        let mut text = String::new();
        let mut shown = String::new();
        let g = model
            .generate(request, &mut |piece| {
                text.push_str(piece);
                let body = extract::partial_ids(&text)
                    .iter()
                    .filter_map(|id| offered.iter().find(|o| &o.id == id))
                    .map(|o| o.text)
                    .collect::<Vec<_>>()
                    .join(" ");
                if body != shown {
                    shown = body.clone();
                    on_event(Event::Draft { body });
                }
            })
            .await?;
        let ids = match extract::parse(&g.text) {
            Ok(extract::Reply::Skip) => return Ok(Outcome::Skipped),
            Ok(extract::Reply::Chosen(ids)) => ids,
            Err(e) => {
                problems = Some(vec![e]);
                continue;
            }
        };
        match extract::chosen(&ids, &offered, &names) {
            Ok(c) => {
                chosen = Some(c);
                break;
            }
            Err(p) => problems = Some(p),
        }
    }
    let Some(chosen) = chosen else {
        return Ok(Outcome::Dropped(problems.unwrap_or_default()));
    };

    Ok(Outcome::Card(finished(
        kind,
        extract::draft(&chosen),
        &picked,
    )))
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex;

    use super::*;
    use crate::generate::Generation;

    /// A model that answers from a script, in order, and records the prompts.
    struct Scripted {
        replies: Mutex<VecDeque<&'static str>>,
        asked: Mutex<Vec<String>>,
    }

    impl Scripted {
        fn new(replies: &[&'static str]) -> Self {
            Self {
                replies: Mutex::new(replies.iter().copied().collect()),
                asked: Mutex::default(),
            }
        }

        fn asked(&self) -> Vec<String> {
            self.asked.lock().unwrap().clone()
        }
    }

    impl Generate for Scripted {
        async fn generate(
            &self,
            request: Request,
            on_text: &mut (dyn FnMut(&str) + Send),
        ) -> Result<Generation, GenerateError> {
            self.asked.lock().unwrap().push(request.prompt.system);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("no scripted reply left");
            // Stream in small pieces, as a model would.
            for piece in reply.as_bytes().chunks(7) {
                on_text(std::str::from_utf8(piece).unwrap());
            }
            Ok(Generation {
                text: reply.into(),
                ..Generation::default()
            })
        }
    }

    fn s(id: &str, source: &str, section: &str, text: &str) -> Snippet {
        Snippet {
            id: id.into(),
            source: source.into(),
            section: section.into(),
            url: format!("https://example.org/{id}"),
            origin: String::new(),
            text: text.into(),
        }
    }

    fn capsaicin() -> Vec<Snippet> {
        vec![
            s(
                "11",
                "wikipedia",
                "History",
                "Capsaicin was first extracted in impure form in 1816 by Christian Friedrich Bucholz.",
            ),
            s(
                "12",
                "pubchem_view",
                "GHS Classification",
                "Signal: Danger\nH301: Toxic if swallowed [Danger Acute toxicity, oral]",
            ),
        ]
    }

    /// The History snippet has one sentence, offered as s1.1.
    const CHOICE: &str = r#"{"sentences": ["s1.1"]}"#;
    const HISTORY: &str =
        "Capsaicin was first extracted in impure form in 1816 by Christian Friedrich Bucholz.";

    async fn run(model: &Scripted, kind: CardKind) -> (Outcome, Vec<Event>) {
        let mut events = Vec::new();
        let outcome = card(model, kind, "capsaicin", &[], &capsaicin(), &mut |e| {
            events.push(e)
        })
        .await
        .unwrap();
        (outcome, events)
    }

    #[test]
    fn has_input_matches_what_card_would_offer() {
        let snippets = capsaicin();
        let input = |k| has_input(k, "capsaicin", &[], &snippets);
        // Weird draws on any snippet but the hazards.
        assert!(input(CardKind::Origin) && input(CardKind::Danger) && input(CardKind::Weird));
        assert!(!input(CardKind::Where));
        // A history sentence that never names the molecule offers nothing.
        let unnamed = [s(
            "1",
            "wikipedia",
            "History",
            "It was first extracted in 1816 by Bucholz.",
        )];
        assert!(!has_input(CardKind::Origin, "capsaicin", &[], &unnamed));
    }

    #[tokio::test]
    async fn a_card_is_the_chosen_sentences_verbatim() {
        let model = Scripted::new(&[CHOICE]);
        let (outcome, events) = run(&model, CardKind::Origin).await;
        let Outcome::Card(card) = outcome else {
            panic!("{outcome:?}")
        };
        assert_eq!((card.title.as_str(), card.body.as_str()), ("", HISTORY));
        assert_eq!(
            card.sources,
            [Source {
                snippet_id: "11".into(),
                label: "Wikipedia, History".into(),
                url: "https://example.org/11".into(),
            }]
        );
        assert_eq!(events.first(), Some(&Event::Writing));
        assert!(events.contains(&Event::Draft {
            body: HISTORY.into()
        }));
        assert_eq!(model.asked().len(), 1, "one request per card");
    }

    #[tokio::test]
    async fn a_bad_choice_is_retried_then_dropped() {
        let unknown = r#"{"sentences": ["s1.7"]}"#;
        let model = Scripted::new(&[unknown, unknown]);
        let (outcome, _) = run(&model, CardKind::Origin).await;
        assert_eq!(
            outcome,
            Outcome::Dropped(vec!["s1.7 is not one of the numbered sentences".into()])
        );
    }

    #[tokio::test]
    async fn skips_and_missing_input_make_no_card() {
        let model = Scripted::new(&[r#"{"skip": true}"#]);
        assert_eq!(run(&model, CardKind::Origin).await.0, Outcome::Skipped);
        let model = Scripted::new(&[]);
        assert_eq!(run(&model, CardKind::Where).await.0, Outcome::NoInput);
    }

    #[tokio::test]
    async fn danger_comes_from_data_without_the_model() {
        let model = Scripted::new(&[]);
        let (outcome, events) = run(&model, CardKind::Danger).await;
        let Outcome::Card(card) = outcome else {
            panic!("{outcome:?}")
        };
        assert_eq!(card.sources[0].label, "PubChem, GHS Classification");
        let hazards = card.hazards.unwrap();
        assert_eq!(hazards.statements[0].code, "H301");
        assert!(events.is_empty() && model.asked().is_empty());
    }

    #[tokio::test]
    async fn cards_are_cached_and_empty_ones_remembered_until_the_snippets_change() {
        use crate::cache::Molecule;
        use crate::pubchem::Compound;

        let molecule = |snippets| Molecule {
            compound: Compound {
                cid: 1548943,
                title: "Capsaicin".into(),
                formula: "C18H27NO3".into(),
                molar_mass: "305.4".into(),
                iupac_name: None,
                cas: None,
                synonyms: vec![],
            },
            sdf_3d: None,
            png_2d: None,
            ghs: None,
            snippets,
            fetched_at: 1,
        };
        let mut cache = Cache::in_memory().unwrap();
        cache.save(&molecule(capsaicin())).unwrap();
        let snippets = cache.molecule(1548943).unwrap().unwrap().snippets;
        let cached = |cache: &Cache, kind, snippets: &[Snippet]| {
            super::cached(cache, 1548943, kind, snippets, "qwen").unwrap()
        };
        assert_eq!(cached(&cache, CardKind::Origin, &snippets), Cached::Missing);

        let model = Scripted::new(&[CHOICE]);
        let outcome = card(
            &model,
            CardKind::Origin,
            "Capsaicin",
            &[],
            &snippets,
            &mut |_| {},
        )
        .await
        .unwrap();
        remember(
            &mut cache,
            1548943,
            CardKind::Origin,
            &snippets,
            "qwen",
            &outcome,
            2,
        )
        .unwrap();
        assert_eq!(cached(&cache, CardKind::Origin, &snippets), {
            let Outcome::Card(c) = outcome else { panic!() };
            Cached::Card(c)
        });

        remember(
            &mut cache,
            1548943,
            CardKind::Weird,
            &snippets,
            "qwen",
            &Outcome::Skipped,
            2,
        )
        .unwrap();
        assert_eq!(cached(&cache, CardKind::Weird, &snippets), Cached::NoCard);
        assert_eq!(
            super::cached(&cache, 1548943, CardKind::Weird, &snippets, "phi").unwrap(),
            Cached::Missing,
            "another model tries again"
        );

        // The History paragraph is edited upstream: the Origin card citing it
        // goes, and the empty Weird card is tried again.
        let mut edited = capsaicin();
        edited[0].text.push_str(" He named it capsicin.");
        cache.save(&molecule(edited)).unwrap();
        let snippets = cache.molecule(1548943).unwrap().unwrap().snippets;
        assert_eq!(cached(&cache, CardKind::Origin, &snippets), Cached::Missing);
        assert_eq!(cached(&cache, CardKind::Weird, &snippets), Cached::Missing);
    }

    #[test]
    fn order_puts_danger_first_then_shortest() {
        let mut snippets = capsaicin();
        snippets.push(s(
            "13",
            "pubchem_view",
            "Uses",
            &"Used in pepper spray. ".repeat(20),
        ));
        assert_eq!(
            order(&snippets),
            [
                CardKind::Danger,
                CardKind::Origin,
                CardKind::Where,
                CardKind::Weird
            ]
        );
        assert_eq!(order(&[]), []);
    }
}

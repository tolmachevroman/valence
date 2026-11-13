//! The card pipeline: select → (pick) → write → validate → claim check →
//! one retry with the problems → card or nothing. Accuracy first: a card that
//! fails twice is dropped, never shown.

use crate::card::{CardDraft, CardKind, Snippet};
use crate::generate::{Generate, GenerateError, Request};
use crate::validate::{self, Reply};
use crate::verify::{self, Style};
use crate::{ghs, prompt, select};

pub const MAX_NEW_TOKENS: usize = 300;
/// Lowered from 0.6 in M0.6b for faithfulness to the snippets.
pub const TEMPERATURE: f64 = 0.2;
const PICK_TOKENS: usize = 40;
const CHECK_TOKENS: usize = 100;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
    pub kind: CardKind,
    pub title: String,
    pub body: String,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Choosing snippets or writing the first attempt.
    Writing,
    /// The card so far, unchecked: fields as far as the model has written them.
    Draft { title: String, body: String },
    /// Validator passed; asking the model whether the snippets state each claim.
    Checking,
    /// The attempt failed; writing again with the problems listed.
    Retrying,
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
    }
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
    Some(finished(CardKind::Danger, draft, &picked))
}

/// Generate one card about `molecule` from its snippets.
pub async fn card<G: Generate>(
    model: &G,
    kind: CardKind,
    molecule: &str,
    snippets: &[Snippet],
    on_event: &mut (dyn FnMut(Event) + Send),
) -> Result<Outcome, GenerateError> {
    if kind == CardKind::Danger {
        return Ok(danger(molecule, snippets).map_or(Outcome::NoInput, Outcome::Card));
    }
    let mut picked = select::select(kind, snippets);
    if picked.is_empty() {
        return Ok(Outcome::NoInput);
    }

    on_event(Event::Writing);
    if prompt::pick_limit(kind).is_some() && picked.len() > 1 {
        let request = Request {
            prompt: prompt::build_pick(kind, molecule, &picked),
            schema: prompt::pick_schema(kind, picked.len()),
            max_tokens: PICK_TOKENS,
            temperature: TEMPERATURE,
        };
        let g = model.generate(request, &mut |_| {}).await?;
        // An unparseable pick is not fatal: write from all the snippets.
        if let Some(chosen) = prompt::parse_pick(&g.text, picked.len()) {
            picked = chosen.into_iter().map(|i| picked[i]).collect();
        }
    }

    let mut problems: Option<Vec<String>> = None;
    for _ in 0..2 {
        if problems.is_some() {
            on_event(Event::Retrying);
        }
        let request = Request {
            prompt: prompt::build(kind, molecule, &picked, problems.as_deref()),
            schema: prompt::output_schema(kind, picked.len()),
            max_tokens: MAX_NEW_TOKENS,
            temperature: TEMPERATURE,
        };
        let mut text = String::new();
        let mut shown = (String::new(), String::new());
        let g = model
            .generate(request, &mut |piece| {
                text.push_str(piece);
                let now = (
                    partial_string(&text, "title").unwrap_or_default(),
                    partial_string(&text, "body").unwrap_or_default(),
                );
                if now != shown && !now.0.is_empty() {
                    shown = now.clone();
                    on_event(Event::Draft {
                        title: now.0,
                        body: now.1,
                    });
                }
            })
            .await?;

        let draft = match validate::parse(&g.text) {
            Ok(Reply::Skip) => return Ok(Outcome::Skipped),
            Ok(Reply::Card(draft)) => draft,
            Err(e) => {
                problems = Some(vec![e.to_string()]);
                continue;
            }
        };
        if let Err(rejections) = validate::validate(&draft, molecule, &picked) {
            problems = Some(rejections.iter().map(ToString::to_string).collect());
            continue;
        }
        on_event(Event::Checking);
        let Some(verdicts) = check_claims(model, molecule, &draft, &picked).await? else {
            problems = Some(vec!["claim check reply unparseable".into()]);
            continue;
        };
        if verdicts.iter().all(|&ok| ok) {
            return Ok(Outcome::Card(finished(kind, draft, &picked)));
        }
        if let Some(trimmed) = trim(&draft, &verdicts)
            && validate::validate(&trimmed, molecule, &picked).is_ok()
        {
            return Ok(Outcome::Card(finished(kind, trimmed, &picked)));
        }
        problems = Some(
            verify::claims(&draft)
                .iter()
                .zip(&verdicts)
                .filter(|(_, ok)| !**ok)
                .map(|(c, _)| format!("not stated in the cited snippets: \"{c}\""))
                .collect(),
        );
    }
    Ok(Outcome::Dropped(problems.unwrap_or_default()))
}

/// The card without its unsupported body sentences, when the title is
/// supported and enough checked sentences remain. Everything left was judged
/// supported, so trimming keeps the accuracy bar; a rewrite could fail anew.
/// Not when the new opening sentence leans on the removed one ("It was…").
fn trim(draft: &CardDraft, verdicts: &[bool]) -> Option<CardDraft> {
    const LEANING: [&str; 8] = ["It", "Its", "This", "These", "They", "He", "She", "That"];
    let (title_ok, body) = verdicts.split_first()?;
    let kept: Vec<&str> = validate::sentences(&draft.body)
        .into_iter()
        .zip(body)
        .filter(|(_, ok)| **ok)
        .map(|(s, _)| s)
        .collect();
    let first_word = kept.first()?.split_whitespace().next().unwrap_or_default();
    let dangling = body.first() == Some(&false) && LEANING.contains(&first_word);
    (*title_ok && !dangling && kept.len() >= validate::MIN_SENTENCES).then(|| CardDraft {
        body: kept.join(" "),
        ..draft.clone()
    })
}

/// Ask the model, claim by claim (title, then each body sentence), whether
/// the cited snippets state the card. `None` if the reply is unusable.
async fn check_claims<G: Generate>(
    model: &G,
    molecule: &str,
    draft: &CardDraft,
    listed: &[&Snippet],
) -> Result<Option<Vec<bool>>, GenerateError> {
    let cited: Vec<&Snippet> = draft
        .sources
        .iter()
        .filter_map(|id| cited(id, listed))
        .collect();
    let claims = verify::claims(draft);
    let request = Request {
        prompt: verify::build(Style::Verdict, molecule, draft, &cited),
        schema: verify::schema(Style::Verdict, claims.len()),
        max_tokens: CHECK_TOKENS,
        temperature: 0.0,
    };
    let g = model.generate(request, &mut |_| {}).await?;
    Ok(verify::parse(&g.text, claims.len()))
}

/// The value of string field `key` in JSON that may stop anywhere: complete,
/// cut off mid-string, or `None` before the value starts. Only top-level
/// fields are expected; the card schema has no nesting before `sources`.
fn partial_string(json: &str, key: &str) -> Option<String> {
    let at = json.find(&format!("\"{key}\""))? + key.len() + 2;
    let rest = json[at..].trim_start().strip_prefix(':')?;
    let mut chars = rest.trim_start().strip_prefix('"')?.chars();
    let mut out = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => break,
            '\\' => match chars.next() {
                Some('n') => out.push('\n'),
                Some('t') => out.push('\t'),
                Some('u') => {
                    let hex: String = chars.by_ref().take(4).collect();
                    match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                        Some(c) if hex.len() == 4 => out.push(c),
                        // Cut off mid-escape, or a surrogate pair: leave it out for now.
                        _ => {}
                    }
                }
                Some(other) => out.push(other),
                None => break,
            },
            c => out.push(c),
        }
    }
    Some(out)
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

    const GOOD: &str = r#"{"title": "First squeezed out in 1816", "body": "Capsaicin was first extracted in impure form in 1816. Christian Friedrich Bucholz did it.", "sources": ["s1"]}"#;
    const ALL_TRUE: &str =
        r#"{"claims": [{"supported": true}, {"supported": true}, {"supported": true}]}"#;

    async fn run(model: &Scripted, kind: CardKind) -> (Outcome, Vec<Event>) {
        let mut events = Vec::new();
        let outcome = card(model, kind, "capsaicin", &capsaicin(), &mut |e| {
            events.push(e)
        })
        .await
        .unwrap();
        (outcome, events)
    }

    #[test]
    fn verdict_reply_shape_matches_the_checker() {
        // The scripted claim check must be a reply verify::parse accepts.
        let draft: CardDraft = serde_json::from_str(GOOD).unwrap();
        let claims = verify::claims(&draft);
        assert_eq!(claims.len(), 3);
        assert_eq!(verify::parse(ALL_TRUE, 3), Some(vec![true; 3]));
    }

    #[tokio::test]
    async fn a_checked_card_cites_its_snippets() {
        let model = Scripted::new(&[GOOD, ALL_TRUE]);
        let (outcome, events) = run(&model, CardKind::Origin).await;
        let Outcome::Card(card) = outcome else {
            panic!("{outcome:?}")
        };
        assert_eq!(card.title, "First squeezed out in 1816");
        assert_eq!(
            card.sources,
            [Source {
                snippet_id: "11".into(),
                label: "Wikipedia, History".into(),
                url: "https://example.org/11".into(),
            }]
        );
        assert_eq!(events.first(), Some(&Event::Writing));
        assert_eq!(events.last(), Some(&Event::Checking));
        let drafts: Vec<_> = events
            .iter()
            .filter_map(|e| match e {
                Event::Draft { title, body } => Some((title.clone(), body.clone())),
                _ => None,
            })
            .collect();
        assert!(drafts.len() > 3, "streamed: {drafts:?}");
        assert_eq!(drafts.last().unwrap().1, card.body);
    }

    #[tokio::test]
    async fn an_unsupported_claim_is_retried_with_the_problem_then_dropped() {
        let flagged =
            r#"{"claims": [{"supported": true}, {"supported": false}, {"supported": true}]}"#;
        let model = Scripted::new(&[GOOD, flagged, GOOD, flagged]);
        let (outcome, events) = run(&model, CardKind::Origin).await;
        let Outcome::Dropped(problems) = outcome else {
            panic!("{outcome:?}")
        };
        assert_eq!(
            problems,
            [
                "not stated in the cited snippets: \"Capsaicin was first extracted in impure form in 1816.\""
            ]
        );
        assert!(events.contains(&Event::Retrying));
        let asked = model.asked();
        assert_eq!(asked.len(), 4);
        assert!(asked[2].contains("Problems found: not stated in the cited snippets"));
    }

    #[tokio::test]
    async fn an_unsupported_sentence_is_trimmed_when_enough_remains() {
        let three = r#"{"title": "First squeezed out in 1816", "body": "Capsaicin was first extracted in impure form in 1816. Christian Friedrich Bucholz did it. It was very spicy work.", "sources": ["s1"]}"#;
        let last_flagged = r#"{"claims": [{"supported": true}, {"supported": true}, {"supported": true}, {"supported": false}]}"#;
        let model = Scripted::new(&[three, last_flagged]);
        let (outcome, events) = run(&model, CardKind::Origin).await;
        let Outcome::Card(card) = outcome else {
            panic!("{outcome:?}")
        };
        assert_eq!(
            card.body,
            "Capsaicin was first extracted in impure form in 1816. Christian Friedrich Bucholz did it."
        );
        assert!(!events.contains(&Event::Retrying));

        // Nor when "It" would open the card, its subject gone.
        let first_flagged = r#"{"claims": [{"supported": true}, {"supported": false}, {"supported": true}, {"supported": true}]}"#;
        let leaning = r#"{"title": "First squeezed out in 1816", "body": "Capsaicin was first extracted in impure form in 1816. It was Christian Friedrich Bucholz. It was very spicy work.", "sources": ["s1"]}"#;
        let draft: CardDraft = serde_json::from_str(leaning).unwrap();
        let verdicts: Vec<bool> = verify::parse(first_flagged, 4).unwrap();
        assert_eq!(trim(&draft, &verdicts), None);

        // A flagged title can't be trimmed away.
        let title_flagged = r#"{"claims": [{"supported": false}, {"supported": true}, {"supported": true}, {"supported": true}]}"#;
        let model = Scripted::new(&[three, title_flagged, three, title_flagged]);
        assert!(matches!(
            run(&model, CardKind::Origin).await.0,
            Outcome::Dropped(_)
        ));
    }

    #[tokio::test]
    async fn a_validator_rejection_is_retried_and_can_pass() {
        // 1817 is in no snippet.
        let wrong = r#"{"title": "Hot news", "body": "Capsaicin was first extracted in 1817. Bucholz did it.", "sources": ["s1"]}"#;
        let model = Scripted::new(&[wrong, GOOD, ALL_TRUE]);
        let (outcome, _) = run(&model, CardKind::Origin).await;
        assert!(matches!(outcome, Outcome::Card(_)), "{outcome:?}");
        assert!(model.asked()[1].contains("1817"));
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
        assert!(events.is_empty() && model.asked().is_empty());
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

    #[test]
    fn partial_strings_from_cut_off_json() {
        let full = r#"{"title": "Hot \"stuff\"", "body": "Lineé two"#;
        assert_eq!(
            partial_string(full, "title").as_deref(),
            Some("Hot \"stuff\"")
        );
        assert_eq!(
            partial_string(full, "body").as_deref(),
            Some("Line\u{e9} two")
        );
        assert_eq!(
            partial_string(r#"{"title": "Ho"#, "title").as_deref(),
            Some("Ho")
        );
        assert_eq!(
            partial_string(r#"{"title": "A\u00"#, "title").as_deref(),
            Some("A")
        );
        assert_eq!(partial_string(r#"{"title": "#, "title"), None);
        assert_eq!(partial_string(r#"{"title": "x", "bo"#, "body"), None);
    }
}

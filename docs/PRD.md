# Valence — Molecule Fun Facts App PRD

Oct 18, 2026 · @Roman

## Overview

Valence is a macOS desktop app that turns any molecule into a set of short, accurate, surprising fact cards: where you meet it, who discovered it, how dangerous it is, and the strangest thing about it. Every fact comes from real chemistry data, and a local AI model running on Apple Silicon turns that data into readable cards.

**Vision:** make chemistry feel like a cabinet of curiosities. You type "capsaicin" and within seconds you see a rotating 3D model and learn why birds don't feel its heat, how it relates to vanilla, and what dose would actually hurt you.

**Why it exists:**

- **Curiosity has no good tool.** Wikipedia is long, PubChem is dense, and chatbots invent facts with confidence. Nothing gives a fast, trustworthy, fun answer to "tell me something cool about this molecule."
- **Accuracy is the product.** Toxicity and history are exactly where AI models hallucinate. Valence only lets the model retell facts it was given, and it shows the source for each one.
- **It is a learning and portfolio project.** Desktop Rust, local AI inference on Metal, and retrieval-augmented generation in a real, open-source product.

Valence is the product name.

## Goals, non-goals and success metrics

The POC succeeds when searching a common molecule shows a 3D model and four sourced fact cards in under 10 seconds, with no invented facts.

**Goals**

1. Deliver fun, short fact cards for any compound in PubChem that has descriptive data.
2. Guarantee that every factual claim traces to a fetched source.
3. Run the AI fully on-device on an M1 with 16 GB, with no cloud AI calls.
4. Ship as an open-source project with a polished README, demo GIF and signed macOS build.

**Non-goals for the POC**

- Drawing or editing molecules, reaction prediction, or any computational chemistry
- Windows and Linux builds
- Accounts, sync or any server of our own
- Medical or safety advice beyond showing official hazard classifications

**Success metrics**

| Metric | Target |
| --- | --- |
| Time to first card, uncached molecule | under 10 s on M1, 16 GB |
| Time to open a cached molecule | under 1 s |
| Cards with a claim not found in the source data | 0 in a review of 50 molecules |
| Molecules with at least 3 cards out of the 100 most-viewed PubChem compounds | 90 or more |
| App memory with model loaded | under 5 GB |
| GitHub stars in 3 months after launch | 200 (stretch goal) |

## Target users and use cases

The primary user is a curious adult learning chemistry for fun, who wants quick, trustworthy stories about molecules rather than a textbook.

| User | What they want | Example |
| --- | --- | --- |
| Curious adult learner (primary) | Fun, reliable facts that make chemistry stick | "What is in chili peppers, and why does it burn?" |
| Hobbyist who also uses Mathematica | Quick context before or after deeper exploration | Looks up a compound's story, then models it in Mathematica |
| Student | Memorable hooks for compounds in their course | Learns why aspirin came from willow bark |
| Science communicator or teacher | Shareable, sourced trivia | Exports a card for a class or post |

**Core use cases**

1. **Look up:** search a molecule I just heard about and get the interesting parts in seconds.
2. **Wander:** press "Surprise me" and discover a molecule I didn't know existed.
3. **Follow a thread:** jump from caffeine to its relatives in chocolate and tea.
4. **Check safety:** see whether something is dangerous, and how, from the official classification.
5. **Ask:** ask a follow-up question answered only from this molecule's data.

## Core experience and key flows

The app is a single window: a search bar on top, the 3D model on the left, fact cards streaming in on the right.

**Main screen layout**

- **Top bar:** search with autocomplete, a "Surprise me" button, favorites, settings.
- **Left panel:** rotatable 3D model, the common name, formula, molar mass and a few synonyms.
- **Right panel:** fact cards, each with a title, two to four sentences, and a source link at the bottom.
- **Footer:** model status (loaded, loading, not downloaded) and data freshness (cached date).

**Flow 1: search a molecule**

1. The user types a name; autocomplete suggests matches from PubChem.
2. The user picks one. Structured facts (formula, mass, hazard labels) and the 3D model appear immediately from data.
3. The AI cards stream in one by one as each is generated.
4. Everything is saved to the local cache.

**Flow 2: surprise me**

1. The user presses "Surprise me".
2. The app picks a compound from a curated list of about 500 molecules with good stories.
3. The same screen loads as in Flow 1.

**Flow 3: ask a question (nice-to-have)**

1. The user types a question under the cards, such as "Why is it bitter?"
2. The model answers using only the fetched data for this molecule.
3. If the data has no answer, it says so instead of guessing.

**First launch**

1. A welcome screen explains that the AI runs locally.
2. The user downloads the recommended model (about 2.5 GB) with a progress bar.
3. While it downloads, data-only features already work: search, 3D view and hazard labels.

## Functional requirements

Nine requirements make up the POC; everything else waits until the core loop of search, view and read cards feels fast and trustworthy.

| ID | Requirement | Acceptance criteria | Priority |
| --- | --- | --- | --- |
| F1 | Search with autocomplete | Suggestions appear within 300 ms of typing; accepts common names, IUPAC names and CAS numbers | P0 (POC) |
| F2 | Molecule header | Shows name, formula, molar mass and up to 5 synonyms from PubChem | P0 (POC) |
| F3 | 3D structure view | Rotate, zoom and toggle ball-and-stick or space-filling; rendered with 3Dmol.js (WebGL) in the app's webview; falls back to 2D image if no 3D conformer exists | P0 (POC) |
| F4 | Fact card: Where you meet it | Everyday uses and natural occurrence, grounded in fetched data, with source link | P0 (POC) |
| F5 | Fact card: Origin story | Discovery or first synthesis, who and when; omitted when the data has no history | P0 (POC) |
| F6 | Fact card: Danger level | Official GHS hazard pictograms and statements shown verbatim, plus a plain-language summary generated from them | P0 (POC) |
| F7 | Fact card: Weird fact | The most surprising detail found in the data; omitted if nothing qualifies | P0 (POC) |
| F8 | Surprise me | Picks a random molecule from a curated list of about 500 | P0 (POC) |
| F9 | Local cache and offline mode | Cached molecules open in under 1 s with no network; cards are stored, not regenerated | P0 (POC) |
| F10 | Related molecules | Up to 5 structurally similar or same-family compounds, clickable | P1 (next) |
| F11 | Ask a question | Answers only from this molecule's fetched data; says "not in the data" otherwise | P1 (next) |
| F12 | Favorites and history | Star molecules; browse recently viewed | P1 (next) |
| F13 | Regenerate a card | Re-run one card, for example in a different tone or language | P1 (next) |
| F14 | Export card as image | PNG of a card with the molecule and source, for sharing | P2 (later) |
| F15 | Explanation language | Cards in English, Spanish or Russian | P2 (later) |
| F16 | 3D renderer polish | Renderer polish: lighting, atom labels and smooth transitions between molecules | P2 (later) |

Cards with no supporting data are hidden, never filled with general knowledge from the model.

## Data sources and grounding

All facts come from PubChem and Wikipedia; the local model never adds facts of its own.

| Source | What we take | Access |
| --- | --- | --- |
| [PubChem PUG-REST](https://pubchem.ncbi.nlm.nih.gov/docs/programmatic-access) | Name to CID lookup, formula, molar mass, synonyms, 3D conformer (SDF), 2D image, similar compounds | Free REST API, JSON and SDF |
| [PubChem PUG-View](https://pubchem.ncbi.nlm.nih.gov/docs/programmatic-access) | Annotated sections: uses and manufacturing, GHS hazard classification, toxicity, natural occurrence | Free REST API, one compound per request |
| Wikipedia REST API | Article summary and the History section, for origin stories | Free REST API |
| Curated list (bundled) | About 500 molecules with good stories, for "Surprise me" | JSON file in the app |

**Fetching rules**

- Stay at or below 5 requests per second to PubChem, which its usage policy asks of every client; use a shared rate limiter in the Rust backend.
- PubChem requests time out after 30 seconds on its side; set our own timeout at 15 s and retry once with backoff on 429 or 5xx errors.
- Fetch in parallel within the limit: core properties and the 3D conformer first, so the left panel renders before annotations arrive.
- Send a descriptive User-Agent with the app name, version and repository URL.

**Grounding rules**

- Every fetched passage is stored as a *source snippet* with its URL and section name.
- The model receives only snippets, never the open question "tell me about X".
- Every card stores which snippet IDs it used, and the UI links the card to those sources.
- Hazard pictograms and statements are rendered from GHS data directly; the model only writes the plain-language summary beside them.

**Caching**

- Raw responses, snippets and generated cards are stored in SQLite.
- Cached data is considered fresh for 90 days, then refetched in the background when viewed.
- Cards are regenerated only when their source snippets change or the user asks.

## AI design

The model is a retelling engine: it receives source snippets, writes one card at a time, cites the snippets it used, and a validator rejects any card that cites nothing or names facts not in its sources.

**Model**

- A 3–4B parameter instruction model, 4-bit GGUF (Q4\_K\_M), about 2–2.5 GB, as the default.
- An optional 7–8B model as a "higher quality" setting, for users with headroom.
- Inference through mistral.rs with the Metal backend, streaming tokens to the UI.
- Model family chosen by an evaluation on 30 molecules at the start of development, since small models change every few months.

**Pipeline per card**

1. **Select snippets:** pick the snippets relevant to the card type, such as uses and occurrence for "Where you meet it".
2. **Skip if empty:** no relevant snippets means no card.
3. **Generate:** prompt the model with the card type, the snippets (each with an ID) and the output format.
4. **Validate:** parse the output and run the checks below.
5. **Retry once** with a stricter prompt if validation fails, then drop the card.

**Output format**

```json
{"title": "Birds don't feel the burn", "body": "2 to 4 sentences", "sources": ["s3", "s7"]}
```

**Validation checks**

- `sources` is not empty and every ID exists among the snippets given.
- Every number, year and proper name in `body` appears in at least one cited snippet (simple normalized string matching).
- Body length is 2 to 4 sentences and under 80 words.

**Prompt rules**

- Use only the facts in the snippets; if something is not there, leave it out.
- Be playful in tone but exact in content; no exaggeration of danger or safety.
- Never give instructions for making, extracting or concentrating a substance; snippets about manufacturing methods are excluded from card input.
- For the Danger card, summarize only the official hazard statements and never state safe doses.

**Ask a question (P1)**

The same pipeline with the user's question as the card type: retrieve the most relevant snippets by keyword and embedding match, answer with citations, and return "That isn't in the data for this molecule" when nothing relevant is found.

## Architecture and tech stack

Valence is a Tauri app: a React frontend in the system webview on top of a Rust core that fetches, caches, generates and validates. The model runs on the GPU through Metal; the 3D view uses WebGL in the webview.

&#91;embedded content: Valence architecture · UI, Rust core, external data and local model\]

The UI never talks to PubChem or the model directly; it calls Tauri commands and listens to Tauri events, a strict content security policy blocks any other network access from the webview, every fact passes through the cache and every card through the validator.

| Layer | Choice | Why |
| --- | --- | --- |
| App shell | Tauri 2 | Rust backend with a system-webview UI; small bundle, signed and notarized macOS builds, cross-platform later |
| UI | React and TypeScript, built with Vite | Text-heavy UI (autocomplete, streaming cards, links) is quick to build in HTML and CSS; the webview works with VoiceOver |
| UI ↔ core | Tauri commands and events | Commands start lookups and generation; events stream tokens and partial results to the UI |
| 3D view | 3Dmol.js | Reads PubChem SDF directly; ball-and-stick and space-filling out of the box; WebGL |
| Inference | mistral.rs with Metal backend | Pure Rust, loads GGUF, streams tokens |
| Async and networking | tokio, reqwest, governor for rate limiting | Standard async Rust stack |
| Storage | SQLite via rusqlite | Single local file, no server |
| Serialization | serde and serde\_json; SDF parsed in Rust | PubChem JSON, model output and 3D coordinates |
| Packaging | Tauri bundler | Signed and notarized .app and .dmg for GitHub Releases |
| Testing | 50-molecule review set with snapshot tests of cards | Catches accuracy regressions when prompts or models change |

**Code layout**

- `crates/valence-core`: data clients, snippet extraction, cache, generation and validation; no UI code, testable from the command line.
- `app/src-tauri`: the Tauri shell, which exposes the core to the frontend as commands and events.
- `app/src`: the React frontend, including the 3Dmol.js viewer.

## Data model

Five SQLite tables hold everything: molecules, their source snippets, generated cards, the links between cards and snippets, and user state.

```sql
CREATE TABLE molecule (
  cid            INTEGER PRIMARY KEY,   -- PubChem compound ID
  name           TEXT NOT NULL,
  formula        TEXT,
  molar_mass     REAL,
  synonyms_json  TEXT,                  -- up to 5 common synonyms
  sdf_3d         TEXT,                  -- 3D conformer, NULL if none
  ghs_json       TEXT,                  -- pictograms + hazard statements, verbatim
  fetched_at     TEXT NOT NULL
);

CREATE TABLE snippet (
  id         INTEGER PRIMARY KEY,
  cid        INTEGER NOT NULL REFERENCES molecule(cid),
  source     TEXT NOT NULL,             -- 'pubchem_view' | 'wikipedia'
  section    TEXT NOT NULL,             -- e.g. 'Uses', 'History', 'Toxicity'
  url        TEXT NOT NULL,
  text       TEXT NOT NULL,
  hash       TEXT NOT NULL              -- detects changed sources on refetch
);

CREATE TABLE card (
  id           INTEGER PRIMARY KEY,
  cid          INTEGER NOT NULL REFERENCES molecule(cid),
  kind         TEXT NOT NULL,           -- 'where' | 'origin' | 'danger' | 'weird' | 'answer'
  title        TEXT NOT NULL,
  body         TEXT NOT NULL,
  model        TEXT NOT NULL,           -- model file name, for regeneration
  created_at   TEXT NOT NULL
);

CREATE TABLE card_source (
  card_id     INTEGER NOT NULL REFERENCES card(id),
  snippet_id  INTEGER NOT NULL REFERENCES snippet(id),
  PRIMARY KEY (card_id, snippet_id)
);

CREATE TABLE user_molecule (
  cid          INTEGER PRIMARY KEY REFERENCES molecule(cid),
  favorite     INTEGER NOT NULL DEFAULT 0,
  last_viewed  TEXT
);
```

Models, settings and the curated "Surprise me" list live outside the database: model files in the app's Application Support folder, settings in a small TOML file.

## Non-functional requirements

The app must feel instant on cached molecules, stay under 5 GB of memory with the model loaded, and never send user data anywhere.

| Area | Requirement |
| --- | --- |
| Platform | macOS 13 or later, Apple Silicon only; reference machine M1, 16 GB |
| Cold start | App window visible in under 2 s; model loads in the background |
| Model load | Default model ready in under 10 s after launch |
| First card | Under 10 s for an uncached molecule, streaming starts under 3 s |
| Cached molecule | Header, 3D view and stored cards in under 1 s |
| Memory | Under 5 GB total with the default model loaded |
| Privacy | No telemetry, no accounts; network calls only to PubChem, Wikipedia and Hugging Face for model downloads |
| Offline | Cached molecules fully usable offline; search shows a clear offline message |
| Accessibility | Full keyboard navigation; cards readable by VoiceOver; respects system light and dark mode |
| Distribution | Signed and notarized .dmg on GitHub Releases; open source under MIT or Apache-2.0 |
| Attribution | Source links on every card; PubChem and Wikipedia credited in About, per their terms |

## Milestones and roadmap

The POC ships in five milestones over about twelve weeks of evenings. Using Tauri and 3Dmol.js shortens M2 to two weeks, and the time saved goes to the card pipeline in M3.

1. **M0 · Model spike (week 1–2):** load two or three candidate models with mistral.rs on Metal, generate cards for 30 molecules from hand-pasted snippets, and pick the default model.
   - Exit: one model meets the 10 s target and passes validation on most cards.
2. **M1 · Data layer (week 3–4):** PubChem and Wikipedia clients with rate limiting, snippet extraction, SQLite cache.
   - Exit: a CLI command prints all snippets for any CID.
3. **M2 · App shell and 3D view (week 5–6):** Tauri window, search with autocomplete, molecule header, and a 3Dmol.js ball-and-stick view with rotate and zoom.
   - Exit: searching any molecule shows its header and a smooth 3D model at 60 fps on the M1.
4. **M3 · Cards end to end (week 7–10):** the four card types, streaming into the UI, validation, source links, cache.
   - Exit: success metrics for speed and accuracy met on the 50-molecule review set.
5. **M4 · Polish and launch (week 11–12):** Surprise me, first-launch model download, offline mode, signed build, README, demo GIF, launch post.
   - Exit: v0.1 public on GitHub.

**After the POC**

- **v0.2:** related molecules, Ask a question, favorites and history.
- **v0.3:** export as image, more card languages, space-filling view and renderer polish.
- **v1.0:** Windows and Linux builds, since Tauri is cross-platform.

## Risks and open questions

The biggest risk is that small models stay accurate but write dull cards; the model spike in M0 exists to find that out first.

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Small model writes accurate but boring cards | The "fun" promise fails | Few-shot examples of great cards in the prompt; optional 7–8B model; card-specific prompts |
| Model adds facts not in the snippets | Wrong facts, lost trust | Snippet-only prompts, citation requirement, string-match validation, drop on failure |
| Two languages to maintain (Rust and TypeScript) | Slower changes across the boundary | Keep the frontend thin; all logic lives in the Rust core; shared types generated or kept in one module per side |
| 3Dmol.js is slow or awkward on large molecules | Choppy 3D view | Ball-and-stick only in the POC; test the largest curated molecules in M2; keep the core crate UI-free so it can ship as a CLI demo if needed |
| Sparse data for less common molecules | Few or no cards | Hide empty cards; curate Surprise me from well-documented molecules |
| PubChem throttling or downtime | Slow or failed lookups | Rate limiter, retries with backoff, aggressive caching |
| PubChem annotation structure is complex | Slow M1 | Start with 4 or 5 sections only; write parsers per section |
| 3D conformer missing for some compounds | Empty left panel | Fall back to PubChem's 2D image |
| Model download size, about 2.5 GB | Users drop off at first launch | Data features work during download; clear progress and size up front |

**Open questions**

- [ ] Decided: the product name is Valence.
- [ ] Which model family wins the M0 spike?
- [x] Decided (Oct 20, 2026): Tauri with a React frontend and 3Dmol.js, replacing the earlier Iced choice. The UI is mostly text, a ready-made molecule viewer removes the custom renderer, and the webview supports VoiceOver; the Rust core is unchanged.
- [x] Decided: license is MIT.
- [ ] Should the Danger card show toxicity numbers such as LD50, or only GHS hazard classes?
- [ ] Decided: models download from Hugging Face; the app pins each model to a specific repository revision and verifies the file checksum.

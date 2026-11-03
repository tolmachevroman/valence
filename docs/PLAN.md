# Valence — Build Plan

Derived from the PRD (Oct 18, 2026). Each task is sized to roughly one evening and ends in something you can run or test. Check items off as they land.

Order follows the PRD: the riskiest unknown (can a small local model write good, grounded cards?) is answered first, then data, then UI, then the full card pipeline, then launch.

---

## Phase 0 · Groundwork (a few evenings)

- [x] **0.1 Toolchain** — `rustup default stable`; add `rust-toolchain.toml` pinning stable + `rustfmt`, `clippy`.
- [x] **0.2 Workspace** — Cargo workspace with `crates/valence-core` and the Tauri crate in `app/src-tauri`; React + TypeScript frontend in `app/`. Shared lints and edition in the workspace `Cargo.toml`.
- [x] **0.3 CI** — GitHub Actions on `macos-14` (Apple Silicon): frontend lint + build, then `fmt --check`, `clippy -D warnings`, `test`.
- [x] **0.4 Repo hygiene** — `.gitignore` (target, model files, `*.sqlite`), README skeleton, copy the PRD to `docs/PRD.md`.

Done when: `cargo test --workspace` passes locally and in CI.

---

## M0 · Model spike (weeks 1–2)

Goal: pick the default model, and find out early whether small models write cards that are both accurate and fun.

- [x] **M0.1 Hello mistral.rs** — `valence-core` example that loads a GGUF (Q4_K_M) on Metal and streams tokens to stdout. Print load time, time to first token, tokens/s, peak memory. Results in [spike/m0-results.md](spike/m0-results.md).
- [x] **M0.2 Spike fixtures** — `fixtures/spike/<molecule>.json` for 30 molecules, each snippet with `id`, `source`, `section`, `url`, `origin`, `text`. Mix famous (caffeine, capsaicin, aspirin) and obscure. Fetched with `scripts/fetch_spike_fixtures.py` instead of pasted by hand.
- [x] **M0.3 Card types and prompts v1** — `CardKind` enum (`where`, `origin`, `danger`, `weird`), one prompt template per kind, few-shot examples, JSON output format from the PRD. Plus a JSON schema per kind for constrained decoding (`Constraint::JsonSchema`), and `{"skip": true}` for Origin and Weird.
- [x] **M0.4 Validator v1** (production code, not throwaway) — parse JSON; sources non-empty and known; every number, year and proper name in `body` found in a cited snippet (normalized match); 2–4 sentences, under 80 words. Unit tests with good and bad cards.
- [x] **M0.5 Spike runner** — `cargo run --example spike -- --model <path>` generates all cards for all 30 fixtures, with retry-once-stricter, and writes a CSV/Markdown report: pass rate, latency, memory, plus the cards for hand review.
- [x] **M0.6 Bake-off** — run 2–3 current 3–4B instruct models (and one 7–8B for the "higher quality" setting). Score fun 1–5 by hand. Done for three 4B models; provisional pick Qwen3 4B, but about a third of cards still carry an unsupported claim. 7–8B not yet tried.
- [x] **M0.6b Accuracy pass** — Danger card from GHS data without the model; pick-then-write for Weird and Where; prompt rules for hedges, merged facts and unlisted examples; lower temperature. Unsupported claims roughly halved (Qwen3 4B: 27% → 16%).
- [x] **M0.6c Memory** — make memory predictable under 5 GB regardless of how busy the Mac is. Opt-in kernel-pressure memory estimate on our mistral.rs fork, fixed 4K-token paged cache, no plain-cache fallback: 4.1 GB peak on the full Qwen3 run. Upstream pull request still to open.
- [x] **M0.6d Last mile on accuracy** — claim check measured (catches 91% of unsupported cards offline); in the loop it cuts unsupported model cards 21% → 15% but drops 21 cards and doubles time to the first model card. Dose and method sentences scrubbed from snippets (rule violations 3 → 0); Weird example fixed.
- [x] **M0.7 Decision record** — [decisions/0001-default-model.md](decisions/0001-default-model.md): Qwen3 4B Instruct 2507 Q4_K_M, pinned revision and sha256; accuracy over speed and card count, so the claim check gates every model card.

Exit: one model gets first card in under 10 s and passes validation on most cards.

---

## M1 · Data layer (weeks 3–4)

- [ ] **M1.1 HTTP foundation** — shared `reqwest` client, `governor` limiter at 5 req/s for PubChem, 15 s timeout, one retry with backoff on 429/5xx, User-Agent `Valence/<ver> (+repo URL)`.
- [ ] **M1.2 PUG-REST: identity** — autocomplete (`/rest/autocomplete/compound/{q}`), name / CAS → CID, properties (formula, molar mass), top 5 synonyms.
- [ ] **M1.3 PUG-REST: structure** — 3D conformer SDF (handle "no 3D" cleanly), 2D PNG for fallback.
- [ ] **M1.4 PUG-View sections** — fetch by heading; one parser per section, starting with 4–5 only: Uses, Natural occurrence / sources, Toxicity, GHS classification. Manufacturing sections are excluded from card input.
- [ ] **M1.5 GHS** — pictogram codes + hazard statements stored verbatim in `ghs_json`.
- [ ] **M1.6 Wikipedia** — summary + History section → snippets with section URL.
- [ ] **M1.7 SQLite cache** — PRD schema as migration v1 (rusqlite); repository layer; snippet `hash`; `fetched_at` with 90-day freshness.
- [ ] **M1.8 Fetch orchestration** — properties + SDF first, annotations in parallel afterwards, all within the limiter; results stream out as they arrive.
- [ ] **M1.9 CLI** — `valence snippets <name|CAS|CID>` prints the header + all snippets, served from cache on the second run.
- [ ] **M1.10 Tests** — recorded HTTP fixtures for ~5 molecules (no live network in CI); parser tests per section.

Exit: the CLI prints every snippet for any CID.

---

## M2 · App shell and 3D view (weeks 5–6)

- [ ] **M2.1 Layout** — React shell with top bar, left panel, right panel and footer; follows system light/dark.
- [ ] **M2.2 Core ↔ UI bridge** — Tauri commands for lookups, events for streamed results; serde types mirrored in one TypeScript module. The webview never reaches the network (CSP).
- [ ] **M2.3 Search + autocomplete** — debounced input, accessible combobox (keyboard up/down/enter/esc, VoiceOver), under 300 ms.
- [ ] **M2.4 Molecule header** — name, formula (with subscripts), molar mass, synonyms.
- [ ] **M2.5 3D view** — 3Dmol.js component fed the cached SDF from the core; ball-and-stick, rotate and zoom, auto-fit; theme-aware background.
- [ ] **M2.6 2D fallback** — show PubChem PNG (served through the core, not fetched by the webview) when there's no 3D conformer.
- [ ] **M2.7 Perf pass** — 60 fps on M1 with the largest molecules in the curated list.

Exit: searching any molecule shows its header and a smooth 3D model at 60 fps.

---

## M3 · Cards end to end (weeks 7–10)

- [ ] **M3.1 Snippet selection** — per-kind rules (`where` ← uses + occurrence, `origin` ← Wikipedia history, `danger` ← GHS, `weird` ← all non-manufacturing); no snippets → no card. A v1 exists in `valence-core::select` from M0.5.
- [ ] **M3.2 Model service** — loads in the background at launch, one generation queue (shortest card first), token stream to the UI; footer shows loaded / loading / not downloaded. Load with the kernel-pressure memory estimate and a fixed 4K-token paged cache (see M0.6c); if mistral.rs refuses, show a clear "not enough free memory" state instead of falling back.
- [ ] **M3.3 Pipeline** — select → generate → validate → claim check → retry once with the problems → persist or drop. Validator and `verify` from M0, hardened.
- [ ] **M3.4 Danger card** — GHS pictograms + statements rendered straight from data; model writes only the plain-language summary; no doses.
- [ ] **M3.5 Persistence** — `card` + `card_source`; regenerate only when cited snippet hashes change.
- [ ] **M3.6 Card UI** — cards stream in one by one; title, body, source links open in the browser; VoiceOver labels; keyboard focus.
- [ ] **M3.7 Review set** — 50 molecules; snapshot tests of cards (`insta`); script that reports latency and validation stats against PRD targets.

Exit: speed and accuracy metrics met on the 50-molecule review set.

---

## M4 · Polish and launch (weeks 11–12)

- [ ] **M4.1 Surprise me** — bundled `curated.json` (~500 well-documented molecules), random pick.
- [ ] **M4.2 First launch** — welcome screen; model download from Hugging Face at a pinned revision with progress, resume and sha256 check; search, 3D and hazards work during the download.
- [ ] **M4.3 Offline and freshness** — clear offline message in search; cached molecules fully usable; stale (>90 d) data refreshed in the background.
- [ ] **M4.4 Settings and About** — TOML settings (model choice), About with PubChem and Wikipedia attribution.
- [ ] **M4.5 Packaging** — Tauri bundler .app/.dmg; Developer ID signing + notarization; release workflow on tag.
- [ ] **M4.6 Launch kit** — README with demo GIF, launch post.

Exit: v0.1 public on GitHub.

---

## Later (from the PRD roadmap)

- **v0.2:** related molecules (F10), Ask a question (F11), favorites and history (F12), regenerate card (F13).
- **v0.3:** export card as PNG (F14), more languages (F15), space-filling view and renderer polish (F16).
- **v1.0:** Windows and Linux builds.

## Open decisions

- [x] License: MIT (already in `LICENSE`).
- [x] UI stack: Tauri 2 + React + TypeScript (Vite), 3D with 3Dmol.js (decided 2026-10-20; PRD updated).
- [ ] Danger card: GHS classes only, or also LD50? Suggest GHS only for the POC, in line with "never state safe doses".
- [x] Default model: Qwen3 4B Instruct 2507 Q4_K_M (M0.7).
- [x] Accuracy vs speed: accuracy first; the claim check drops cards it can't verify (decided 2026-11-03).
- [ ] Apple Developer ID ($99/yr) for signing and notarization: needed by M4.5.

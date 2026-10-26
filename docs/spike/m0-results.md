# M0 model spike: results log

Machine: MacBook Pro M1 Pro, 16 GB, with Chrome and VS Code open. mistral.rs v0.9.5, Metal, GGUF Q4_K_M, 8K context, thinking off.

## M0.1 smoke test (2026-10-24)

One "Weird fact" card for capsaicin from three snippets (`crates/valence-core/examples/model_smoke.rs`). Warm runs; the first cold load from disk took 22 s.

| Model | Load | First token | Full card | Decode | Peak memory |
| --- | --- | --- | --- | --- | --- |
| Qwen3 4B Instruct 2507 | 7.1 s | 0.79 s | 3.7 s | 40 tok/s | 3.10 GB |
| Qwen3.5 4B | 7.8 s | 0.84 s | 3.9 s | 32 tok/s | 4.12 GB |
| Phi-4 mini instruct | 6.0 s | 0.59 s | 2.7 s | 46 tok/s | 3.13 GB |

All three are inside the PRD targets: model ready under 10 s, streaming under 3 s, card under 10 s, memory under 5 GB.

First impressions from one card each (not a verdict; that's M0.6):

- **Qwen3 4B** added claims not in the snippets ("evolutionary adaptation helps spread chili plants", "selective toxicity"), cited only s2, and wrote ids as numbers.
- **Qwen3.5 4B** was the most playful and stayed close to the snippets; ids as numbers.
- **Phi-4 mini** was accurate but plain, used the requested `"s1"` id format, and said "isolated" where the source says "extracted in impure form".

## M0.5 spike runner (2026-10-25)

Phi-4 mini, all four cards for all 30 fixtures (`crates/valence-core/examples/spike.rs`, full report in `runs/`). Two runs: the first exposed validator bugs, the second is after fixing them.

| | First run | After fixes |
| --- | --- | --- |
| Shown (passed first try + on retry) | 80% (64% + 16%) | 97% (82% + 15%) |
| Dropped after retry | 22 | 3 |
| Molecules with 3+ cards | 23 of 30 | 29 of 30 (cubane has no history or GHS data) |
| First card per molecule | 2.5 s median, 15.8 s max | 1.7 s median, 3.5 s max |
| Peak memory | 4.50 GB | 4.47 GB |

The three remaining drops are real catches: indigo's origin added "Asia", "Greece" and "Rome"; quinine's "Where" invented "1600s"; serotonin's origin ran to 6 sentences twice.

What changed between the runs:

- **Validator false rejections were the biggest loss.** "It's" and "They're" were checked as names, plurals didn't match ("Buckyballs"), and ordinary words opening a sentence ("Beyond", "Always") looked like names. Fixed with contraction and plural handling and the macOS system word list (`/usr/share/dict/words`, whose lowercase entries are ordinary words).
- **The retry now tells the model what was wrong.** Thirteen first attempts were a single sentence, and the old retry prompt only talked about facts.
- **The schema lists the exact source ids** (`s1`..`sN`), so a card can no longer cite a snippet that doesn't exist.
- **The Danger brief forbids advice.** One card said "handle it carefully", which is safety advice the PRD rules out; the validator can't detect that.
- **Shortest card first.** Generating the card with the least input first (usually Danger or Origin) brought the first card from 2.5 s to 1.7 s median.

## M0.6 bake-off (2026-10-25)

All three models through the same spike, then every shown card reviewed against the full text of the snippets it cites, with one written rubric (accuracy: ok / minor / unsupported; rule violations; fun 1–5). Reviews are in `runs/*.review.jsonl`.

| | Phi-4 mini | Qwen3 4B Instruct 2507 | Qwen3.5 4B |
| --- | --- | --- | --- |
| Shown (passed the validator) | 97% | 95% | 98% |
| **Unsupported claims (review)** | 34% | **27%** | 36% |
| Unsupported, Danger cards excluded | 35% | 32% | 40% |
| Rule violations | 13 (7 advice, 6 exaggeration) | 15 (10 advice, 5 exaggeration) | 6 (5 exaggeration, 1 dose) |
| Fun, mean of 1–5 | 2.77 | 2.92 | **3.05** |
| Accurate and fun (no unsupported claim, no rule violation, fun 4+) | 6 | 12 | **16** |
| First card per molecule (median) | **1.7 s** | 2.1 s | 3.6 s |
| Peak memory | **4.47 GB** | 4.73 GB | 7.58 GB (paged attention refused) |
| Model load | **6.9 s** | 7.6 s | 13.1 s |

**No model meets the PRD's accuracy goal** (zero cards with a claim not in the sources). About a third of the cards that passed the validator still contain one. The validator catches invented names and numbers, not these:

- **Wrong subject or relationship:** "frogs and birds use batrachotoxin for hunting" (it's the Chocó people); "caffeine was used as currency" (cocoa beans were); "birds lack the TRPV1 receptor" (they have it; it doesn't respond).
- **Merged facts:** details from separate snippets joined into a false claim; birth and death years presented as event dates.
- **Dropped hedges:** "may have evolved", "probably", "proposed" become plain fact.
- **Unlisted examples:** Where cards add plausible items the snippets never name.
- **Danger cards misread GHS data:** report percentages read as probabilities, "Suspected of" as "may", "Not Classified" as "safe", the Irritant pictogram as a hazard; plus stock advice ("Avoid…", "handle with care").

Errors concentrate in the **Weird** (13–18 of 30) and **Where** (10 of 30) cards, which get the most snippets. **Origin** is good for all three (2–5 of 21–22).

**Provisional pick: Qwen3 4B Instruct 2507.** It has the fewest unsupported claims, meets every speed and memory target, and is close to Qwen3.5 on fun. Qwen3.5 is the most fun but fails memory and load time on this Mac and invents the most. Phi-4 mini is fastest but the least fun. The pick stays provisional until accuracy improves; the fixes below may change the ranking.

**Next experiments, by expected impact:**

1. **Danger card without the model.** GHS hazard statements are already plain language, so render them directly, deduplicated, with the signal word. This removes every Danger error and every advice or exaggeration violation on those cards.
2. **Fewer snippets for Weird and Where.** Have the model first pick the one or two snippets with the best fact (constrained to ids), then write from only those. Less to merge, and a shorter prompt.
3. **Prompt rules for the observed failures:** keep hedges; don't combine facts from different snippets into one claim; name only examples the snippets list.
4. **Lower temperature** (0.6 to about 0.2) for faithfulness, checking that fun holds up.
5. If still short: a verification pass, where the model checks each sentence against its cited snippets, at the cost of extra time per card.

## Findings that affect the app

- **Building needs full Xcode** for the `metal` compiler; the Command Line Tools are not enough.
- **Auto device mapping refuses to load on a normal 16 GB Mac.** mistral.rs counts only free, file-backed and purgeable pages as available (about 2.2 GB with a browser open), not reclaimable app memory. Fix: `with_device_mapping(DeviceMapMetadata::dummy())` puts every layer on Metal.
- **Automatic paged attention makes memory unpredictable** (3–6 GB for the same model) and blocked Qwen3.5 entirely, because it sizes its cache from the same estimate. M0.1 turned it off, but M0.5 found the plain cache peaks at 6.3 GB on a 2,400-token prompt, against 4.9 GB for paged attention with a **fixed** 8K-token cache (`MemoryGoal::KvTokens`). Current approach: fixed-size paged attention, falling back to the plain cache if its startup check refuses.
- **Prompt processing runs at about 300–330 tokens/s** on the M1 Pro with either cache, so prompt length sets time to first token: about 1 s for a 500-token Danger prompt, 7 s for a 2,400-token Weird prompt. Hence a 6,000-character snippet budget per card and generating the shortest card first.
- With both fixes, the app has to do its own check before loading the model (for example, model size against total RAM) instead of relying on mistral.rs's estimate.
- **Wikipedia intro + History misses the best facts.** Capsaicin's birds (the PRD's own example) are under "Natural function", and its 2021 Nobel link under "Mechanism of action". The M0.2 fixtures also take up to 2 paragraphs from other sections, skipping making/extraction sections. If that helps cards in M0.6, M1.6 should do the same.
- **Wikipedia snippets need a finer filter.** A "Structure and occurrence" section carried a chemical equation and "can also be generated by oxidation of…": synthesis content the section-name filter missed. Filter equations and how-it's-made sentences at the snippet level.
- **GHS data disagrees between authorities** (caffeine: EU "Harmful if swallowed", Japan's NITE adds reproductive toxicity). Fixtures use the first authority listed; M1.5 needs a rule.
- Prompts must pin the source id format; two of three models wrote `[2]` instead of `["s2"]`. The validator should reject or normalize this.

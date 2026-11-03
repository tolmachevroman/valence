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

Phi-4 mini, all four cards for all 30 fixtures (`crates/valence-core/examples/spike.rs`, full report of the second run in `runs/m0.6/`). Two runs: the first exposed validator bugs, the second is after fixing them.

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

All three models through the same spike, then every shown card reviewed against the full text of the snippets it cites, with one written rubric (accuracy: ok / minor / unsupported; rule violations; fun 1–5). Runs and reviews are in `runs/m0.6/`.

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

## M0.6b accuracy pass (2026-10-28)

Changes: the Danger card is built from GHS data without the model; Weird and Where cards pick 1–2 (Weird) or 1–3 (Where) snippets first and are written from those only; new prompt rules (keep hedges, keep facts on their own subject, name only listed examples); temperature 0.6 → 0.2. Same fixtures, same review rubric. Runs and reviews are in `runs/m0.6b/`.

| | Unsupported claims | Rule violations | Fun | Accurate and fun (4+) |
| --- | --- | --- | --- | --- |
| Phi-4 mini | 34% → 17% | 13 → 1 | 2.77 → 2.86 | 6 → 11 |
| **Qwen3 4B** | 27% → **16%** | 15 → 3 | 2.92 → **3.19** | 12 → **24** |
| Qwen3.5 4B | 36% → 19% | 6 → 3 | 3.05 → 3.05 | 16 → 20 |

- **Data-built Danger cards had no unsupported claims** for any model, and advice on Danger cards is gone. The review found three template bugs, since fixed: a `**` footnote marker hid nitroglycerin's H373, the raw "(Deleted)" label leaked into the card, and benzylpenicillin's "Danger" signal came from a statement only 48% of notifiers report, which the card counted but didn't list.
- **Qwen3 4B is now clearly ahead:** fewest unsupported claims, and twice as many accurate-and-fun cards as in M0.6 (24). Its Weird cards average 3.83 for fun.
- **Remaining errors** cluster in an editorial closing sentence on Weird cards ("this is surprising because…"), overconfident titles, and still some merged facts. Rule violations left: lethal doses from toxicity snippets (tetrodotoxin) and historical procedures (roasting frogs for dart poison, a recrystallization step).
- **Cost: time.** The pick step reads every candidate snippet, so a model card takes about 7–9 s instead of about 5 s, and the first model card for a molecule arrives at about 5 s median. The Danger card appears instantly, so something is on screen at once.

**Memory is not solved** (resolved in M0.6c below). In this run mistral.rs also refused fixed-size paged attention for Qwen3 4B (the Mac had about 1 GB "available" by its estimate at that moment) and fell back to the plain cache, which peaked at 8.5 GB. The fallback depends on how busy the Mac is, and the plain cache is too large with 1,500-token prompts.

**Still open before M0.7:**

1. **Memory.** Options: patch mistral.rs's macOS free-memory estimate to count reclaimable (inactive) memory, used by both auto device mapping and paged attention, and send it upstream; or move inference to llama.cpp, which has a mature Metal backend with predictable memory.
2. **The last 16%.** The 320 reviewed cards are now a labeled test set, so a verification step (the model checks each sentence against its cited snippets) can be measured offline for how many bad cards it catches and how many good ones it drops, before adding it to the pipeline. Also: rewrite the Weird example card, whose last sentence models the editorial inference the review keeps flagging, and exclude dose and procedure sentences at snippet selection.

## M0.6c memory (2026-11-01)

**Cause.** On macOS every memory check in mistral.rs (automatic device mapping, paged attention planning, per-request admission, attention workspace) reads one estimate that counts only fully reclaimable pages. Upstream tests make this deliberate (`anonymous_pages_require_explicit_volatile_purgeability`). On a 16 GB Mac with a browser and an editor open it is about 1–2 GB, so the model or its cache is refused even though macOS would free the memory.

**What didn't work.** Patching single checks: there are at least three, each re-applying the same estimate. Counting inactive pages (psutil's definition) helped but still refused on a busy Mac.

**Fix.** An opt-in estimate in mistral.rs, `set_macos_available_memory(MacosAvailableMemory::SystemPressure)`, which uses the kernel's memory pressure level (`kern.memorystatus_level`, the figure behind Activity Monitor's pressure graph). The default is unchanged, and the estimate is never lower than the default. It lives on a fork, [tolmachevroman/mistral.rs `valence/macos-available-memory`](https://github.com/tolmachevroman/mistral.rs/tree/valence/macos-available-memory), based on v0.9.5; Valence pins the exact commit. Upstream's memory tests pass, plus new ones; four Metal attention tests fail on this M1 Pro with and without the patch.

Also changed: the fixed KV cache is 4K tokens instead of 8K (the longest prompt is about 2,500 tokens plus 300 generated), there is no fallback to the plain cache (it reached 8.5 GB and stalled a busy Mac; the app should say "not enough free memory" instead), and the manual device mapping workaround from M0.1 is gone.

With another process holding 6 GB (41% free by the pressure level), Qwen3 4B:

| Estimate | Result |
| --- | --- |
| Reclaimable (mistral.rs default) | refused |
| + inactive pages | refused |
| **Kernel pressure level** | **loaded in 6.5 s, 3.78 GB peak** |

Full 30-molecule Qwen3 run with the pressure estimate: **4.06 GB peak**, 7.7 s load, every molecule completed, card outcomes in line with M0.6b (`runs/m0.6c/`). Qwen3.5 is still refused, correctly: it allocates over 5.7 GB for a 2.7 GB file and doesn't fit the 5 GB budget.

## M0.6d claim check (2026-11-03)

**Claim check.** After a card passes the validator, the same model answers, for the title and each body sentence, whether the cited snippets state it (`valence-core::verify`, constrained to one boolean per claim). Measured first against the 239 hand-reviewed model cards from M0.6b (`runs/m0.6d/verify-*`):

| Style | Flags unsupported cards | Flags cards reviewed ok | Time per card |
| --- | --- | --- | --- |
| Verdict only | 91% | 34% | 1.6 s median |
| Quote the evidence, then verdict | 95% | 59% | 5.2 s median |

Verdict only is the one to use. A fifth of its false alarms came from a sentence splitter bug (initials like "F. L." and titles like "Mr." ended a sentence), fixed for the checker and the validator. Titles cause 9 false alarms but are the only check that catches 6 unsupported cards, so they stay in.

**Also changed:** dose sentences (LD50, mg/kg, daily amounts) and method sentences (reaction equations, "reacts with", "by heating…", "are prepared by") are scrubbed from snippets before selection, while history and biology ("synthesized by Sobrero in 1846", "produced by bacteria") stay. The Weird example no longer ends in a conclusion of its own, and the Weird brief no longer asks the model to explain why a fact is surprising.

**Full Qwen3 4B run with the check in the loop** (a flagged card is retried once with the flagged sentences as problems, then dropped), same review rubric (`runs/m0.6d/`):

| | M0.6b | M0.6d |
| --- | --- | --- |
| Model cards with an unsupported claim | 21% (17 of 80) | **15%** (9 of 59) |
| Rule violations | 3 | **0** |
| Accurate model cards shown | **63** | 50 |
| Accurate and fun (4+) | **24** | 19 |
| Molecules with 3+ cards | **28 of 30** | 22 of 30 |
| First model card per molecule (median) | **about 5 s** | 9.6 s |
| Peak memory | 4.1 GB | 4.1 GB |

- **The check works but costs too much as a gate.** Unsupported claims fall by a third, but 21 cards are dropped and 13 fewer accurate cards are shown, and the first model card takes twice as long. In the loop it caught less than in the offline measurement: 9 unsupported cards got through, mostly overconfident titles ("A discarded drug became a miracle cure") and swapped subjects.
- **Good catches** are mostly editorial closers ("This shows how potent morphine was even in early experiments"); **false alarms** are mostly long, accurate sentences with several facts (carvone's share of caraway oil, indigo's 6,000-year-old fabric).
- **Scrubbing removed every rule violation**; no dose or how-to content reached a card.

## Findings that affect the app

- **Building needs full Xcode** for the `metal` compiler; the Command Line Tools are not enough.
- **mistral.rs's macOS memory estimate is too conservative for a desktop app** (about 2 GB available on a busy 16 GB Mac). Resolved in M0.6c with an opt-in estimate based on the kernel's pressure level, on our fork.
- **Use paged attention with a fixed cache, never the plain cache.** Automatic paged attention sized itself from the estimate (3–6 GB for the same model); the plain cache peaked at 6.3–8.5 GB on long prompts. A fixed 4K-token cache (`MemoryGoal::KvTokens(4096)`) peaks at about 4.1 GB with Qwen3 4B.
- **Prompt processing runs at about 300–330 tokens/s** on the M1 Pro with either cache, so prompt length sets time to first token: about 1 s for a 500-token Danger prompt, 7 s for a 2,400-token Weird prompt. Hence a 6,000-character snippet budget per card and generating the shortest card first.
- When mistral.rs still refuses (real memory pressure), the app should say so clearly ("close some apps to load the model") rather than fall back to a configuration that uses more memory.
- **Wikipedia intro + History misses the best facts.** Capsaicin's birds (the PRD's own example) are under "Natural function", and its 2021 Nobel link under "Mechanism of action". The M0.2 fixtures also take up to 2 paragraphs from other sections, skipping making/extraction sections. If that helps cards in M0.6, M1.6 should do the same.
- **Wikipedia snippets need a finer filter.** A "Structure and occurrence" section carried a chemical equation and "can also be generated by oxidation of…": synthesis content the section-name filter missed. Done in M0.6d: `select::scrub` drops dose and method sentences; M1 should apply it as snippets are extracted.
- **GHS data disagrees between authorities** (caffeine: EU "Harmful if swallowed", Japan's NITE adds reproductive toxicity). Fixtures use the first authority listed; M1.5 needs a rule.
- Prompts must pin the source id format; two of three models wrote `[2]` instead of `["s2"]`. The validator should reject or normalize this.

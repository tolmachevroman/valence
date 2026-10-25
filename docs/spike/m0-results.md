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

## Findings that affect the app

- **Building needs full Xcode** for the `metal` compiler; the Command Line Tools are not enough.
- **Auto device mapping refuses to load on a normal 16 GB Mac.** mistral.rs counts only free, file-backed and purgeable pages as available (about 2.2 GB with a browser open), not reclaimable app memory. Fix: `with_device_mapping(DeviceMapMetadata::dummy())` puts every layer on Metal.
- **Automatic paged attention makes memory unpredictable** (3–6 GB for the same model) and blocked Qwen3.5 entirely, because it sizes its cache from the same estimate. M0.1 turned it off, but M0.5 found the plain cache peaks at 6.3 GB on a 2,400-token prompt, against 4.9 GB for paged attention with a **fixed** 8K-token cache (`MemoryGoal::KvTokens`). Current approach: fixed-size paged attention, falling back to the plain cache if its startup check refuses.
- **Prompt processing runs at about 300–330 tokens/s** on the M1 Pro with either cache, so prompt length sets time to first token: about 1 s for a 500-token Danger prompt, 7 s for a 2,400-token Weird prompt. Hence a 6,000-character snippet budget per card and generating the shortest card first.
- With both fixes, the app has to do its own check before loading the model (for example, model size against total RAM) instead of relying on mistral.rs's estimate.
- **Wikipedia intro + History misses the best facts.** Capsaicin's birds (the PRD's own example) are under "Natural function", and its 2021 Nobel link under "Mechanism of action". The M0.2 fixtures also take up to 2 paragraphs from other sections, skipping making/extraction sections. If that helps cards in M0.6, M1.6 should do the same.
- **GHS data disagrees between authorities** (caffeine: EU "Harmful if swallowed", Japan's NITE adds reproductive toxicity). Fixtures use the first authority listed; M1.5 needs a rule.
- Prompts must pin the source id format; two of three models wrote `[2]` instead of `["s2"]`. The validator should reject or normalize this.

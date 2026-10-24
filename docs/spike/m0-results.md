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

## Findings that affect the app

- **Building needs full Xcode** for the `metal` compiler; the Command Line Tools are not enough.
- **Auto device mapping refuses to load on a normal 16 GB Mac.** mistral.rs counts only free, file-backed and purgeable pages as available (about 2.2 GB with a browser open), not reclaimable app memory. Fix: `with_device_mapping(DeviceMapMetadata::dummy())` puts every layer on Metal.
- **Paged attention makes memory unpredictable** (3–6 GB for the same model) and blocked Qwen3.5 entirely, because it sizes its cache from the same estimate. Fix: `without_paged_attn()`; one short request at a time doesn't need it.
- With both fixes, the app has to do its own check before loading the model (for example, model size against total RAM) instead of relying on mistral.rs's estimate.
- **Wikipedia intro + History misses the best facts.** Capsaicin's birds (the PRD's own example) are under "Natural function", and its 2021 Nobel link under "Mechanism of action". The M0.2 fixtures also take up to 2 paragraphs from other sections, skipping making/extraction sections. If that helps cards in M0.6, M1.6 should do the same.
- **GHS data disagrees between authorities** (caffeine: EU "Harmful if swallowed", Japan's NITE adds reproductive toxicity). Fixtures use the first authority listed; M1.5 needs a rule.
- Prompts must pin the source id format; two of three models wrote `[2]` instead of `["s2"]`. The validator should reject or normalize this.

# 0001 · Default model: Qwen3 4B Instruct 2507, Q4_K_M

Status: accepted (2026-11-03). Evidence: [M0 spike results](../spike/m0-results.md).

## Decision

Valence ships with one default model, downloaded on first launch at a pinned revision and checked by sha256:

| | |
| --- | --- |
| Model | Qwen3 4B Instruct 2507 (base: `Qwen/Qwen3-4B-Instruct-2507`) |
| Hugging Face repo | [`unsloth/Qwen3-4B-Instruct-2507-GGUF`](https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF) |
| File | `Qwen3-4B-Instruct-2507-Q4_K_M.gguf` |
| Revision | `a06e946bb6b655725eafa393f4a9745d460374c9` |
| sha256 | `3605803b982cb64aead44f6c1b2ae36e3acdb41d8e46c8a94c6533bc4c67e597` |
| Size | 2,497,281,120 bytes (2.5 GB) |
| License | Apache-2.0 |

Download URL: `https://huggingface.co/unsloth/Qwen3-4B-Instruct-2507-GGUF/resolve/a06e946bb6b655725eafa393f4a9745d460374c9/Qwen3-4B-Instruct-2507-Q4_K_M.gguf`

## How it runs

- mistral.rs from our fork (M0.6c), Metal, with the kernel-pressure memory estimate and a fixed 4K-token paged KV cache. No fallback to the plain cache; if the model doesn't fit, say so.
- Thinking off, JSON schema constraint, temperature 0.2, at most 300 new tokens per card.
- Accuracy over speed and card count (decided 2026-11-03). The pipeline, per card:
  1. Snippets scrubbed of dose and method sentences, then selected per card type.
  2. Danger card built from GHS data, without the model.
  3. Weird and Where: the model picks 1–2 or 1–3 snippets first, then writes from those only.
  4. Validator (names, numbers, codes, length), then the claim check (`verify`, verdict style).
  5. On any problem, one retry with the problems listed; then the card is dropped.

## Numbers (M1 Pro, 16 GB; 30 spike molecules, M0.6d run)

| | |
| --- | --- |
| Model cards with an unsupported claim (hand review) | 15% (Danger cards from data: 0%) |
| Rule violations (advice, dose, how-to, exaggeration) | 0 |
| Fun, mean of 1–5 | 3.13 (Weird cards 3.60) |
| Molecules with 3+ cards | 22 of 30 |
| First card per molecule | instant (Danger, from data) |
| First model card per molecule | 9.6 s median, 18.9 s p90 |
| Model load | 7.4 s |
| Peak memory | 4.14 GB |

## Alternatives

- **Phi-4 mini instruct (Q4_K_M):** fastest and smallest, but more unsupported claims (17% vs 16% in M0.6b) and half as many accurate-and-fun cards (11 vs 24).
- **Qwen3.5 4B (Q4_K_M):** most fun in M0.6, but more unsupported claims (19% in M0.6b) and it needs over 5.7 GB, beyond the 5 GB budget.
- **A 7–8B model** for a "higher quality" setting: not measured. Revisit after v0.1, with the same review rubric.

## Consequences

- The remaining errors are overconfident titles that drop the snippet's hedge, and facts from two snippets merged into one sentence. The next accuracy work should target titles.
- Retries and the claim check make model cards slow (about 10 s each). M3.2 must stream progress so the wait reads as work, and generate the shortest card first.
- A new model, quantization or revision means a new decision record, a full spike run and a hand review with the same rubric.

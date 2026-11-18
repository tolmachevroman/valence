# Valence

A macOS app that turns any molecule into short, accurate, surprising fact cards, with a rotating 3D model. Every fact comes from PubChem or Wikipedia, and a local AI model running on Apple Silicon retells it. Nothing is sent to a cloud AI.

> Early development. See [docs/PLAN.md](docs/PLAN.md) for progress and [docs/PRD.md](docs/PRD.md) for the product spec.

## Install

Download the .dmg from [Releases](https://github.com/tolmachevroman/valence/releases) and drag Valence to Applications. It needs macOS 13 or later on Apple Silicon.

Valence is a free showcase project and isn't notarized by Apple, so macOS blocks it the first time:

- macOS 15 and later: open Valence, close the warning, then go to System Settings → Privacy & Security and click **Open Anyway** next to the message about Valence.
- macOS 13 and 14: Control-click Valence in Applications, choose **Open**, then **Open** again.

Or, in Terminal: `xattr -dr com.apple.quarantine /Applications/Valence.app`

On first launch, Valence offers to download its AI model (2.5 GB, from Hugging Face). Search, 3D models and hazard labels work while it downloads.

## Build requirements

- macOS 13 or later on Apple Silicon
- Rust (the pinned version in `rust-toolchain.toml` installs automatically via rustup)
- Node.js 22 or later
- Xcode (not just the Command Line Tools): mistral.rs compiles its Metal kernels with Apple's `metal` compiler at build time. Check with `xcrun -sdk macosx metal --version`.

## Build

```sh
cd app
npm install
npm run tauri dev        # run the app with hot reload
npm run tauri build      # build Valence.app and a .dmg

cargo test --workspace   # Rust tests, from the repo root
```

## Layout

| Path | Contents |
| --- | --- |
| `crates/valence-core` | Data clients, cache, card generation and validation (no UI) |
| `app/src-tauri` | Tauri shell: exposes the core to the frontend as commands and events |
| `app/src` | React + TypeScript frontend |

## License

The code is MIT. Molecule data comes from [PubChem](https://pubchem.ncbi.nlm.nih.gov/) and [Wikipedia](https://www.wikipedia.org/) and keeps its own terms: Wikipedia text is © Wikipedia contributors under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/), and PubChem data comes from the NCBI and the depositors each record names ([data policies](https://pubchem.ncbi.nlm.nih.gov/docs/about)). That applies to the recorded replies and review runs in this repository (`fixtures/`, `docs/spike/`, `docs/review/`, test snapshots) and to the cards the app shows.

# Valence

A macOS app that turns any molecule into short, accurate, surprising fact cards, with a rotating 3D model. Every fact comes from PubChem or Wikipedia, and a local AI model running on Apple Silicon retells it. Nothing is sent to a cloud AI.

> Early development. See [docs/PLAN.md](docs/PLAN.md) for progress and [docs/PRD.md](docs/PRD.md) for the product spec.

## Requirements

- macOS 13 or later on Apple Silicon
- Rust (the pinned version in `rust-toolchain.toml` installs automatically via rustup)
- Node.js 22 or later

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

MIT. Molecule data from [PubChem](https://pubchem.ncbi.nlm.nih.gov/) and [Wikipedia](https://www.wikipedia.org/).

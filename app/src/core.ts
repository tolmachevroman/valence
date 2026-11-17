// The Rust core as the UI sees it: commands, and the types they send.
// Mirrors app/src-tauri/src/lib.rs and the valence-core types it serializes.

import { Channel, invoke } from '@tauri-apps/api/core'

/** valence_core::pubchem::Compound */
export interface Compound {
  cid: number
  title: string
  /** Plain text like "C8H10N4O2"; subscripts are added when shown. */
  formula: string
  /** g/mol as PubChem formats it, like "194.19". */
  molarMass: string
  iupacName: string | null
  cas: string | null
  synonyms: string[]
}

/** valence_core::ghs::HazardStatement */
export interface HazardStatement {
  code: string
  text: string
  /** Share of notifiers reporting it, when the source gives one. */
  percent: number | null
  level: 'Danger' | 'Warning' | null
}

/** valence_core::ghs::Ghs */
export interface Ghs {
  signal: string | null
  pictograms: string[]
  statements: HazardStatement[]
}

/** valence_core::card::CardKind */
export type CardKind = 'where' | 'origin' | 'danger' | 'weird'

/** CardView: a card that passed every check. */
export interface Card {
  kind: CardKind
  /** "Where you meet it" */
  label: string
  /** Empty for cards made of source sentences; the label heads them. */
  title: string
  body: string
  sources: { label: string; url: string }[]
  /** Danger only: official statements to list verbatim, chosen by the core. */
  hazards: { pictograms: string[]; statements: HazardStatement[] } | null
}

/** MoleculeEvent: one part of a molecule, as it arrives, then its cards. */
export type MoleculeEvent =
  | { kind: 'header'; compound: Compound }
  /** SDF for the 3D view, or a PNG data URL when there is no 3D conformer. */
  | { kind: 'structure'; sdf: string | null; png: string | null }
  | { kind: 'hazards'; ghs: Ghs | null }
  /** Everything arrived and is cached. fetchedAt is Unix seconds. */
  | { kind: 'done'; snippetCount: number; fetchedAt: number }
  /** Cards to expect, in the order they will be written. */
  | { kind: 'cardsPlanned'; cards: CardKind[] }
  | { kind: 'cardWriting'; card: CardKind }
  /** The card so far: the source sentences chosen. */
  | { kind: 'cardDraft'; card: CardKind; body: string }
  | { kind: 'cardRetrying'; card: CardKind }
  | { kind: 'cardReady'; card: Card }
  /** No card of this kind: skipped, failed its checks, or no model. */
  | { kind: 'cardGone'; card: CardKind }

/** ModelStatus: the local model, downloaded on request, loaded in the background. */
export type ModelStatus =
  /** No model file yet. `downloaded` bytes are kept from an earlier attempt; `error` says why it stopped. */
  | { state: 'notDownloaded'; size: number; downloaded: number; error: string | null }
  | { state: 'downloading'; done: number; total: number }
  | { state: 'loading' }
  | { state: 'ready'; loadSeconds: number }
  /** Too little free memory for the model and its cache. */
  | { state: 'notEnoughMemory' }
  | { state: 'failed'; message: string }

export function appVersion(): Promise<string> {
  return invoke<string>('app_version')
}

/** Compound names starting like `query`, best first. */
export function autocomplete(query: string): Promise<string[]> {
  return invoke<string[]>('autocomplete', { query })
}

/**
 * Look up a molecule by name, CAS number or CID and receive its parts as they
 * arrive. Resolves with the CID once everything is in; rejects with a message.
 */
export function openMolecule(query: string, onEvent: (e: MoleculeEvent) => void): Promise<number> {
  const channel = new Channel<MoleculeEvent>()
  channel.onmessage = onEvent
  return invoke<number>('open_molecule', { query, onEvent: channel })
}

/** Receive the model status now and on every change, until it is final. */
export function watchModel(onChange: (s: ModelStatus) => void): Promise<void> {
  const channel = new Channel<ModelStatus>()
  channel.onmessage = onChange
  return invoke<void>('model_status', { onChange: channel })
}

/** Download the model, resuming an earlier attempt; progress arrives through `watchModel`. */
export function downloadModel(): Promise<void> {
  return invoke<void>('download_model')
}

/** "2.5 GB" */
export function gigabytes(bytes: number): string {
  return `${(bytes / 1e9).toFixed(1)} GB`
}

/** Open an https link in the default browser. */
export function openUrl(url: string): Promise<void> {
  return invoke<void>('open_url', { url })
}

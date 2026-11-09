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

/** MoleculeEvent: one part of a molecule, as it arrives. */
export type MoleculeEvent =
  | { kind: 'header'; compound: Compound }
  /** SDF for the 3D view, or a PNG data URL when there is no 3D conformer. */
  | { kind: 'structure'; sdf: string | null; png: string | null }
  | { kind: 'hazards'; ghs: Ghs | null }
  /** Everything arrived and is cached. fetchedAt is Unix seconds. */
  | { kind: 'done'; snippetCount: number; fetchedAt: number }

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

// The open molecule, filled in part by part as the core sends it, then its
// cards as they are written.

import { useCallback, useRef, useState } from 'react'

import { type Card, type CardKind, type Compound, type Ghs, type MoleculeEvent, openMolecule } from './core'

/** One planned card, from waiting for the model to ready (or gone). */
export interface CardSlot {
  kind: CardKind
  state: 'waiting' | 'writing' | 'retrying' | 'ready'
  /** The text so far, unchecked; cleared when a retry starts. */
  draft?: string
  card?: Card
}

export interface MoleculeState {
  /** What was searched for, or the name of a "Surprise me" pick. */
  query: string
  status: 'loading' | 'done' | 'error'
  compound?: Compound
  /** Absent until it arrives; then SDF, or a PNG data URL without 3D. */
  structure?: { sdf: string | null; png: string | null }
  /** Absent until it arrives; null when PubChem has no classification. */
  ghs?: Ghs | null
  /** Unix seconds. */
  fetchedAt?: number
  error?: string
  /** Absent until planned; cards that turn out empty are removed. */
  cards?: CardSlot[]
}

function updateCard(
  cards: CardSlot[] | undefined,
  kind: CardKind,
  change: (slot: CardSlot) => CardSlot | undefined,
): CardSlot[] | undefined {
  return cards?.flatMap((slot) => {
    if (slot.kind !== kind) return [slot]
    const next = change(slot)
    return next ? [next] : []
  })
}

export function useMolecule() {
  const [molecule, setMolecule] = useState<MoleculeState>()
  // Each open gets a number; events from an older open are dropped.
  const current = useRef(0)

  /** Open `query` (a name, CAS number or CID), shown as `name` until the header arrives. */
  const open = useCallback((query: string, name = query) => {
    const request = ++current.current
    const update = (change: (m: MoleculeState) => Partial<MoleculeState>) => {
      if (request === current.current) setMolecule((m) => (m ? { ...m, ...change(m) } : m))
    }
    const card = (kind: CardKind, change: (slot: CardSlot) => CardSlot | undefined) =>
      update((m) => ({ cards: updateCard(m.cards, kind, change) }))
    setMolecule({ query: name, status: 'loading' })
    const onEvent = (e: MoleculeEvent) => {
      switch (e.kind) {
        case 'header':
          return update(() => ({ compound: e.compound }))
        case 'structure':
          return update(() => ({ structure: { sdf: e.sdf, png: e.png } }))
        case 'hazards':
          return update(() => ({ ghs: e.ghs }))
        case 'done':
          return update(() => ({ status: 'done', fetchedAt: e.fetchedAt }))
        case 'cardsPlanned':
          return update(() => ({ cards: e.cards.map((kind) => ({ kind, state: 'waiting' })) }))
        case 'cardWriting':
          return card(e.card, (s) => ({ ...s, state: 'writing' }))
        case 'cardDraft':
          return card(e.card, (s) => ({ ...s, draft: e.body }))
        case 'cardRetrying':
          return card(e.card, (s) => ({ ...s, state: 'retrying', draft: undefined }))
        case 'cardReady':
          return card(e.card.kind, (s) => ({ ...s, state: 'ready', draft: undefined, card: e.card }))
        case 'cardGone':
          return card(e.card, () => undefined)
      }
    }
    openMolecule(query, onEvent).catch((error: unknown) =>
      update(() => ({ status: 'error', error: String(error) })),
    )
  }, [])

  return { molecule, open }
}

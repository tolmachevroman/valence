// The open molecule, filled in part by part as the core sends it.

import { useCallback, useRef, useState } from 'react'

import { type Compound, type Ghs, type MoleculeEvent, openMolecule } from './core'

export interface MoleculeState {
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
}

export function useMolecule() {
  const [molecule, setMolecule] = useState<MoleculeState>()
  // Each open gets a number; events from an older open are dropped.
  const current = useRef(0)

  const open = useCallback((query: string) => {
    const request = ++current.current
    const update = (change: Partial<MoleculeState>) => {
      if (request === current.current) setMolecule((m) => (m ? { ...m, ...change } : m))
    }
    setMolecule({ query, status: 'loading' })
    const onEvent = (e: MoleculeEvent) => {
      switch (e.kind) {
        case 'header':
          return update({ compound: e.compound })
        case 'structure':
          return update({ structure: { sdf: e.sdf, png: e.png } })
        case 'hazards':
          return update({ ghs: e.ghs })
        case 'done':
          return update({ status: 'done', fetchedAt: e.fetchedAt })
      }
    }
    openMolecule(query, onEvent).catch((error: unknown) =>
      update({ status: 'error', error: String(error) }),
    )
  }, [])

  return { molecule, open }
}

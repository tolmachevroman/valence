// Left panel: the 3D model (M2.5, 2D fallback in M2.6) and the molecule header.

import { lazy, Suspense } from 'react'

import type { MoleculeState } from '../useMolecule'
import MoleculeHeader from './MoleculeHeader'

// 3Dmol.js is most of the bundle; load it with the first molecule, not at launch.
const Viewer = lazy(() => import('./Viewer'))

export default function MoleculePanel({ molecule }: { molecule?: MoleculeState }) {
  return (
    <section className="molecule-panel" aria-label="Molecule" aria-busy={molecule?.status === 'loading'}>
      <div className="viewer">
        {molecule?.structure?.sdf ? (
          <Suspense fallback={<span className="placeholder">Loading model…</span>}>
            <Viewer sdf={molecule.structure.sdf} title={molecule.compound?.title ?? molecule.query} />
          </Suspense>
        ) : (
          <span className="placeholder" aria-hidden="true">
            {molecule?.status === 'loading' ? 'Loading model…' : '3D model'}
          </span>
        )}
      </div>
      {molecule?.compound ? (
        <MoleculeHeader compound={molecule.compound} />
      ) : (
        <div className="molecule-header">
          {!molecule && <p className="muted">No molecule yet.</p>}
          {molecule?.status === 'loading' && <p className="muted">Looking up {molecule.query}…</p>}
          {molecule?.status === 'error' && <p role="alert">{molecule.error}</p>}
        </div>
      )}
    </section>
  )
}

// Left panel: the 3D model (M2.5, 2D fallback in M2.6) and the molecule header.

import type { MoleculeState } from '../useMolecule'
import MoleculeHeader from './MoleculeHeader'

export default function MoleculePanel({ molecule }: { molecule?: MoleculeState }) {
  return (
    <section className="molecule-panel" aria-label="Molecule" aria-busy={molecule?.status === 'loading'}>
      <div className="viewer" aria-hidden="true">
        <span className="placeholder">3D model</span>
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

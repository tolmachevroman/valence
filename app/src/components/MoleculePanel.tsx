// Left panel: the 3D model (M2.5, 2D fallback in M2.6) and the molecule
// header (M2.4).

export default function MoleculePanel() {
  return (
    <section className="molecule-panel" aria-label="Molecule">
      <div className="viewer" aria-hidden="true">
        <span className="placeholder">3D model</span>
      </div>
      <div className="molecule-header">
        <p className="muted">No molecule yet.</p>
      </div>
    </section>
  )
}

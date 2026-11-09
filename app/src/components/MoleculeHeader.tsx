// Name, formula, molar mass, CAS number and common synonyms.

import type { Compound } from '../core'
import Formula from './Formula'

export default function MoleculeHeader({ compound }: { compound: Compound }) {
  return (
    <div className="molecule-header">
      <h2>{compound.title}</h2>
      <dl className="facts">
        <div>
          <dt>Formula</dt>
          <dd>
            <Formula formula={compound.formula} />
          </dd>
        </div>
        <div>
          <dt>Molar mass</dt>
          <dd>{compound.molarMass} g/mol</dd>
        </div>
        {compound.cas && (
          <div>
            <dt>CAS</dt>
            <dd>{compound.cas}</dd>
          </div>
        )}
        <div>
          <dt>PubChem</dt>
          <dd>CID {compound.cid}</dd>
        </div>
      </dl>
      {compound.synonyms.length > 0 && (
        <p className="synonyms">
          <span className="muted">Also called </span>
          {compound.synonyms.join(', ')}
        </p>
      )}
    </div>
  )
}

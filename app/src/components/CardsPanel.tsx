// Right panel: fact cards, streamed in one by one. A card's text appears as
// the model writes it, marked as unchecked until it passes the checks against
// its sources; cards that fail twice disappear.

import type { Card, CardKind, ModelStatus } from '../core'
import { openUrl } from '../core'
import type { CardSlot, MoleculeState } from '../useMolecule'

const LABELS: Record<CardKind, string> = {
  where: 'Where you meet it',
  origin: 'Origin story',
  danger: 'Danger level',
  weird: 'Weird fact',
}

function progress(slot: CardSlot, model?: ModelStatus): string {
  switch (slot.state) {
    case 'waiting':
      return model?.state === 'ready' ? 'Waiting its turn…' : 'Waiting for the model to load…'
    case 'writing':
      return slot.draft ? 'Writing…' : 'Reading the sources…'
    case 'checking':
      return 'Checking each sentence against the sources…'
    case 'retrying':
      return 'Rewriting: the first draft said more than its sources…'
    case 'ready':
      return ''
  }
}

/** Official GHS statements, shown as PubChem gives them. */
function Hazards({ hazards }: { hazards: NonNullable<Card['hazards']> }) {
  if (hazards.statements.length === 0) return null
  return (
    <div className="hazards">
      {hazards.pictograms.length > 0 && <p className="pictograms">{hazards.pictograms.join(' · ')}</p>}
      <ul>
        {hazards.statements.map((s) => (
          <li key={s.code}>
            <span className="code">{s.code}</span> {s.text}
            {s.percent != null && <span className="muted"> ({s.percent}% of reports)</span>}
          </li>
        ))}
      </ul>
    </div>
  )
}

function CardView({ slot, model }: { slot: CardSlot; model?: ModelStatus }) {
  const ready = slot.state === 'ready' && slot.card
  const title = slot.card?.title ?? slot.draft?.title
  const body = slot.card?.body ?? slot.draft?.body
  return (
    <article className={`card card-${slot.kind}${ready ? '' : ' card-pending'}`} aria-busy={!ready}>
      <h3 className="card-label">{LABELS[slot.kind]}</h3>
      {title ? <h4 className="card-title">{title}</h4> : <div className="line line-title" />}
      {body ? <p className="card-body">{body}</p> : !ready && <div className="line" />}
      {slot.card?.hazards && <Hazards hazards={slot.card.hazards} />}
      {ready ? (
        <p className="sources">
          {slot.card!.sources.map((s) => (
            <a
              key={s.url}
              href={s.url}
              onClick={(e) => {
                e.preventDefault()
                openUrl(s.url)
              }}
            >
              {s.label}
            </a>
          ))}
        </p>
      ) : (
        <p className="card-progress muted" role="status">
          {progress(slot, model)}
        </p>
      )}
    </article>
  )
}

export default function CardsPanel({ molecule, model }: { molecule?: MoleculeState; model?: ModelStatus }) {
  if (!molecule || molecule.status === 'error') {
    return (
      <section className="cards-panel" aria-label="Fact cards">
        <div className="empty">
          <h1>Molecule fun facts, grounded in real data</h1>
          <p className="muted">
            Search a molecule to see where you meet it, how it was found, how dangerous it is, and its
            strangest detail. Every card cites its sources.
          </p>
        </div>
      </section>
    )
  }
  const cards = molecule.cards
  return (
    <section className="cards-panel" aria-label="Fact cards">
      {!cards && <p className="muted">Gathering sources…</p>}
      {cards?.length === 0 && <p className="muted">Not enough in the sources for a fact card.</p>}
      {/* Announce finished cards only, not every streamed word. */}
      <div className="cards">
        {cards?.map((slot) => (
          <CardView key={slot.kind} slot={slot} model={model} />
        ))}
      </div>
      <div className="visually-hidden" aria-live="polite">
        {cards
          ?.filter((c) => c.state === 'ready')
          .map((c) => `${LABELS[c.kind]}: ${c.card?.title}. `)
          .join('')}
      </div>
    </section>
  )
}

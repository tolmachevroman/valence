// Right panel: fact cards, streamed in one by one (M3.6).

export default function CardsPanel() {
  return (
    <section className="cards-panel" aria-label="Fact cards">
      <div className="empty">
        <h1>Molecule fun facts, grounded in real data</h1>
        <p className="muted">
          Search a molecule to see where you meet it, how it was found, how dangerous it is, and
          its strangest detail. Every card cites its sources.
        </p>
      </div>
    </section>
  )
}

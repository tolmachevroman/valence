// Footer: model status (M3.2) and data freshness (M4.3).

export default function StatusBar({ version }: { version?: string }) {
  return (
    <footer className="status-bar">
      <span>Model: not loaded</span>
      <span className="spacer" />
      {version && <span>Valence {version}</span>}
    </footer>
  )
}

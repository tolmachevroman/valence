// Search, "Surprise me", favorites and settings. Settings arrive in M4.4,
// favorites in v0.2.

import SearchBox from './SearchBox'

export default function TopBar({
  onOpen,
  onSurprise,
}: {
  onOpen: (query: string) => void
  onSurprise: () => void
}) {
  return (
    <header className="top-bar">
      <div className="brand" aria-hidden="true">
        Valence
      </div>
      <SearchBox onOpen={onOpen} />
      <button type="button" onClick={onSurprise} title="Open a random molecule with a good story">
        Surprise me
      </button>
      <button type="button" className="icon" aria-label="Favorites" title="Favorites" disabled>
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <path d="M12 3.5l2.6 5.3 5.9.9-4.25 4.1 1 5.85L12 16.9l-5.25 2.75 1-5.85L3.5 9.7l5.9-.9z" />
        </svg>
      </button>
      <button type="button" className="icon" aria-label="Settings" title="Settings" disabled>
        <svg viewBox="0 0 24 24" aria-hidden="true">
          <circle cx="12" cy="12" r="3" />
          <path d="M12 2.5v3M12 18.5v3M2.5 12h3M18.5 12h3M5.3 5.3l2.1 2.1M16.6 16.6l2.1 2.1M5.3 18.7l2.1-2.1M16.6 7.4l2.1-2.1" />
        </svg>
      </button>
    </header>
  )
}

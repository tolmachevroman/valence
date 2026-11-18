// Footer: model status, and when the open molecule's data was fetched.

import { gigabytes, type ModelStatus } from '../core'

/** Short footer text, and a longer explanation for the tooltip and VoiceOver. */
function describe(model?: ModelStatus): [string, string] {
  switch (model?.state) {
    case undefined:
    case 'loading':
      return ['Model: loading…', 'The fact-card model is loading. Search and 3D work meanwhile.']
    case 'ready':
      return ['Model: ready', `The fact-card model loaded in ${model.loadSeconds.toFixed(1)} s.`]
    case 'notDownloaded':
      return ['Model: not downloaded', 'Fact cards need the model; search, 3D and hazards work without it.']
    case 'downloading': {
      const percent = Math.floor((100 * model.done) / model.total)
      return [`Model: downloading ${percent}%`, `Downloading the model: ${gigabytes(model.done)} of ${gigabytes(model.total)}.`]
    }
    case 'notEnoughMemory':
      return [
        'Model: not enough free memory',
        'The fact-card model needs about 4 GB of free memory. Quit some apps and restart Valence to try again.',
      ]
    case 'failed':
      return ['Model: failed to load', model.message]
  }
}

/** Data older than this is fetched again after it is shown (FRESH_FOR in the core). */
const FRESH_DAYS = 90

function fetched(at: number): [string, string] {
  const date = new Date(at * 1000).toLocaleDateString(undefined, { day: 'numeric', month: 'short', year: 'numeric' })
  return [
    `Data from ${date}`,
    `PubChem and Wikipedia data fetched on ${date}. Data older than ${FRESH_DAYS} days is fetched again when you open the molecule.`,
  ]
}

export default function StatusBar({
  version,
  model,
  fetchedAt,
}: {
  version?: string
  model?: ModelStatus
  /** The open molecule's data, Unix seconds. */
  fetchedAt?: number
}) {
  const [text, detail] = describe(model)
  const data = fetchedAt != null ? fetched(fetchedAt) : undefined
  return (
    <footer className="status-bar">
      <span className={`model model-${model?.state ?? 'loading'}`} title={detail} role="status" aria-label={detail}>
        {text}
      </span>
      {data && (
        <span className="fetched" title={data[1]} aria-label={data[1]}>
          {data[0]}
        </span>
      )}
      <span className="spacer" />
      {version && <span>Valence {version}</span>}
    </footer>
  )
}

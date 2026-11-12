// Footer: model status, and data freshness (M4.3).

import type { ModelStatus } from '../core'

/** Short footer text, and a longer explanation for the tooltip and VoiceOver. */
function describe(model?: ModelStatus): [string, string] {
  switch (model?.state) {
    case undefined:
    case 'loading':
      return ['Model: loading…', 'The fact-card model is loading. Search and 3D work meanwhile.']
    case 'ready':
      return ['Model: ready', `The fact-card model loaded in ${model.loadSeconds.toFixed(1)} s.`]
    case 'notDownloaded':
      return ['Model: not downloaded', `No model file at ${model.path}. Cards need it; search, 3D and hazards work without it.`]
    case 'notEnoughMemory':
      return [
        'Model: not enough free memory',
        'The fact-card model needs about 4 GB of free memory. Quit some apps and restart Valence to try again.',
      ]
    case 'failed':
      return ['Model: failed to load', model.message]
  }
}

export default function StatusBar({ version, model }: { version?: string; model?: ModelStatus }) {
  const [text, detail] = describe(model)
  return (
    <footer className="status-bar">
      <span className={`model model-${model?.state ?? 'loading'}`} title={detail} role="status" aria-label={detail}>
        {text}
      </span>
      <span className="spacer" />
      {version && <span>Valence {version}</span>}
    </footer>
  )
}

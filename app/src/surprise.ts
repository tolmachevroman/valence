// "Surprise me": a random molecule from the bundled list, which
// `valence curate` writes (molecules with an origin story and 3+ cards).

import curated from './curated.json'

export interface Pick {
  cid: number
  name: string
}

/** A random molecule from the list, never the one open now. */
export function surprise(openCid?: number): Pick {
  const choices = curated.filter((m) => m.cid !== openCid)
  return choices[Math.floor(Math.random() * choices.length)]
}

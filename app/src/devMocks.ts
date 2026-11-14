// Dev only: a stand-in core for running the UI in a plain browser
// (`npm run dev`), so layout work needs no Rust build. Serves recorded data
// for caffeine (3D) and buckminsterfullerene (2D only) with realistic delays.
// `?q=caffeine` opens a molecule on load; `?model=missing|memory|failed`
// picks how the model load ends. Never part of a production build.

import { mockIPC } from '@tauri-apps/api/mocks'

import caffeineSdf from '../../fixtures/http/molecules/caffeine/sdf-3d.sdf?raw'
import c60Png from '../../fixtures/http/molecules/buckminsterfullerene/png.png?inline'
import type { Card, CardKind, Compound, Ghs, ModelStatus, MoleculeEvent } from './core'

interface Recorded {
  compound: Compound
  sdf: string | null
  png: string | null
  ghs: Ghs | null
  /** In writing order; `null` is a card that fails its checks twice. */
  cards: [CardKind, Card | null][]
}

const WIKI = 'https://en.wikipedia.org/wiki/'
const PUBCHEM = 'https://pubchem.ncbi.nlm.nih.gov/compound/'

function card(kind: CardKind, label: string, title: string, body: string, sources: [string, string][]): Card {
  return { kind, label, title, body, sources: sources.map(([label, url]) => ({ label, url })), hazards: null }
}

// Written by the real pipeline (`valence cards caffeine`), plus one dropped card.
const CAFFEINE_CARDS: [CardKind, Card | null][] = [
  [
    'danger',
    {
      ...card(
        'danger',
        'Danger level',
        'Official signal word: Warning',
        'Caffeine carries "Warning", the milder of the two GHS signal words, used for the less severe hazard categories.',
        [['PubChem, GHS Classification', `${PUBCHEM}2519#section=GHS-Classification`]],
      ),
      hazards: {
        pictograms: ['Irritant'],
        statements: [{ code: 'H302', text: 'Harmful if swallowed', percent: null, level: 'Warning' }],
      },
    },
  ],
  ['origin', null],
  [
    'weird',
    card(
      'weird',
      'Weird fact',
      'Caffeine helps premature babies gain weight',
      'Caffeine is used to treat bronchopulmonary dysplasia in premature infants. It may improve weight gain and reduce risks of cerebral palsy and cognitive delay. Subtle long-term side effects are possible.',
      [['Wikipedia, Uses', `${WIKI}Caffeine#Uses`]],
    ),
  ],
]

const RECORDED: Record<string, Recorded> = {
  caffeine: {
    compound: {
      cid: 2519,
      title: 'Caffeine',
      formula: 'C8H10N4O2',
      molarMass: '194.19',
      iupacName: '1,3,7-trimethylpurine-2,6-dione',
      cas: '58-08-2',
      synonyms: ['Guaranine', 'Methyltheobromine', 'Theine', 'Thein', 'Cafeina'],
    },
    sdf: caffeineSdf,
    png: null,
    ghs: {
      signal: 'Warning',
      pictograms: ['Irritant'],
      statements: [{ code: 'H302', text: 'Harmful if swallowed', percent: null, level: 'Warning' }],
    },
    cards: CAFFEINE_CARDS,
  },
  buckminsterfullerene: {
    compound: {
      cid: 123591,
      title: 'Buckminsterfullerene',
      formula: 'C60',
      molarMass: '720.6',
      iupacName: null,
      cas: '99685-96-8',
      synonyms: ['Buckyball', 'Fullerene C60'],
    },
    sdf: null,
    png: c60Png,
    ghs: {
      signal: 'Warning',
      pictograms: ['Irritant'],
      statements: [
        { code: 'H315', text: 'Causes skin irritation', percent: 100, level: 'Warning' },
        { code: 'H319', text: 'Causes serious eye irritation', percent: 100, level: 'Warning' },
        { code: 'H335', text: 'May cause respiratory irritation', percent: 100, level: 'Warning' },
      ],
    },
    cards: [],
  },
}

const NAMES = ['caffeine', 'caffeic acid', 'buckminsterfullerene', 'capsaicin', 'carvone']

const sleep = (ms: number) => new Promise((r) => setTimeout(r, ms))

interface Internals {
  runCallback: (id: number, data: unknown) => void
}

/** Sends `message` to the frontend Channel passed as a command argument. */
function sender<T>(channelArg: unknown) {
  const { id } = channelArg as { id: number }
  const internals = (window as unknown as { __TAURI_INTERNALS__: Internals }).__TAURI_INTERNALS__
  let index = 0
  return (message: T) => internals.runCallback(id, { message, index: index++ })
}

function modelOutcome(): ModelStatus {
  switch (new URLSearchParams(location.search).get('model')) {
    case 'missing':
      return { state: 'notDownloaded', path: '~/Library/Application Support/…/models/model.gguf' }
    case 'memory':
      return { state: 'notEnoughMemory' }
    case 'failed':
      return { state: 'failed', message: 'unsupported GGUF architecture (dev mock)' }
    default:
      return { state: 'ready', loadSeconds: 2.4 }
  }
}

/** Cards as the pipeline sends them: drafts word by word, a check, and for a
 * failing card a retry and then nothing. */
async function streamCards(cards: [CardKind, Card | null][], send: (e: MoleculeEvent) => void) {
  send({ kind: 'cardsPlanned', cards: cards.map(([kind]) => kind) })
  for (const [kind, ready] of cards) {
    if (kind === 'danger' && ready) {
      send({ kind: 'cardReady', card: ready })
      continue
    }
    send({ kind: 'cardWriting', card: kind })
    await sleep(1500)
    const text = ready ?? card(kind, '', 'A draft that says too much', 'A first sentence from the sources. A second one that is not in them.', [])
    for (const attempt of ready ? [1] : [1, 2]) {
      if (attempt === 2) send({ kind: 'cardRetrying', card: kind })
      const words = text.body.split(' ')
      for (let i = 1; i <= words.length; i += 2) {
        send({ kind: 'cardDraft', card: kind, title: text.title, body: words.slice(0, i).join(' ') })
        await sleep(90)
      }
      await sleep(600)
    }
    send(ready ? { kind: 'cardReady', card: ready } : { kind: 'cardGone', card: kind })
  }
}

export function installDevMocks() {
  mockIPC(async (cmd, payload) => {
    const args = payload as Record<string, unknown>
    switch (cmd) {
      case 'app_version':
        return '0.1.0-dev'
      case 'autocomplete': {
        await sleep(150)
        const q = String(args.query).toLowerCase()
        return NAMES.filter((n) => n.startsWith(q))
      }
      case 'open_molecule': {
        const query = String(args.query).toLowerCase()
        const found = Object.values(RECORDED).find(
          (r) => r.compound.title.toLowerCase() === query || String(r.compound.cid) === query,
        )
        if (!found) throw `PubChem has no compound called "${args.query}" (dev mock)`
        const send = sender<MoleculeEvent>(args.onEvent)
        await sleep(300)
        send({ kind: 'header', compound: found.compound })
        await sleep(200)
        send({ kind: 'structure', sdf: found.sdf, png: found.png })
        await sleep(400)
        send({ kind: 'hazards', ghs: found.ghs })
        await sleep(600)
        send({ kind: 'done', snippetCount: 30, fetchedAt: Math.floor(Date.now() / 1000) })
        void streamCards(found.cards, send)
        return found.compound.cid
      }
      case 'model_status': {
        const send = sender<ModelStatus>(args.onChange)
        send({ state: 'loading' })
        await sleep(2400)
        send(modelOutcome())
        return null
      }
      case 'open_url':
        window.open(String(args.url), '_blank')
        return null
    }
    throw `no dev mock for ${cmd}`
  })
}

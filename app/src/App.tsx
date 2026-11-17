import { useEffect, useRef, useState } from 'react'

import CardsPanel from './components/CardsPanel'
import MoleculePanel from './components/MoleculePanel'
import StatusBar from './components/StatusBar'
import TopBar from './components/TopBar'
import { appVersion, type ModelStatus, watchModel } from './core'
import { surprise } from './surprise'
import { useMolecule } from './useMolecule'

export default function App() {
  const [version, setVersion] = useState<string>()
  const [model, setModel] = useState<ModelStatus>()
  const { molecule, open } = useMolecule()

  useEffect(() => {
    appVersion().then(setVersion)
    watchModel(setModel)
  }, [])

  // A molecule opened before the model was downloaded has no cards waiting
  // for it; once the model is ready, open it again to write them.
  const wasMissing = useRef(false)
  const shown = useRef(molecule)
  useEffect(() => {
    shown.current = molecule
  }, [molecule])
  useEffect(() => {
    if (model?.state === 'notDownloaded') wasMissing.current = true
    if (model?.state !== 'ready' || !wasMissing.current) return
    wasMissing.current = false
    const m = shown.current
    if (m?.compound && !m.cards?.some((c) => c.state !== 'ready')) open(String(m.compound.cid), m.compound.title)
  }, [model, open])

  // Dev only: `?q=caffeine` opens a molecule on load, for screenshots.
  useEffect(() => {
    const q = import.meta.env.DEV && new URLSearchParams(location.search).get('q')
    if (q) open(q)
  }, [open])

  return (
    <div className="app">
      <TopBar
        onOpen={open}
        onSurprise={() => {
          const pick = surprise(molecule?.compound?.cid)
          open(String(pick.cid), pick.name)
        }}
      />
      <main className="panels">
        <MoleculePanel molecule={molecule} />
        <CardsPanel molecule={molecule} model={model} />
      </main>
      <StatusBar version={version} model={model} />
    </div>
  )
}

import { useEffect, useRef, useState } from 'react'

import CardsPanel from './components/CardsPanel'
import SettingsDialog from './components/SettingsDialog'
import MoleculePanel from './components/MoleculePanel'
import StatusBar from './components/StatusBar'
import TopBar from './components/TopBar'
import { appVersion, type DialogTab, type ModelStatus, onShowDialog, watchModel } from './core'
import { surprise } from './surprise'
import { useMolecule } from './useMolecule'

/** Dev only: `?dialog=settings|about` opens the dialog on load, for screenshots. */
function devDialog(): DialogTab | undefined {
  const tab = import.meta.env.DEV ? new URLSearchParams(location.search).get('dialog') : null
  return tab === 'settings' || tab === 'about' ? tab : undefined
}

export default function App() {
  const [version, setVersion] = useState<string>()
  const [model, setModel] = useState<ModelStatus>()
  const { molecule, open } = useMolecule()

  const [dialog, setDialog] = useState<DialogTab | undefined>(devDialog)

  useEffect(() => {
    appVersion().then(setVersion)
    watchModel(setModel)
    const unlisten = onShowDialog(setDialog)
    return () => {
      unlisten.then((stop) => stop())
    }
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
        onSettings={() => setDialog('settings')}
      />
      <main className="panels">
        <MoleculePanel molecule={molecule} />
        <CardsPanel molecule={molecule} model={model} />
      </main>
      <StatusBar version={version} model={model} fetchedAt={molecule?.fetchedAt} />
      <SettingsDialog
        tab={dialog}
        onTab={setDialog}
        onClose={() => setDialog(undefined)}
        version={version}
        model={model}
      />
    </div>
  )
}

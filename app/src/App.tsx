import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'

import CardsPanel from './components/CardsPanel'
import MoleculePanel from './components/MoleculePanel'
import StatusBar from './components/StatusBar'
import TopBar from './components/TopBar'

export default function App() {
  const [version, setVersion] = useState<string>()

  useEffect(() => {
    invoke<string>('app_version').then(setVersion)
  }, [])

  return (
    <div className="app">
      <TopBar />
      <main className="panels">
        <MoleculePanel />
        <CardsPanel />
      </main>
      <StatusBar version={version} />
    </div>
  )
}

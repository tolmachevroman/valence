import { useEffect, useState } from 'react'

import CardsPanel from './components/CardsPanel'
import MoleculePanel from './components/MoleculePanel'
import StatusBar from './components/StatusBar'
import TopBar from './components/TopBar'
import { appVersion } from './core'

export default function App() {
  const [version, setVersion] = useState<string>()

  useEffect(() => {
    appVersion().then(setVersion)
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

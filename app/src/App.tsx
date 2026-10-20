import { invoke } from '@tauri-apps/api/core'
import { useEffect, useState } from 'react'

export default function App() {
  const [version, setVersion] = useState<string>()

  useEffect(() => {
    invoke<string>('app_version').then(setVersion)
  }, [])

  return (
    <>
      <main>
        <h1>Valence</h1>
        <p>Molecule fun facts, grounded in real chemistry data.</p>
      </main>
      <footer>{version && `Valence ${version}`}</footer>
    </>
  )
}

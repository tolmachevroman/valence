import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'

// In a plain browser during development there is no Rust core: stand in for it.
if (import.meta.env.DEV && !('__TAURI_INTERNALS__' in window)) {
  const { installDevMocks } = await import('./devMocks')
  installDevMocks()
}

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)

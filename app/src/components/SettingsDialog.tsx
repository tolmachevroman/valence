// Settings and About, one dialog with two tabs. Opened by the gear button and
// by the app menu's "About Valence" and "Settings…". Settings are edited in
// settings.toml; this shows what is in effect and where the files are.

import { useEffect, useId, useRef, useState } from 'react'

import { type DialogTab, gigabytes, type ModelStatus, openUrl, type SettingsInfo, settingsInfo, showFile } from '../core'

function Link({ href, children }: { href: string; children: React.ReactNode }) {
  return (
    <a
      href={href}
      onClick={(e) => {
        e.preventDefault()
        openUrl(href)
      }}
    >
      {children}
    </a>
  )
}

function modelState(model?: ModelStatus): string {
  switch (model?.state) {
    case undefined:
    case 'loading':
      return 'Loading'
    case 'ready':
      return 'Ready'
    case 'notDownloaded':
      return 'Not downloaded'
    case 'downloading':
      return `Downloading, ${Math.floor((100 * model.done) / model.total)}%`
    case 'notEnoughMemory':
      return 'Not enough free memory'
    case 'failed':
      return `Failed: ${model.message}`
  }
}

function SettingsTab({ info, model }: { info?: SettingsInfo; model?: ModelStatus }) {
  const [error, setError] = useState<string>()
  if (!info) return <p className="muted">Loading…</p>
  const show = (which: 'model' | 'settings') => showFile(which).catch((e: unknown) => setError(String(e)))
  return (
    <>
      <section>
        <h3>Model</h3>
        <dl className="facts">
          <dt>File</dt>
          <dd>{info.modelFile}</dd>
          <dt>Size</dt>
          <dd>{info.modelBytes != null ? gigabytes(info.modelBytes) : 'Not downloaded'}</dd>
          <dt>Status</dt>
          <dd>{modelState(model)}</dd>
        </dl>
        <p className="path">{info.modelPath}</p>
        {!info.modelIsDefault && (
          <p className="dialog-warn">
            Not the default model. Only the default, Qwen3 4B Instruct 2507, is tested for accurate cards; with this
            one, cards may choose sentences less well.
          </p>
        )}
        <button type="button" onClick={() => show('model')}>
          Show in Finder
        </button>
      </section>
      <section>
        <h3>Settings file</h3>
        <p>
          Settings live in a small text file. To use another model file, set <code>model</code> there, then restart
          Valence.
        </p>
        <p className="path">{info.settingsPath}</p>
        {info.settingsError && (
          <p className="dialog-warn" role="alert">
            The file couldn't be read, so the defaults are in use: {info.settingsError}
          </p>
        )}
        <button type="button" onClick={() => show('settings')}>
          Edit settings.toml
        </button>
      </section>
      {error && (
        <p className="dialog-warn" role="alert">
          {error}
        </p>
      )}
    </>
  )
}

function AboutTab({ version }: { version?: string }) {
  return (
    <>
      <section>
        <h3>Valence {version}</h3>
        <p>Molecule fun facts, grounded in real data. Free and open source under the MIT license.</p>
        <p>
          <Link href="https://github.com/tolmachevroman/valence">github.com/tolmachevroman/valence</Link>
        </p>
      </section>
      <section>
        <h3>Data</h3>
        <p>
          Compound data, annotations and hazard classifications come from{' '}
          <Link href="https://pubchem.ncbi.nlm.nih.gov/">PubChem</Link>, National Library of Medicine. Many annotations
          come from PubChem's contributing sources, credited on the PubChem pages that cards link to. See PubChem's{' '}
          <Link href="https://pubchem.ncbi.nlm.nih.gov/docs/citation-guidelines">citation guidelines</Link>.
        </p>
        <p>
          Article text comes from <Link href="https://en.wikipedia.org/">Wikipedia</Link>, available under{' '}
          <Link href="https://creativecommons.org/licenses/by-sa/4.0/">CC BY-SA 4.0</Link>. Cards quote it word for
          word and link to the article; that text stays under the same license.
        </p>
      </section>
      <section>
        <h3>Model</h3>
        <p>
          <Link href="https://huggingface.co/Qwen/Qwen3-4B-Instruct-2507">Qwen3 4B Instruct 2507</Link> by the Qwen
          team, under the Apache 2.0 license, in a GGUF build by Unsloth. It runs on this Mac only.
        </p>
      </section>
      <section>
        <h3>Built with</h3>
        <p>
          <Link href="https://tauri.app/">Tauri</Link> (MIT or Apache 2.0),{' '}
          <Link href="https://github.com/EricLBuehler/mistral.rs">mistral.rs</Link> (MIT),{' '}
          <Link href="https://3dmol.org/">3Dmol.js</Link> (BSD 3-Clause) and{' '}
          <Link href="https://react.dev/">React</Link> (MIT).
        </p>
      </section>
    </>
  )
}

export default function SettingsDialog({
  tab,
  onTab,
  onClose,
  version,
  model,
}: {
  /** The open tab, or undefined when the dialog is closed. */
  tab?: DialogTab
  onTab: (tab: DialogTab) => void
  onClose: () => void
  version?: string
  model?: ModelStatus
}) {
  const dialog = useRef<HTMLDialogElement>(null)
  const [info, setInfo] = useState<SettingsInfo>()
  const id = useId()
  const isOpen = tab !== undefined

  useEffect(() => {
    const d = dialog.current
    if (!d) return
    if (isOpen && !d.open) {
      d.showModal()
      // showModal focuses the first tab; the open one is where focus belongs.
      d.querySelector<HTMLElement>('[role="tab"][aria-selected="true"]')?.focus()
      settingsInfo().then(setInfo)
    }
    if (!isOpen && d.open) d.close()
  }, [isOpen])

  const tabs: [DialogTab, string][] = [
    ['settings', 'Settings'],
    ['about', 'About'],
  ]
  return (
    <dialog ref={dialog} className="dialog" aria-labelledby={`${id}-title`} onClose={onClose}>
      <header className="dialog-header">
        <h2 id={`${id}-title`} className="visually-hidden">
          Settings and About
        </h2>
        <div role="tablist" className="tabs">
          {tabs.map(([key, label]) => (
            <button
              key={key}
              type="button"
              role="tab"
              id={`${id}-${key}`}
              aria-selected={tab === key}
              aria-controls={`${id}-panel`}
              tabIndex={tab === key ? 0 : -1}
              onClick={() => onTab(key)}
              onKeyDown={(e) => {
                if (e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
                  const next = tabs[(tabs.findIndex(([k]) => k === key) + 1) % tabs.length][0]
                  onTab(next)
                  document.getElementById(`${id}-${next}`)?.focus()
                }
              }}
            >
              {label}
            </button>
          ))}
        </div>
        <button type="button" className="icon" aria-label="Close" title="Close" onClick={onClose}>
          <svg viewBox="0 0 24 24" aria-hidden="true">
            <path d="M6 6l12 12M18 6L6 18" />
          </svg>
        </button>
      </header>
      <div className="dialog-body" role="tabpanel" id={`${id}-panel`} aria-labelledby={`${id}-${tab}`}>
        {tab === 'settings' && <SettingsTab info={info} model={model} />}
        {tab === 'about' && <AboutTab version={version} />}
      </div>
    </dialog>
  )
}

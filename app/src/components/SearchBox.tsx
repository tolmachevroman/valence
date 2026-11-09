// Molecule search: an ARIA 1.2 combobox with PubChem autocomplete.
// Arrow keys move through suggestions, Enter opens the highlighted one (or
// what was typed: a name, CAS number or CID), Escape closes the list.

import { useEffect, useId, useState } from 'react'

import { autocomplete } from '../core'

const DEBOUNCE_MS = 120
const MIN_CHARS = 2

export default function SearchBox({ onOpen }: { onOpen: (query: string) => void }) {
  const [text, setText] = useState('')
  // Replies by query; what's shown is the reply for what's typed now, so a
  // late reply to an older query never shows.
  const [found, setFound] = useState<Record<string, string[]>>({})
  const [open, setOpen] = useState(false)
  const [active, setActive] = useState(-1)
  const listId = useId()

  const query = text.trim().toLowerCase()
  const ready = query.length >= MIN_CHARS
  const suggestions = (ready && found[query]) || []
  const known = ready && query in found

  useEffect(() => {
    if (!ready || known) return
    const timer = setTimeout(() => {
      // A failed lookup isn't stored, so it is tried again later.
      autocomplete(query)
        .then((list) => setFound((f) => ({ ...f, [query]: list })))
        .catch(() => {})
    }, DEBOUNCE_MS)
    return () => clearTimeout(timer)
  }, [query, ready, known])

  const showList = open && suggestions.length > 0

  function choose(value: string) {
    if (!value.trim()) return
    setText(value)
    setOpen(false)
    setActive(-1)
    onOpen(value.trim())
  }

  function onKeyDown(e: React.KeyboardEvent<HTMLInputElement>) {
    switch (e.key) {
      case 'ArrowDown':
        e.preventDefault()
        setOpen(true)
        setActive((i) => (suggestions.length ? (i + 1) % suggestions.length : -1))
        break
      case 'ArrowUp':
        e.preventDefault()
        setOpen(true)
        setActive((i) => (suggestions.length ? (i <= 0 ? suggestions.length - 1 : i - 1) : -1))
        break
      case 'Enter':
        e.preventDefault()
        choose(showList && active >= 0 ? suggestions[active] : text)
        break
      case 'Escape':
        if (showList) {
          e.preventDefault()
          setOpen(false)
          setActive(-1)
        }
        break
    }
  }

  return (
    <div className="search-box">
      <input
        className="search"
        type="text"
        role="combobox"
        aria-label="Search a molecule by name, CAS number or PubChem CID"
        aria-autocomplete="list"
        aria-expanded={showList}
        aria-controls={listId}
        aria-activedescendant={showList && active >= 0 ? `${listId}-${active}` : undefined}
        placeholder="Search a molecule: caffeine, 58-08-2, 2519"
        autoComplete="off"
        autoCorrect="off"
        spellCheck={false}
        value={text}
        onChange={(e) => {
          setText(e.target.value)
          setOpen(true)
          setActive(-1)
        }}
        onKeyDown={onKeyDown}
        onFocus={() => setOpen(true)}
        onBlur={() => setOpen(false)}
      />
      <ul id={listId} role="listbox" aria-label="Suggestions" className="suggestions" hidden={!showList}>
        {suggestions.map((s, i) => (
          <li
            key={s}
            id={`${listId}-${i}`}
            role="option"
            aria-selected={i === active}
            // mousedown, not click: the input's blur would close the list first.
            onMouseDown={(e) => {
              e.preventDefault()
              choose(s)
            }}
            onMouseEnter={() => setActive(i)}
          >
            {s}
          </li>
        ))}
      </ul>
    </div>
  )
}

// The 3D model (3Dmol.js, WebGL): ball-and-stick, drag to rotate, scroll to
// zoom, fitted to the panel. The background follows the theme.

import * as $3Dmol from '3dmol'
import { useEffect, useRef } from 'react'

const STYLE = {
  stick: { radius: 0.14, colorscheme: 'Jmol' },
  sphere: { scale: 0.24, colorscheme: 'Jmol' },
}

const DARK = window.matchMedia('(prefers-color-scheme: dark)')

/** The panel's surface color, so the canvas blends in. */
function surface(element: HTMLElement): string {
  return getComputedStyle(element).getPropertyValue('--surface').trim() || '#f6f5f8'
}

export default function Viewer({ sdf, title }: { sdf: string; title: string }) {
  const element = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const el = element.current
    if (!el) return
    const viewer = $3Dmol.createViewer(el, { backgroundColor: surface(el), antialias: true })
    viewer.addModel(sdf, 'sdf')
    viewer.setStyle({}, STYLE)
    viewer.zoomTo()
    viewer.render()

    const resize = new ResizeObserver(() => {
      viewer.resize()
      viewer.render()
    })
    resize.observe(el)
    const retheme = () => {
      viewer.setBackgroundColor(surface(el), 1)
      viewer.render()
    }
    DARK.addEventListener('change', retheme)
    return () => {
      DARK.removeEventListener('change', retheme)
      resize.disconnect()
      viewer.clear()
      el.replaceChildren()
    }
  }, [sdf])

  return (
    <div
      ref={element}
      className="viewer-canvas"
      role="img"
      aria-label={`3D ball-and-stick model of ${title}. Drag to rotate, scroll to zoom.`}
    />
  )
}

import react from '@vitejs/plugin-react'
import { defineConfig } from 'vite'

// https://v2.tauri.app/start/frontend/vite/
export default defineConfig({
  plugins: [react()],
  // Keep Rust compiler output visible in the terminal
  clearScreen: false,
  server: {
    port: 5173,
    strictPort: true,
    watch: { ignored: ['**/src-tauri/**'] },
    // The dev mocks read recorded data from ../fixtures.
    fs: { allow: ['..'] },
  },
})

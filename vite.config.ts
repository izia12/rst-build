import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  worker: {
    format: 'es'
  },
  build: {
    rollupOptions: {
      output: {
        manualChunks: {
          three: ['three'],
          react: ['react', 'react-dom'],
          controls: ['three/examples/jsm/controls/OrbitControls']
        }
      }
    },
    chunkSizeWarningLimit: 1500
  }
})

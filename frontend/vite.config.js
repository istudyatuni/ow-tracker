import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [
    svelte(),
    VitePWA({
      registerType: 'autoUpdate',
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico}'],
      },
      includeAssets: ['*.json', 'translations/*.json', 'sprites/*.jpg'],
    }),
  ],
  base: '/ow-tracker',
  build: {
    sourcemap: true,
    rollupOptions: {
      treeshake: true,
    },
  },
})

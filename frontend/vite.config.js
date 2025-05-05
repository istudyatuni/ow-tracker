import { defineConfig } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { VitePWA } from 'vite-plugin-pwa'
import Icons from 'unplugin-icons/vite'

export default defineConfig({
  plugins: [
    svelte(),
    VitePWA({
      registerType: 'autoUpdate',
      workbox: {
        globPatterns: ['**/*.{js,css,html,ico}'],
      },
      includeAssets: ['*.json', 'translations/*.json', 'translations/ui/*.ftl', 'sprites/*.jpg'],
    }),
    Icons({ compiler: 'svelte' }),
  ],
  base: '/ow-tracker',
  build: {
    sourcemap: true,
    rollupOptions: {
      treeshake: true,
    },
  },
})

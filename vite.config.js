import { defineConfig } from 'vite'
import ViteRsw from 'vite-plugin-rsw'

export default defineConfig({
  base: "/vite_nannou_template/",
  plugins: [
    ViteRsw(),
  ],
})

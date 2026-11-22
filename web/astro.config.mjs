// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';
import { fileURLToPath } from 'node:url';

const monacoEsm = fileURLToPath(new URL('./node_modules/monaco-editor/esm/vs/', import.meta.url));

export default defineConfig({
  integrations: [react()],
  server: { port: 4321 },
  vite: {
    plugins: [tailwindcss()],
    resolve: {
      // monaco-vim importa rutas internas antiguas de Monaco (monaco-editor/esm/vs/...)
      // que el campo "exports" de Monaco 0.57 ya no expone: las redirigimos al archivo real.
      alias: [{ find: /^monaco-editor\/esm\/vs\/(.*)$/, replacement: `${monacoEsm}$1.js` }],
    },
    server: {
      // La API (compilar + simular) la sirve mir-server en Rust.
      proxy: { '/api': 'http://127.0.0.1:8787', '/auth': 'http://127.0.0.1:8787' },
    },
  },
});

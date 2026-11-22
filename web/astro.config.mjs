// @ts-check
import { defineConfig } from 'astro/config';
import react from '@astrojs/react';
import tailwindcss from '@tailwindcss/vite';

export default defineConfig({
  integrations: [react()],
  server: { port: 4321 },
  vite: {
    plugins: [tailwindcss()],
    server: {
      // La API (compilar + simular) la sirve mir-server en Rust.
      proxy: { '/api': 'http://127.0.0.1:8787', '/auth': 'http://127.0.0.1:8787' },
    },
  },
});

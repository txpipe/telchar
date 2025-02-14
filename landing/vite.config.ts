import { reactRouter } from '@react-router/dev/vite';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';
import tsconfigPaths from 'vite-tsconfig-paths';
import { reactRouterDevTools } from 'react-router-devtools';

export default defineConfig({
  server: {
    host: '0.0.0.0',
    port: 3001,
  },
  plugins: [
    reactRouterDevTools(),
    tailwindcss(),
    reactRouter(),
    tsconfigPaths(),
  ],
});

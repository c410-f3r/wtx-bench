import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
  assetsInclude: ['**/*.gzip'],
  build: {
    rollupOptions: {
      output: {
        manualChunks: (id) => {
          if (id.includes('chart.js')) {
            return 'index_chartjs';
          }
          return 'index';
        }
      }
    }
  },
  plugins: [sveltekit()]
});

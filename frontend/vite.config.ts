import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig, loadEnv } from 'vite';

export default defineConfig(({ mode }) => {
  const apiTarget = loadEnv(mode, '.', '').API_PROXY_TARGET || 'http://127.0.0.1:9000';
  return {
    plugins: [sveltekit()],
    server: {
      proxy: {
        '/api': apiTarget,
        '/uploads': apiTarget
      }
    }
  };
});

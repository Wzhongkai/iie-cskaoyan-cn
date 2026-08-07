import adapter from '@sveltejs/adapter-node';

/** @type {import('@sveltejs/kit').Config} */
const config = {
  kit: {
    adapter: adapter(),
    alias: {
      '$components': './src/lib/components',
      '$server': './src/lib/server'
    }
  }
};

export default config;

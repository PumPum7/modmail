import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import ClosePlugin from './vite-plugin-close';

export default defineConfig({
	plugins: [sveltekit(), ClosePlugin()],
	server: {
		host: '0.0.0.0',
		port: 5173,
		hmr: {
			host: '0.0.0.0',
			port: 5173,
			protocol: 'ws'
		}
	}
});

import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				changeOrigin: true,
				target: 'http://localhost:3000',
			},
		},
	},
});

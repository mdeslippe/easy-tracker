// Vite.
import react from '@vitejs/plugin-react';

// Vitest.
import { defineConfig } from 'vitest/config';

// Vite PWA.
import { VitePWA } from 'vite-plugin-pwa';

// Path.
import path from 'path';

/**
 * The vite configuration for the website.
 */
export default defineConfig({
	plugins: [react(), VitePWA({ registerType: 'autoUpdate' })],
	test: {
		globals: true,
		environment: 'jsdom'
	},
	resolve: {
		alias: [
			{ find: '@website', replacement: path.resolve(__dirname, 'src') }
		]
	}
});

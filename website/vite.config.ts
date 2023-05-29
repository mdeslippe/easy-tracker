// Vite.
import react from '@vitejs/plugin-react';

// Vitest.
import { defineConfig } from 'vitest/config';

// Vite PWA.
import { VitePWA } from 'vite-plugin-pwa';

// Vite basic ssl.
import basicSsl from '@vitejs/plugin-basic-ssl';

// Path.
import path from 'path';

/**
 * The vite configuration for the website.
 */
export default defineConfig({
	plugins: [basicSsl(), react(), VitePWA({ registerType: 'autoUpdate' })],
	server: {
		https: true,
		host: 'localhost',
		port: 5000
	},
	test: {
		globals: true,
		environment: 'jsdom'
	},
	resolve: {
		alias: [{ find: '@website', replacement: path.resolve(__dirname, 'src') }]
	}
});

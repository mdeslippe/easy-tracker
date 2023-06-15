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
	plugins: [
		basicSsl(),
		react(),
		VitePWA({
			registerType: 'autoUpdate',
			strategies: 'generateSW',
			manifest: {
				name: 'Easy Tracker',
				short_name: 'Easy Tracker',
				display: 'standalone',
				start_url: '/',
				scope: '/',
				lang: 'en',
				description:
					'A web-based utility that enables users to easily monitor the current and historical status of their digital services',
				theme_color: '#f2994a',
				background_color: '#1d212e',
				icons: [
					{
						src: '/android-chrome-192x192.png',
						sizes: '192x192',
						type: 'image/png'
					},
					{
						src: '/android-chrome-512x512.png',
						sizes: '512x512',
						type: 'image/png'
					}
				]
			}
		})
	],
	server: {
		https: true,
		host: 'localhost',
		port: 5000
	},
	preview: {
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

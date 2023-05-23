// Vite.
import react from '@vitejs/plugin-react';

// Vitest.
import { defineConfig } from 'vitest/config';

// Path.
import path from 'path';

/**
 * The vite configuration for the website.
 */
export default defineConfig({
	plugins: [react()],
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

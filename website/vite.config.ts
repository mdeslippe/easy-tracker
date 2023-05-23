// Vite.
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// Path.
import path from 'path';

/**
 * The vite configuration for the website.
 */
export default defineConfig({
  plugins: [react()],
  resolve: {
		alias: [
			{ find: '@website', replacement: path.resolve(__dirname, 'src') }
		]
	}
})

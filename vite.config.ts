import path from 'path';
import { defineConfig } from 'vite';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
	root: 'front',
	clearScreen: false,
	publicDir: 'static',
	build: {
		target: 'es2024',
	},
	resolve: {
		alias: {
			'^actions': path.resolve(__dirname, './front/src/actions'),
			'^api': path.resolve(__dirname, './front/src/api'),
			'^events': path.resolve(__dirname, './front/src/app-events'),
			'^state': path.resolve(__dirname, './front/src/app-state'),
			'^ds': path.resolve(__dirname, './front/src/ui/design-system'),
			'^lib': path.resolve(__dirname, './front/src/lib')
		},
	},
	server: {
		port: 1420,
		strictPort: true,
		host: host || false,
		hmr: host
			? {
				protocol: 'ws',
				host,
				port: 1421,
			}
			: undefined,
		watch: {
			ignored: ['crates/**'],
		},
	},
}));

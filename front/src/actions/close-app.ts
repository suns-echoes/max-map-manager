import { exit } from '@tauri-apps/plugin-process';

export function closeApp(exitCode: number = 0) {
	exit(exitCode);
}

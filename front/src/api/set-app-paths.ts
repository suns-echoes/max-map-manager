import { invoke } from '@tauri-apps/api/core';


interface Paths {
	maxPath: string;
	savesPath: string;
	archivePath: string;
}


export async function setAppPaths({ maxPath, savesPath, archivePath }: Paths) {
	try {
		const result = await invoke<boolean>('set_app_paths_command', { maxPath, savesPath, archivePath });
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

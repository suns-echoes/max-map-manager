import { invoke } from '@tauri-apps/api/core';


interface Settings {
    game_dir: string;
    saves_dir: string;
    archive_dir: string;
}


export async function readSettings(): Promise<Result<Settings, string>> {
	try {
		const result = await invoke<Settings>('read_settings_command');
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

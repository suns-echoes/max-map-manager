import { invoke } from '@tauri-apps/api/core';


export async function archiveMapAndSaves(mapHashId: string): Promise<Result<string, string>> {
	try {
		const result = await invoke<string>('archive_map_and_saves_command', { mapHashId });
		return { ok: true, data: result };
	} catch (error) {
		console.error(error);
		return { ok: false, error: (error as Error).message ?? error };
	}
}

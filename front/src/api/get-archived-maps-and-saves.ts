import { invoke } from '@tauri-apps/api/core';


interface RawMapAndSaves {
	map: string;
	map_hash_id: string;
	saves: string[];
}


export async function getArchivedMapsAndSaves(): Promise<Result<RawMapAndSaves[], string>> {
	try {
		const result = await invoke<RawMapAndSaves[]>('get_archived_maps_and_saves_command');
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

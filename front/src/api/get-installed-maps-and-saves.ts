import { invoke } from '@tauri-apps/api/core';


interface MapAndSaves {
	map: string;
	map_hash_id: string;
	saves: string[];
}


export async function getInstalledMapsAndSaves(): Promise<Result<MapAndSaves[], string>> {
	try {
		const result = await invoke<MapAndSaves[]>('get_installed_maps_and_saves_command');
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

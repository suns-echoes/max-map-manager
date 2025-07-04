import { invoke } from '@tauri-apps/api/core';


interface SaveMetadata {
	version: number,
	save_type: string,
	name: string,
	map_hash_id: string,
	mission_index: number,
	current_turn: number,
	difficulty: string,
	game_mode: string,
	victory_type: string,
	victory_limit: number,
	player_color: string,
	player_name: string,
}


export async function readSaveFilesMetadata(saveFiles: string[], mapWidth: number, mapHeight: number): Promise<Result<SaveMetadata[], string>> {
	try {
		const result = await invoke<SaveMetadata[]>('read_save_files_metadata_command', {
			saveFilePaths: saveFiles,
			mapWidth,
			mapHeight,
		});
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

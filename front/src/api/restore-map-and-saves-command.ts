import { invoke } from '@tauri-apps/api/core';

import { MainViewState } from '^state/main-view-state';


export async function restoreMapAndSaves(mapHashId: MapHashId): Promise<Result<MapHashId[], string>> {
	try {
		const targetMapFileName = MainViewState.selectedPlanet.value + '_'
			+ (MainViewState.selectedMapSlotIndex.value + 1).toString()
			+ '.WRL';
		const result = await invoke<MapHashId[]>('restore_map_and_saves_command', { mapHashId, targetMapFileName });
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

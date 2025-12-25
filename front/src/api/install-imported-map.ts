import { invoke } from '@tauri-apps/api/core';


export async function installImportedMap(importPath: string, slotName: string): Promise<Result<boolean, string>> {
	try {
		const result = await invoke<boolean>('install_imported_map_command', { importPath, slotName });
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

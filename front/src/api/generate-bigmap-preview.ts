import { invoke } from '@tauri-apps/api/core';


export async function generateBigMapPreview(mapHashId: string | null, scanline: boolean): Promise<Result<Vec<u8>, string>> {
	try {
		const result = await invoke<Vec<u8>>('generate_bigmap_preview_command', {
			mapHashId,
			scanline,
		});
		return { ok: true, data: result };
	} catch (error) {
		console.error(error);
		return { ok: false, error: (error as Error).message ?? error };
	}
}

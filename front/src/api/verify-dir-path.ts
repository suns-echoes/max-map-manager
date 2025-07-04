import { invoke } from '@tauri-apps/api/core';


export async function verifyDirPath(path: string): Promise<Result<boolean, string>> {
	try {
		const result = await invoke<boolean>('verify_dir_path_command', { path });
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

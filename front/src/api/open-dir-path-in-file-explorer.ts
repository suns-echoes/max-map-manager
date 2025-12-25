import { invoke } from '@tauri-apps/api/core';


export async function openDirPathInFileExplorer(dirPath: string): Promise<Result<boolean, string>> {
	try {
		const result = await invoke<boolean>('open_dir_path_in_file_explorer_command', { path: dirPath });
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

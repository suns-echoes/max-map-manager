import { invoke } from '@tauri-apps/api/core';


export async function isSetupRequired(): Promise<Result<boolean, string>> {
	try {
		const result = await invoke<boolean>('is_setup_required_command');
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

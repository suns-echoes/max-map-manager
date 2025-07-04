import { invoke } from '@tauri-apps/api/core';


interface WRLTailHeader {
	_v: number;
	hash_id: string;
	name: string;
	version: string;
	date: string;
	author: string;
	description: string;
	comments: string;
}

interface MapMetadata {
	file_path: string;
	width: number;
	height: number;
	minimap: Vec<u8>;
	tail: WRLTailHeader;
}


export async function readArchivedMapsMetadata(): Promise<Result<MapMetadata[], string>> {
	try {
		const result = await invoke<MapMetadata[]>('read_archived_maps_metadata_command');
		return { ok: true, data: result };
	} catch (error) {
		return { ok: false, error: (error as Error).message ?? error };
	}
}

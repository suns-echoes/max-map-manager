import { api } from '^api/index.ts';


export async function putMapInArchive(mapHashId: MapHashId): Promise<boolean> {
	const result = await api.archiveMapAndSaves(mapHashId);

	if (result.ok) {
		return true;
	}

	console.error(result.error);
	return false;
}

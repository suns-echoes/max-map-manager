import { api } from '^api/index.ts';


export async function restoreMapFromArchive(mapHashId: MapHashId): Promise<boolean> {
	const result = await api.restoreMapAndSaves(mapHashId);

	if (result.ok) {
		return true;
	}

	console.error(result.error);
	return false;
}

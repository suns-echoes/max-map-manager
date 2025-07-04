import { api } from '../api/index.ts';


export async function getArchivedMapsAndSaves(): Promise<Map<MapHashId, MapAndSaves>> {
	return api.getArchivedMapsAndSaves()
		.then(result => {
			if (result.ok) {
				return new Map(result.data.map(item => [item.map_hash_id, {
					...item,
					mapHashId: item.map_hash_id,
				}]));
			}
			console.error(result.error);
			return new Map();
		});
}

import { api } from '../api/index.ts';


export async function getInstalledMapsInfo(): Promise<MapInfo[]> {
	return api.readInstalledMapsMetadata()
		.then(result => {
			if (result.ok) {
				return result.data.map(function (mapMetadata) {
					const [_, planetName, planetSlot] = mapMetadata.file_path.match(/(CRATER|GREEN|DESERT|SNOW)_([123456])\.WRL$/i) ?? [];
					if (!planetName || !planetSlot) {
						return null;
					}
					return {
						mapHashId: mapMetadata.tail.hash_id as MapHashId,
						filePath: mapMetadata.file_path,
						planetName: planetName.toUpperCase(),
						planetSlot,
						width: mapMetadata.width,
						height: mapMetadata.height,
						minimap: mapMetadata.minimap,
						name: mapMetadata.tail.name,
						description: mapMetadata.tail.description,
						author: mapMetadata.tail.author,
						version: mapMetadata.tail.version,
						date: mapMetadata.tail.date,
						comments: mapMetadata.tail.comments,
						isInstalled: true,
					} satisfies MapInfo;
				}).filter(function (mapInfo){
					return mapInfo !== null;
				}).sort(sortMapInfoByPlanetSlot);
			}
			console.error(result.error);
			return [];
		});
}


function sortMapInfoByPlanetSlot(a: MapInfo, b: MapInfo): number {
	return a.planetSlot.localeCompare(b.planetSlot);
}

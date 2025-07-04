import { api } from '../api/index.ts';


export async function getMapRenderData(selectedMapHashId: string | null, scanline: boolean = false): Promise<Vec<u8> | null> {
	return await api.generateBigMapPreview(selectedMapHashId, scanline)
		.then(result => {
			if (result.ok) {
				return result.data;
			}
			console.error(result.error);
			return null;
		});
}

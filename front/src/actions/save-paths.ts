import { api } from '../api/index';


interface Paths {
	maxPath: string;
	savesPath: string;
	archivePath: string;
}


export async function savePaths({ maxPath, savesPath, archivePath }: Paths) {
	const result = await api.setAppPaths({ maxPath, savesPath, archivePath });
	return result.ok;
}

import { api } from '../api';


export async function openDirPathInFileExplorer(dirPath: string) {
	const result = await api.openDirPathInFileExplorer(dirPath);

	if (!result.ok) {
		// TODO: show in error view
		// TODO: make error view look devastated
		console.error('Failed to open directory in file explorer:', result.error);
	}
}

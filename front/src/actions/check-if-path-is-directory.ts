import { api } from '../api';


export async function checkIfPathIsDirectory(
	path: string,
): Promise<boolean | string> {
	const result = await api.verifyDirPath(path);
	return result.ok ? result.data : result.error;
}

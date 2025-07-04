import { api } from '../api/index.ts';


export async function checkGamePath(
	path: string,
): Promise<boolean | string> {
	const result = await api.verifyGamePath(path);
	return result.ok ? result.data : false;
}

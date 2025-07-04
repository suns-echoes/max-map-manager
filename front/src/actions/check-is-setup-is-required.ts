import { api } from '../api';


export async function checkIfSetupIsRequired(): Promise<boolean> {
	const result = await api.isSetupRequired();
	if (result.ok) {
		return result.data;
	} else {
		console.error('Failed to check if setup is required:', result.error);
		return true;
	}
}

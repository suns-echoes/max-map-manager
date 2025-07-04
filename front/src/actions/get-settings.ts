import { api } from '../api';


interface Settings {
	maxPath: string;
	savesPath: string;
	archivePath: string;
}


export async function getSettings(): Promise<Settings> {
	const result = await api.readSettings();

	if (result.ok) {
		return {
			maxPath: result.data.game_dir,
			savesPath: result.data.saves_dir,
			archivePath: result.data.archive_dir,
		};
	}

	console.error(result.error);
	return {
		maxPath: '',
		savesPath: '',
		archivePath: ''
	};
}

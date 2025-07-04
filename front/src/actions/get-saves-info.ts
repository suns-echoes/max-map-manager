import { api } from '../api';


export type SaveType = (
	'Custom' |
	'Tutorial' |
	'Campaign' |
	'HotSeat' |
	'Multiplayer' |
	'Demo' |
	'Debug' |
	'Text' |
	'Scenario' |
	'MultiScenario'
);

export type Difficulty = (
	'Clueless' |
	'Apprentice' |
	'Average' |
	'Expert' |
	'Master' |
	'God'
);

export type GameMode = (
	'Turn Based' |
	'Simultaneous Moves'
);

export type VictoryType = (
	'Duration' |
	'Score'
);

export type PlayerColor = (
	'Red' |
	'Green' |
	'Blue' |
	'Gray'
);

export interface SaveInfo {
	saveType: SaveType,
	name: string,
	mapHashId: string,
	missionIndex: number,
	currentTurn: number,
	difficulty: Difficulty,
	gameMode: GameMode,
	victoryType: VictoryType,
	victoryLimit: number,
	playerColor: PlayerColor,
	playerName: string,
}

export async function getSavesInfo(saveFiles: string[], mapWidth: number, mapHeight: number): Promise<SaveInfo[]> {
	return await api.readSaveFilesMetadata(saveFiles, mapWidth, mapHeight)
		.then(result => {
			if (result.ok) {
				return result.data.map(function (data) {
					return {
						saveType: data.save_type as SaveType,
						name: data.name,
						mapHashId: data.map_hash_id,
						missionIndex: data.mission_index,
						currentTurn: data.current_turn,
						difficulty: data.difficulty as Difficulty,
						gameMode: data.game_mode as GameMode,
						victoryType: data.victory_type as VictoryType,
						victoryLimit: data.victory_limit,
						playerColor: data.player_color as PlayerColor,
						playerName: data.player_name,
					} satisfies SaveInfo;
				});
			}
			console.error(result.error);
			return [];
		});
}

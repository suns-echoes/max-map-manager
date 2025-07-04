import { Value } from '^lib/reactive/value.class';
import { HTMLNode } from '^lib/reactive/html-node.class';

import { Screen } from '^ds/screen/screen';

import { MainViewState } from './main-view-state';
import { ArchiveViewState } from './archive-view-state';
import { Expr } from '../lib/reactive/expr.class';


export interface ViewState {
	selectedMapHashId: Value<MapHashId | null>;
	selectedMapSlotIndex: Value<number>;

	name: string;

	blur(): void;
	focus(): void;

	viewScreens: HTMLNode<any, Screen>[];
}


export class AppState {
	/**
	 * Info about all maps, both installed and not installed.
	 */
	static mapsInfo = new Value<Map<MapHashId, MapInfo>>(new Map());

	/**
	 * Generic progress value that can be used to track loading or processing states by the backend.
	 */
	static progress = new Value<number>(0);

	static windowSize = new Value<Size>({
		width: window.innerWidth,
		height: window.innerHeight,
	});

	/**
	 * Update installed maps and saves info.
	 */
	static async update() {
		await Promise.all([
			MainViewState.update(),
			ArchiveViewState.update(),
		]);

		const data = new Map<MapHashId, MapInfo>([]);

		ArchiveViewState.mapsInfo.value.forEach((mapInfo, mapHashId) => {
			data.set(mapHashId, mapInfo);
		});

		MainViewState.mapsInfo.value.forEach((mapInfo, mapHashId) => {
			data.set(mapHashId, mapInfo);
		});

		AppState.mapsInfo.set(data);
	}

	private static _currentViewState: ViewState | null = null;
	private static _previousViewState: ViewState | null = null;

	static focusView(view: ViewState) {
		if (this._currentViewState !== view) {
			this._currentViewState?.blur();
			this._previousViewState = this._currentViewState;
		}
		this._currentViewState = view;
		view.focus();
		window.dispatchEvent(new CustomEvent('app-view-changed', { detail: view.name }));
	}

	static focusPreviousView() {
		if (this._previousViewState) {
			this.focusView(this._previousViewState);
			this._previousViewState = MainViewState;
		}
	}

	static saveFilesCountByType = new Expr<SaveFilesCount>(function getSaveFilesCount() {
		let count: SaveFilesCount = {
			custom: 0,
			campaign: 0,
			scenario: 0,
			multi: 0,
			hot: 0,
			other: 0,
		};
		ArchiveViewState.mapsAndSaves.value.forEach((mapAndSaves) => {
			mapAndSaves.saves.forEach((saveFile) => {
				switch (saveFile.split('.').pop()?.toUpperCase()) {
					case 'DTA':
						count.custom++;
						break;
					case 'CAM':
						count.campaign++;
						break;
					case 'SCE':
						count.scenario++;
						break;
					case 'MUL':
						count.multi++;
						break;
					case 'HOT':
						count.hot++;
						break;
					default:
						count.other++;
						break;
				}
			});
		});
		return count;
	}, {
		custom: 0,
		multi: 0,
		hot: 0,
		scenario: 0,
		campaign: 0,
		other: 0,
	}).on([AppState.mapsInfo]);
}


interface SaveFilesCount {
	custom: number;
	campaign: number;
	scenario: number;
	multi: number;
	hot: number;
	other: number;
}


// @ts-ignore
window['AppState'] = AppState;


/*
const errorModal = ErrorModal({
	title: 'Fatal Error',
	message: 'Failed to load maps metadata.',
	buttons: {
		custom: {
			title: 'EXIT',
			action: () => {
				errorModal.x.close();
			}
		},
	}
})
*/

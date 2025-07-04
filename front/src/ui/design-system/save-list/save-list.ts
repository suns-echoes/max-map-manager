import { getSavesInfo, type SaveInfo } from '^actions/get-saves-info';

import { Effect } from '^lib/reactive/effect.class';
import { Div, P, Section, Span } from '^lib/reactive/html-node.elements';
import { Value } from '^lib/reactive/value.class';
import { HTMLNode } from '^lib/reactive/html-node.class';

import { StandardBrokenButton } from '^ds/buttons/standard-broken-button';
import { Heading3 } from '^ds/headings/headings';
import { Inset } from '^ds/inset/inset';
import { Screen } from '^ds/screen/screen';

import { SaveSlot } from './save-slot/save-slot';

import styles from './save-list.module.css';


interface ViewState {
	selectedMapHashId: Value<MapHashId | null>;
	mapsAndSaves: Value<Map<MapHashId, MapAndSaves>>;
	mapsInfo: Value<Map<MapHashId, MapInfo>>;
	viewScreens: HTMLNode<any, Screen>[];
}

export function SaveList(viewState: ViewState, title: string, showButton: boolean = true) {
	let saveEntries;

	const saveList = (
		Section().classes(styles.saveList, 'frame outset frame-2 flex flex-col p-4').nodes([
			Div().classes('flex flex-spread flex-row no-grow ph-16 pv-8').nodes([
				Heading3(title).class(styles.title),
				showButton && StandardBrokenButton('').nodes([
					...'...'.split('').map((char) => Span().text(char)),
				]),
			]),
			Inset(8, ['fill-all']).nodes([
				saveEntries = Screen(viewState as any, true, ['fill-all']),
			]),
		])
	);

	new Effect(async function () {
		const selectedMapHashId = viewState.selectedMapHashId.value;
		if (!selectedMapHashId) {
			saveEntries.nodes([]);
			return;
		}

		const selectedMapInfo = viewState.mapsInfo.value.get(selectedMapHashId);
		if (!selectedMapInfo) return;

		const saveFilePaths = viewState.mapsAndSaves.value.get(selectedMapHashId)?.saves ?? [];

		let savesInfo: SaveInfo[] = [];
		try {
			savesInfo = await getSavesInfo(saveFilePaths, selectedMapInfo.width, selectedMapInfo.height);
		} catch (error) {
			console.error('Error loading saves:', error);
		}

		if (savesInfo.length === 0) {
			saveEntries.nodes([
				Div().classes('flex flex-col flex-center fill-all p-16').nodes([
					P().text('Error: x337').class('p-8'),
					P().text('There are no associated battle files to be shown.').class('p-16 text-center'),
					P().text(' ').class('p-16'),
				]),
			]);
			return;
		}

		saveEntries.nodes(savesInfo.map(function (saveInfo) {
			return SaveSlot(saveInfo);
		}));
	}).on([viewState.selectedMapHashId]);

	return saveList;
}

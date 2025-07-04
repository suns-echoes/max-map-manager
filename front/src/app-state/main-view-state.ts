import type { HTMLNode } from '^lib/reactive/html-node.class';
import { Value } from '^lib/reactive/value.class';

import { getInstalledMapsAndSaves } from '^actions/get-installed-maps-and-saves';

import { Screen } from '^ds/screen/screen';
import { getInstalledMapsInfo } from '../actions/get-installed-maps-info';


type MapSlots = Map<MapResName, MapHashId | null>;


export class MainViewState {
	static name = 'MAIN_VIEW_STATE';

	static selectedPlanet = new Value<PlanetName>('CRATER');
	static selectedMapHashId = new Value<MapHashId | null>(null);
	static selectedMapSlotIndex = new Value<number>(0);

	static mapSlots = new Value<MapSlots>(new Map([
		['CRATER_1', null],
		['CRATER_2', null],
		['CRATER_3', null],
		['CRATER_4', null],
		['CRATER_5', null],
		['CRATER_6', null],
		['DESERT_1', null],
		['DESERT_2', null],
		['DESERT_3', null],
		['DESERT_4', null],
		['DESERT_5', null],
		['DESERT_6', null],
		['GREEN_1', null],
		['GREEN_2', null],
		['GREEN_3', null],
		['GREEN_4', null],
		['GREEN_5', null],
		['GREEN_6', null],
		['SNOW_1', null],
		['SNOW_2', null],
		['SNOW_3', null],
		['SNOW_4', null],
		['SNOW_5', null],
		['SNOW_6', null],
	]));

	/**
	 * Info about installed maps and related save files.
	 */
	static mapsAndSaves = new Value<Map<MapHashId, MapAndSaves>>(new Map());

	/**
	 * Info about all installed maps.
	 */
	static mapsInfo = new Value<Map<MapHashId, MapInfo>>(new Map());

	static viewScreens: HTMLNode<any, Screen>[] = [];

	static blur() {
		this.viewScreens.forEach(screenNode => {
			screenNode.x.turnOff();
		});
	}

	static focus() {
		this.viewScreens.forEach(screenNode => {
			screenNode.x.turnOn();
		});
	}


	/**
	 * Update installed maps and saves info.
	 */
	static async update() {
		const [mapsAndSaves, mapsInfo] = await Promise.all([
			getInstalledMapsAndSaves(),
			getInstalledMapsInfo(),
		]);

		MainViewState.mapsAndSaves.set(mapsAndSaves);
		MainViewState.mapsInfo.set(new Map(mapsInfo.map(mapInfo => [mapInfo.mapHashId, mapInfo])));

		MainViewState.mapSlots.apply(function (mapSlots) {
			const mapping = new Map([...MainViewState.mapsAndSaves.value.entries()]
				.map(function ([mapHashId, mapAndSaves]) {
					const slotName = mapAndSaves.map.match(/(\w+_\d).wrl$/i)![1];
					return [slotName, mapHashId];
				}),
			);
			mapSlots.forEach(function (_, slotName) {
				mapSlots.set(
					slotName,
					mapping.get(slotName) ?? null,
				)
			});
			return mapSlots;
		}, true);

		MainViewState.refreshMapSelection();
	}

	/**
	 * Refresh map selection (force UI update).
	 */
	static refreshMapSelection() {
		const selectedMapHashId = MainViewState.mapSlots.value.get(
			`${MainViewState.selectedPlanet.value}_${MainViewState.selectedMapSlotIndex.value + 1}` as MapResName
		) ?? null;
		MainViewState.selectedMapHashId.set(selectedMapHashId);
		MainViewState.selectedMapSlotIndex.emit();
	}
}


// @ts-ignore
window['MainViewState'] = MainViewState;

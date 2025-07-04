import { HTMLNode } from '^lib/reactive/html-node.class';
import { Value } from '^lib/reactive/value.class';

import { getArchivedMapsAndSaves } from '^actions/get-archived-maps-and-saves';
import { getArchivedMapsInfo } from '^actions/get-archived-maps-info';

import { Screen } from '^ds/screen/screen';


export class ArchiveViewState {
	static name = 'ARCHIVE_VIEW_STATE';

	static selectedPlanet = new Value<PlanetName>('CRATER');
	static selectedMapHashId = new Value<MapHashId | null>(null);
	static selectedMapSlotIndex = new Value<number>(0);

	static showArchiveWindow = new Value<boolean>(false);

	static mapsAndSaves = new Value<Map<MapHashId, MapAndSaves>>(new Map());
	static mapsInfo = new Value<Map<MapHashId, MapInfo>>(new Map());

	static viewScreens: HTMLNode<any, Screen>[] = [];

	/**
	 * Update archived maps and saves info.
	 */
	static async update() {
		const [mapsAndSaves, mapsInfo] = await Promise.all([
			getArchivedMapsAndSaves(),
			getArchivedMapsInfo(),
		]);

		ArchiveViewState.mapsAndSaves.set(mapsAndSaves);
		ArchiveViewState.mapsInfo.set(new Map(mapsInfo.map(mapInfo => [mapInfo.mapHashId, mapInfo])));
	}

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
}


// @ts-ignore
window['ArchiveViewState'] = ArchiveViewState;

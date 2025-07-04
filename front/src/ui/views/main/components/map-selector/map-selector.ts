import { MainViewState } from '^state/main-view-state';

import { Effect } from '^lib/reactive/effect.class';
import { Div } from '^lib/reactive/html-node.elements';

import { Heading2 } from '^ds/headings/headings';
import { Outset } from '^ds/outset/outset';
import { MapBox } from '^ds/map-box/map-box';

import styles from './map-selector.module.css';


export function MapSelector() {
	let mapBox1, mapBox2, mapBox3, mapBox4, mapBox5, mapBox6;

	const mapSector = (
		Outset(2, [styles.mapSelector]).classes('flex flex-col flex-center gap-8 p-8').nodes([
			Heading2('Installed maps'),
			Div().classes('flex gap-8').nodes([
				mapBox1 = MapBox(MainViewState, 0),
				mapBox2 = MapBox(MainViewState, 1),
				mapBox3 = MapBox(MainViewState, 2),
				mapBox4 = MapBox(MainViewState, 3),
				mapBox5 = MapBox(MainViewState, 4),
				mapBox6 = MapBox(MainViewState, 5),
			]),
		])
	);

	new Effect(function updateMapBoxes() {
		const planet = MainViewState.selectedPlanet.value;

		[mapBox1, mapBox2, mapBox3, mapBox4, mapBox5, mapBox6].forEach((box, index) => {
			const slotName = (planet + '_' + (index + 1).toString(10)) as MapResName;
			const mapHashId = MainViewState.mapSlots.value.get(slotName);
			if (!mapHashId) {
				box.x.setMapInfo(null);
				return;
			}
			const mapInfo = MainViewState.mapsInfo.value.get(mapHashId);
			box.x.setMapInfo(mapInfo ?? null);
		});
	}).on([MainViewState.mapSlots, MainViewState.selectedPlanet]);

	return mapSector;
}

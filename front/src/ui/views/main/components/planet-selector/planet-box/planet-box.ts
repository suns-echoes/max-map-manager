import { MainViewState } from '^state/main-view-state';

import { Effect } from '^lib/reactive/effect.class';
import { Div, Img } from '^lib/reactive/html-node.elements';

import { Inset } from '^ds/inset/inset';
import { Screen } from '^ds/screen/screen';

import styles from './planet-box.module.css';


export function PlanetBox(planet: PlanetName) {
	const image = mapPlanetNameToImageUrl(planet);
	let screen;

	const planetBox = (
		Inset(8, [styles.planetBox]).nodes([
			screen = Screen(MainViewState, true).nodes([
				Div().classes('pointer', styles.selectionFrame).nodes([
					Img().src(image),
				]),
			]),
		])
	);

	let isSelected = false;

	function select() {
		if (isSelected) return;
		isSelected = true;
		planetBox.element.classList.add(styles.selected);
		MainViewState.selectedPlanet.set(planet);
		MainViewState.refreshMapSelection();
	}

	planetBox.addEventListener('click', select);

	if (MainViewState.selectedPlanet.value === planet) {
		select();
	}

	new Effect(function () {
		if (MainViewState.selectedPlanet.value === planet) {
			if (!isSelected) {
				select();
			}
		} else if (isSelected) {
			isSelected = false;
			planetBox.element.classList.remove(styles.selected);
		}
	}).on([MainViewState.selectedPlanet]);

	window._[2] = window._[2] || {};
	window._[2][planet] = screen;

	return planetBox;
}

function mapPlanetNameToImageUrl(planet: PlanetName): string {
	let file_name_prefix = 'STAR';
	switch (planet) {
		case 'CRATER':
			file_name_prefix = 'CRTR';
			break;
		case 'DESERT':
			file_name_prefix = 'DSRT';
			break;
		case 'GREEN':
			file_name_prefix = 'GREN';
			break;
		case 'SNOW':
			file_name_prefix = 'SNOW';
			break;
	}
	return `be://get-res-image/${file_name_prefix}_PIC`;
}

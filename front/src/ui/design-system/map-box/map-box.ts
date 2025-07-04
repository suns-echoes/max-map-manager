import { Effect } from '^lib/reactive/effect.class';
import { Div, Img } from '^lib/reactive/html-node.elements';

import { ViewState } from '^state/app-state';

import { Screen } from '^ds/screen/screen';
import { Inset } from '^ds/inset/inset';

import styles from './map-box.module.css';
import { EmptyText } from '../fx/empty-text/empty-text';


export interface MapBoxProps {
	setMapInfo: (mapInfo: MapInfo | null) => void;
}

export function MapBox(viewState: ViewState, mapSlotIndex: number) {
	let mapScreen, stars, emptyText;

	const mapBox = (
		Inset<MapBoxProps>(8).nodes([
			Div().classes(styles.mapBox).nodes([
				mapScreen = Screen(viewState, true).nodes([
					Div().classes('pointer fill-all flex flex-center', styles.selectionFrame).nodes([
						stars = Img().src('be://get-res-image/STAR_PIC').style({
							width: '112px',
							height: '112px',
						}),
						emptyText = EmptyText().classes('absolute full-size center'),
					]),
				]),
			]),
		])
	);

	mapBox.onDestroy(() => {
		viewState.viewScreens = viewState.viewScreens.filter((screen) => screen !== mapScreen);
	});

	let mapHashId: MapHashId | null = null;
	let isSelected = false;

	mapBox.addEventListener('click', () => {
		viewState.selectedMapHashId.set(mapHashId);
		viewState.selectedMapSlotIndex.set(mapSlotIndex);
	});

	mapBox.x.setMapInfo = function (newMapInfo: MapInfo | null) {
		if (newMapInfo) {
			stars.element.src = `be://get-wrl-minimap/${newMapInfo.mapHashId}`;
			emptyText.element.style.display = 'none';
			mapHashId = newMapInfo.mapHashId;
		} else {
			stars.element.src = `be://get-res-image/STAR_PIC`;
			emptyText.element.style.display = 'block';
			mapHashId = null;
		}
	};

	new Effect(function updateMapBoxSelection() {
		if (viewState.selectedMapSlotIndex.value === mapSlotIndex) {
			if (!isSelected) {
				isSelected = true;
				mapBox.element.classList.add(styles.selected);
			}
		} else {
			if (isSelected) {
				isSelected = false;
				mapBox.element.classList.remove(styles.selected);
			}
		}
	}).on([viewState.selectedMapSlotIndex]).update();

	return mapBox;
}

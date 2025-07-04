import { Section } from '^lib/reactive/html-node.elements';

import { MainViewState } from '^state/main-view-state';

import { SaveList } from '^ds/save-list/save-list';

import { Header } from './components/header/header';
import { MapPreview } from './components/map-preview/map-preview';
import { MapSelector } from './components/map-selector/map-selector';
import { PlanetSelector } from './components/planet-selector/planet-selector';
import { Footer } from './components/footer/footer';

import styles from './layout.module.css';


export function MainWindowLayout() {
	return (
		Section().class(styles.mainWindowLayout).nodes([
			Header(),
			PlanetSelector(),
			MapSelector(),
			MapPreview(),
			SaveList(MainViewState, 'Battles to win'),
			Footer(),
		])
	);
}

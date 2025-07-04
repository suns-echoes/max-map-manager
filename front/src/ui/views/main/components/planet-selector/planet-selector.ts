import { Div } from '^lib/reactive/html-node.elements';

import { Heading2 } from '^ds/headings/headings';
import { Outset } from '^ds/outset/outset';

import { PlanetBox } from './planet-box/planet-box';

import styles from './planet-selector.module.css';


export function PlanetSelector() {
	const planetSelector = (
		Outset(2, [styles.planetSelector]).classes('flex flex-col flex-center gap-8 p-8').nodes([
			Heading2('PLANETS'),
			Div().classes('flex flex-col flex-center gap-16').nodes([
				PlanetBox('CRATER'),
				PlanetBox('DESERT'),
				PlanetBox('GREEN'),
				PlanetBox('SNOW'),
			]),
		])
	);
	return planetSelector;
}

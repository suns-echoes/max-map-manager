import type { SaveInfo } from '^actions/get-saves-info';

import { Div, Img } from '^lib/reactive/html-node.elements';

import styles from './save-slot.module.css';


export function SaveSlot(saveInfo: SaveInfo) {
	return Div().classes(styles.saveSlot, 'flex flex-col').nodes([
		Div().classes('flex gap-8 flex-row flex-start-center').nodes([
			Img().src('./images/brain.png').classes('no-grow'),
			Div().text(saveInfo.name),
		]),
		Div().classes('flex flex-row p-8 text-small').nodes([
			Div().text(`Turn: ${saveInfo.currentTurn}`),
		]),
	]);
}

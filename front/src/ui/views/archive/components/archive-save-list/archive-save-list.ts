import { Div, Section } from '^lib/reactive/html-node.elements';

import { ArchiveViewState } from '^state/archive-view-state';

import { Inset } from '^ds/inset/inset';
import { Outset } from '^ds/outset/outset';
import { Screen } from '^ds/screen/screen';
import { SquareRippedButton } from '^ds/buttons/square-ripped-button';
import { SquareBrokenButton } from '^ds/buttons/square-broken-button';
import { SquareButton } from '^ds/buttons/square-button';
import { SaveList } from '^ds/save-list/save-list';

import styles from './archive-save-list.module.css';


export function ArchiveSaveList() {
	let brokenButton1, brokenButton2, brokenMessage;

	function generateNewBrokenMessage() {
		const x = Math.random().toString(36).substring(2, 4);
		const y = Math.random().toString(36).substring(2, 4);
		const h = Math.floor(Math.random() * 0x10000).toString(16).padStart(4, '0');
		return `. / ${x}_${y} / 0x${h} / 0x16`;
	}

	const archiveSaveList = (
		Outset(4, [styles.archiveSaveList]).classes('flex flex-col p-2').style({ display: 'none' }).nodes([
			SaveList(ArchiveViewState, 'Unfnshd battles', false).class('fill-all flex flex-col p-4'),
			Section().classes('frame outset frame-2 no-shrink flex flex-col p-4').nodes([
				Inset(8, ['fill-all']).nodes([
					Screen(ArchiveViewState, true).nodes([
						brokenMessage = Div().class(styles.brokenScreen).id('broken-message').text(generateNewBrokenMessage()),
					]),
				]),
				Div().classes('flex flex-center flex-row gap-4 pv-8').nodes([
					SquareRippedButton(48),
					brokenButton1 = SquareButton(48, '✨'),
					SquareRippedButton(48),
					SquareRippedButton(48),
					SquareRippedButton(48),
					brokenButton2 = Div().style({
						position: 'relative',
						width: '0',
						overflow: 'visible',
						left: '-52px',
						transform: 'rotate(1deg)',
					}).nodes([
						SquareBrokenButton(48, '🏗️'),
					]),
				]),
			]),
		])
	);

	brokenButton1.addEventListener('click', function () {
		brokenMessage.text(generateNewBrokenMessage());
	});

	brokenButton2.addEventListener('click', function () {
		brokenButton2.element.classList.add(styles.fallOff);
	});

	return archiveSaveList;
}

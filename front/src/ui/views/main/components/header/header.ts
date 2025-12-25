import { Div, Img, Section, Span } from '^lib/reactive/html-node.elements';
import { Effect } from '^lib/reactive/effect.class';

import { ArchiveViewState } from '^state/archive-view-state';

import { StandardBrokenButton } from '^ds/buttons/standard-broken-button';
import { StandardButton } from '^ds/buttons/standard-button';
import { ToggleButton } from '^ds/buttons/toggle-button';
import { Heading1 } from '^ds/headings/headings';
import { helpModal } from '^ds/help-modal/help-modal';

import styles from './header.module.css';
import { importNewMap } from '../../../../../actions/import-new-map';
import { MainViewState } from '../../../../../app-state/main-view-state';
import { createMapSlotName } from '../../../../../lib/create-map-slot-name';


export function Header() {
	let importButton, archiveButton, helpButton;

	const header = (
		Section().classes(styles.header, 'flex flex-row flex-spread ph-16').nodes([
			Img().src('./images/vent.png').classes(styles.vent, 'no-grow'),
			Heading1('M.A.X. Map Manager'),
			Div().classes('flex flex-row gap-8').nodes([
				StandardBrokenButton('').nodes([
					...'R̵̳E̷͎_̷̧E̷̹ /̵̖'.split('').map((char) => Span().text(char)),
				]),
				importButton = StandardButton('IMPORT'),
				archiveButton = ToggleButton('ARCHIVE').id('archive-toggle-button'),
				helpButton = StandardButton('?'),
			]),
			Img().src('./images/vent.png').classes(styles.vent, 'no-grow'),
		])
	);

	importButton.addEventListener('click', async function () {
		const selectedSlotName = createMapSlotName(
			MainViewState.selectedPlanet.value,
			MainViewState.selectedMapSlotIndex.value,
		);
		const needsUpdate = await importNewMap(selectedSlotName);
		if (needsUpdate) {
			MainViewState.update();
			ArchiveViewState.update();
		}
	});

	helpButton.addEventListener('click', function () {
		helpModal.x.open();
	});

	archiveButton.addEventListener('click', function () {
		ArchiveViewState.showArchiveWindow.apply(function (isShown) {
			const show = !isShown;
			archiveButton.x.toggle(show);
			return show;
		});
	});

	new Effect(function () {
		const isShown = ArchiveViewState.showArchiveWindow.value;
		archiveButton.x.toggle(isShown);
	}).on([ArchiveViewState.showArchiveWindow]);

	return header;
}

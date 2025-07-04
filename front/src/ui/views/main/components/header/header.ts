import { Div, Img, Section, Span } from '^lib/reactive/html-node.elements';
import { Effect } from '^lib/reactive/effect.class';

import { ArchiveViewState } from '^state/archive-view-state';

import { StandardBrokenButton } from '^ds/buttons/standard-broken-button';
import { StandardButton } from '^ds/buttons/standard-button';
import { ToggleButton } from '^ds/buttons/toggle-button';
import { Heading1 } from '^ds/headings/headings';

import styles from './header.module.css';


export function Header() {
	let archiveButton;

	const header = (
		Section().classes(styles.header, 'flex flex-row flex-spread ph-16').nodes([
			Img().src('./images/vent.png').classes(styles.vent, 'no-grow'),
			Heading1('M.A.X. Map Manager'),
			Div().classes('flex flex-row gap-8').nodes([
				StandardBrokenButton('').nodes([
					...'R̵̳E̷͎_̷̧E̷̹ /̵̖'.split('').map((char) => Span().text(char)),
				]),
				StandardButton('IMPORT'),
				archiveButton = ToggleButton('ARCHIVE').id('archive-toggle-button'),
				StandardBrokenButton('').nodes([
					...'~?̴̖'.split('').map((char) => Span().text(char)),
				]),
			]),
			Img().src('./images/vent.png').classes(styles.vent, 'no-grow'),
		])
	);

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

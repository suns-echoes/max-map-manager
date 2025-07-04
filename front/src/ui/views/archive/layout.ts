import { Section } from '^lib/reactive/html-node.elements';
import { Effect } from '^lib/reactive/effect.class';

import { AppState } from '^state/app-state';
import { ArchiveViewState } from '^state/archive-view-state';

import { ArchiveSaveList } from './components/archive-save-list/archive-save-list';
import { ArchiveMapPreview } from './components/archive-map-preview/archive-map-preview';
import { ArchiveMapSelector } from './components/archive-map-selector/archive-map-selector';

import styles from './layout.module.css';


interface ArchiveWindowLayout {
	open: () => void;
	close: () => void;
}

export function ArchiveWindowLayout() {
	let saveList, mapPreview, mapSelector;

	const archiveWindowLayout = (
		Section<ArchiveWindowLayout>().class(styles.archiveWindowLayout).nodes([
			mapSelector = ArchiveMapSelector().id('archive-map-selector'),
			mapPreview = ArchiveMapPreview().id('archive-map-preview'),
			saveList = ArchiveSaveList().id('archive-save-list'),
		])
	);

	let animationTimer: ReturnType<typeof setTimeout> | null = null;

	const open = function () {
		if (animationTimer) {
			clearTimeout(animationTimer);
			animationTimer = null;
		}
		saveList.element.style.display = 'flex';
		mapPreview.element.style.display = 'flex';
		mapSelector.element.style.display = 'flex';

		AppState.focusView(ArchiveViewState);

		requestAnimationFrame(function () {
			mapSelector.x.updateScreens();
			saveList.element.classList.add(styles.open);
			mapPreview.element.classList.add(styles.open);
			mapSelector.element.classList.add(styles.open);
		});
	};

	const close = function () {
		saveList.element.classList.remove(styles.open);
		mapPreview.element.classList.remove(styles.open);
		mapSelector.element.classList.remove(styles.open);
		animationTimer = setTimeout(function () {
			animationTimer = null;
			saveList.element.style.display = 'none';
			mapPreview.element.style.display = 'none';
			mapSelector.element.style.display = 'none';
		}, 1000);

		AppState.focusPreviousView();
	};

	new Effect(function toggleArchiveView() {
		if (ArchiveViewState.showArchiveWindow.value) {
			open();
		} else {
			close();
		}
	}).on([ArchiveViewState.showArchiveWindow])

	return archiveWindowLayout;
}

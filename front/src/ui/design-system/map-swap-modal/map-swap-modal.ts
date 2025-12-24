import { be } from '^lib/be';
import { Div, Img, P, Span } from '^lib/reactive/html-node.elements.ts';

import { AppState } from '^state/app-state';
import { SwapMapViewState } from '^state/swap-map-view-state';

import { swapMaps } from '^actions/swap-maps';

import { Modal, type ModalInterface } from '^ds/modal/modal.ts';

import { Inset } from '../inset/inset';
import { Outset } from '../outset/outset';
import { Screen } from '../screen/screen';

import { Heading3, Heading4 } from '../headings/headings';
import { StandardButton } from '../buttons/standard-button';
import { EmptyText } from '../fx/empty-text/empty-text';

import styles from './map-swap-modal.module.css';


interface MapSwapModal {
	open: (installedMapInfo: MapInfo | null, archivedMapInfo: MapInfo | null) => void;
	close: () => void;
}

export function MapSwapModal() {
	let stars1, emptyText1,
		stars2, emptyText2,
		toInstallMapName, toArchiveMapName,
		cancelButton, confirmButton;

	const modal = Modal<MapSwapModal>().nodes([
		Outset(2, [styles.mapSwapModal]).classes('fill-all pv-16 ph-24').nodes([
			Heading3('Swap Maps').classes('mb-16', 'text-center'),
			Inset(2),
			Inset(2),
			Div().classes('flex flex-row flex-center mv-16').nodes([
				Div().nodes([
					Heading4('MAX', ['text-center']),
					Inset(8).nodes([
						Screen(SwapMapViewState, true).nodes([
							Div().classes('pointer fill-all flex flex-center', styles.selectionFrame).nodes([
								stars1 = Img().src(be('get-res-image/STAR_PIC')).style({
									width: '112px',
									height: '112px',
								}),
								emptyText1 = P().classes('absolute full-size center').nodes([
									EmptyText(),
								]),
							]),
						]),
					]),
				]),
				Div().class(styles.arrows).nodes([
					Div().class(styles.arrowRight).text('🡆'),
					Div().class(styles.arrowLeft).text('🡄'),
				]),
				Div().nodes([
					Heading4('Archive', ['text-center']),
					Inset(8).nodes([
						Screen(SwapMapViewState, true).nodes([
							Div().classes('pointer fill-all flex flex-center', styles.selectionFrame).nodes([
								stars2 = Img().src(be('get-res-image/STAR_PIC')).style({
									width: '112px',
									height: '112px',
								}),
								emptyText2 = P().classes('absolute full-size center').nodes([
									EmptyText(),
								]),
							]),
						]),
					]),
				]),
			]),
			Inset(8, ['mv-16']).nodes([
				Screen(SwapMapViewState, true).nodes([
					Div().classes('m-4', styles.selectionFrame).nodes([
						P().classes('mv-8 text-ellipsis').text('to install: ').nodes([
							Span().text('install: '),
							toInstallMapName = Span().text('[empty]'),
						]),
						P().classes('mv-8 text-ellipsis').text('to archive: ').nodes([
							Span().text('archive: '),
							toArchiveMapName = Span().text('[empty]'),
						]),
					]),
				]),
			]),
			Inset(2),
			Inset(2),
			Div().classes('flex flex-row flex-spaced mt-16').nodes([
				cancelButton = StandardButton('Cancel'),
				confirmButton = StandardButton('Confirm'),
			]),
		]),
	]);

	// --- Logic ---

	let toInstallMapInfo: MapInfo | null = null;
	let toArchiveMapInfo: MapInfo | null = null;

	const setToInstallMapInfo = function (newMapInfo: MapInfo | null) {
		toInstallMapInfo = newMapInfo;
		if (newMapInfo) {
			stars2.element.src = be(`get-wrl-minimap/${newMapInfo.mapHashId}`);
			emptyText2.element.style.display = 'none';
			toInstallMapName.text(newMapInfo.name);
		} else {
			stars2.element.src = be(`get-res-image/STAR_PIC`);
			emptyText2.element.style.display = 'block';
			toInstallMapName.text('[empty]');
		}
	}

	const setToArchiveMapInfo = function (newMapInfo: MapInfo | null) {
		toArchiveMapInfo = newMapInfo;
		if (newMapInfo) {
			stars1.element.src = be(`get-wrl-minimap/${newMapInfo.mapHashId}`);
			emptyText1.element.style.display = 'none';
			toArchiveMapName.text(newMapInfo.name);
		} else {
			stars1.element.src = be(`get-res-image/STAR_PIC`);
			emptyText1.element.style.display = 'block';
			toArchiveMapName.text('[empty]');
		}
	}

	const _open = modal.x.open as ModalInterface['open'];

	modal.x.open = function (newInstalledMapInfo: MapInfo | null, newArchivedMapInfo: MapInfo | null) {
		setToInstallMapInfo(newArchivedMapInfo);
		setToArchiveMapInfo(newInstalledMapInfo);
		AppState.focusView(SwapMapViewState);
		_open();
	};

	modal.x.close = function () {
		modal.element.remove();
		AppState.focusPreviousView();
	};

	cancelButton.addEventListener('click', () => {
		modal.x.close();
	});

	confirmButton.addEventListener('click', async () => {
		swapMaps(toInstallMapInfo?.mapHashId ?? null, toArchiveMapInfo?.mapHashId ?? null);
		modal.x.close();
	});

	return modal;
}

export const mapSwapModal = MapSwapModal();

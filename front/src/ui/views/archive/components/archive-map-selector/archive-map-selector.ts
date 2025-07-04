import { AppState } from '^state/app-state';
import { ArchiveViewState } from '^state/archive-view-state';
import { MainViewState } from '^state/main-view-state';

import { openMapSwapModal } from '^actions/open-map-swap-modal';

import { Effect } from '^lib/reactive/effect.class';
import { Div, P, Span } from '^lib/reactive/html-node.elements';

import { Heading2 } from '^ds/headings/headings';
import { Inset } from '^ds/inset/inset';
import { Outset } from '^ds/outset/outset';
import { Screen } from '^ds/screen/screen';
import { SquareButton } from '^ds/buttons/square-button';
import { StandardButton } from '^ds/buttons/standard-button';
import { MapBox } from '^ds/map-box/map-box';

import styles from './archive-map-selector.module.css';
import { Value } from '../../../../../lib/reactive/value.class';


interface ArchiveMapSelector {
	updateScreens(): void;
}

export function ArchiveMapSelector() {
	let mapInstallButton, prevPageButton, nextPageButton,
		mapList, archivedMapCount, paginationCurrent, paginationMax,
		saveOther, saveCustom, saveCampaign, saveScenario, saveMultiplayer, saveHotSeat;

	const archiveMapSelector = (
		Outset<ArchiveMapSelector>(4, [styles.archiveMapSelector]).classes('flex flex-col p-2').style({ display: 'none' }).nodes([
			Div().classes('flex flex-spread').nodes([
				Outset(2, ['flex-grow']).classes('flex flex-spread ph-16 pv-8').nodes([
					Heading2('MAP ARCHIVE'),
					mapInstallButton = StandardButton('INSTALL').class(styles.installButton),
				]),
				Div().classes('flex ph-16').nodes([
					prevPageButton = SquareButton(42, '🢀'),
					nextPageButton = SquareButton(42, '🢂'),
				]),
			]),
			mapList = Outset(2, ['fill-all']).classes('flex flex-wrap row-space-evenly row-center-vertical gap-16 p-16 stop'),
			Outset(2).classes('basis-third fill-all ph-16 pv-8 no-shrink', styles.infoBox).nodes([
				Inset(4).classes('fill-all ph-16 pv-8').nodes([
					Inset(8).classes('fill-all').nodes([
						Screen(ArchiveViewState, true, ['fill-all flex flex-col flex-spread p-8']).nodes([
							Div().class('flex flex-spread full-width').nodes([
								P().nodes([
									Span().text('Found total of '),
									archivedMapCount = Span().text('0'),
									Span().text(' maps'),
								]),
								P().nodes([
									Span().text('Page '),
									paginationCurrent = Span().text('1'),
									Span().text(' of '),
									paginationMax = Span().text('1'),
								]),
							]),
							Div().class(styles.divider),
							Div().class('flex flex-spread full-width').nodes([
								P().nodes([
									Span().text('Found total of '),
									Span().text('0'),
									Span().text(' save files:'),
								]),
								P().nodes([
									Span().text('Other: '),
									saveOther = Span().text('0'),
								]),
							]),
							Div().class('flex flex-spread full-width').nodes([
								P().nodes([
									Span().text('CUS: '),
									saveCustom = Span().text('0'),
								]),
								P().nodes([
									Span().text('CAM: '),
									saveCampaign = Span().text('0'),
								]),
								P().nodes([
									Span().text('SCN: '),
									saveScenario = Span().text('0'),
								]),
								P().nodes([
									Span().text('MUL: '),
									saveMultiplayer = Span().text('0'),
								]),
								P().nodes([
									Span().text('HOT: '),
									saveHotSeat = Span().text('0'),
								]),
							]),
						]),
					]),
				]),
			]),
		])
	);

	// --- Logic ---

	mapInstallButton.addEventListener('click', function () {
		const installedMapHashId = MainViewState.selectedMapHashId.value;
		const installedMapInfo = MainViewState.mapsInfo.value.get(installedMapHashId!)
			?? null;

		const archivedMapHashId = ArchiveViewState.selectedMapHashId.value;
		const archivedMapInfo = ArchiveViewState.mapsInfo.value.get(archivedMapHashId!)
			?? null;

		openMapSwapModal(
			installedMapInfo,
			archivedMapInfo,
		);
	});

	const mapBoxes: ReturnType<typeof MapBox>[] = [];
	const currentPage = new Value(0);

	prevPageButton.addEventListener('click', function () {
		if (currentPage.value > 0) {
			currentPage.set(currentPage.value - 1);
			paginationCurrent.text((currentPage.value + 1));
		}
	});

	nextPageButton.addEventListener('click', function () {
		const mapCount = ArchiveViewState.mapsInfo.value.size;
		const mapsPerPage = mapBoxes.length;
		const maxPage = Math.floor((mapCount - 1) / mapsPerPage);
		if (currentPage.value < maxPage) {
			currentPage.set(currentPage.value + 1);
			paginationCurrent.text((currentPage.value + 1));
		}
	});

	const updateMapPreviews = function updateMapPreviews() {
		const archivedMapsInfo = [...ArchiveViewState.mapsInfo.value];
		const start = currentPage.value * mapBoxes.length;

		paginationMax.text(Math.max(1, Math.ceil(archivedMapsInfo.length / mapBoxes.length)));

		if (ArchiveViewState.selectedMapHashId.value !== archivedMapsInfo?.[ArchiveViewState.selectedMapSlotIndex.value]?.[0]) {
			if (!ArchiveViewState.mapsInfo.value.has(ArchiveViewState.selectedMapHashId.value!)) {
				ArchiveViewState.selectedMapHashId.set(archivedMapsInfo[0]?.[0] ?? null, true);
				ArchiveViewState.selectedMapSlotIndex.set(0, true);
			} else {
				let slotIndexForMapHashId = archivedMapsInfo.findIndex(([mapHashId]) =>
					mapHashId === ArchiveViewState.selectedMapHashId.value
				);
				if (slotIndexForMapHashId === -1) {
					ArchiveViewState.selectedMapHashId.set(archivedMapsInfo[0]?.[0] ?? null, true);
					ArchiveViewState.selectedMapSlotIndex.set(0, true);
				} else {
					ArchiveViewState.selectedMapSlotIndex.set(slotIndexForMapHashId, true);
				}
			}
		}

		for (let i = 0; i < mapBoxes.length; i++) {
			const mapInfo = [...archivedMapsInfo][i + start]?.[1];
			mapBoxes[i].x.setMapInfo(mapInfo ?? null);
		}

		if (ArchiveViewState.selectedMapHashId.value === null) {
			const firstBoxMapHashId = archivedMapsInfo[0]?.[0];
			if (firstBoxMapHashId) {
				ArchiveViewState.selectedMapHashId.set(firstBoxMapHashId);
			}
		}
	}

	archiveMapSelector.x.updateScreens = function updateScreens() {
		const screenOuterSize = 148 + 16;
		const screenXCount = ((mapList!.element.clientWidth - 16) / screenOuterSize) | 0;
		const screenYCount = ((mapList!.element.clientHeight - 16) / screenOuterSize) | 0;
		const totalScreens = screenXCount * screenYCount;

		for (let i = mapBoxes.length; i < totalScreens; i++) {
			const mapBox = MapBox(ArchiveViewState, i);
			mapBoxes.push(mapBox);
		}
		for (let i = totalScreens; i < mapBoxes.length; i++) {
			const screen = mapBoxes[i];
			screen.element.remove();
		}
		mapBoxes.length = totalScreens;

		updateMapPreviews();

		requestAnimationFrame(function () {
			mapList!.nodes(mapBoxes);
		});
	}

	new Effect(function updateMapBoxes() {
		updateMapPreviews();
	}).on([ArchiveViewState.mapsInfo, currentPage]);

	let screensUpdateDelay: number;

	new Effect(function updateMapSelectorBoxes() {
		mapList.nodes([]);
		clearTimeout(screensUpdateDelay);
		screensUpdateDelay = window.setTimeout(function () {
			archiveMapSelector.x.updateScreens();
		}, 170);
	}).on([AppState.windowSize]).update();

	new Effect(function updateMapCount() {
		archivedMapCount.text(`${ArchiveViewState.mapsInfo.value.size}`);
	}).on([ArchiveViewState.mapsInfo]);

	new Effect(function updateSavesCount() {
		saveOther.text(`${AppState.saveFilesCountByType.value.other ?? 0}`);
		saveCustom.text(`${AppState.saveFilesCountByType.value.custom ?? 0}`);
		saveCampaign.text(`${AppState.saveFilesCountByType.value.campaign ?? 0}`);
		saveScenario.text(`${AppState.saveFilesCountByType.value.scenario ?? 0}`);
		saveMultiplayer.text(`${AppState.saveFilesCountByType.value.multi ?? 0}`);
		saveHotSeat.text(`${AppState.saveFilesCountByType.value.hot ?? 0}`);
	}).on([AppState.saveFilesCountByType]);

	return archiveMapSelector;
}

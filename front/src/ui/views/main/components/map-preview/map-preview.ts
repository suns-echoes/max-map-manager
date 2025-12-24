import { putMapInArchive } from '^actions/put-map-in-archive';

import { ArchiveViewState } from '^state/archive-view-state';
import { AppState } from '^state/app-state';
import { MainViewState } from '^state/main-view-state';

import { be } from '^lib/be';
import { Effect } from '^lib/reactive/effect.class';
import { Div, Img, P } from '^lib/reactive/html-node.elements';

import { StandardButton } from '^ds/buttons/standard-button';
import { Heading3 } from '^ds/headings/headings';
import { Inset } from '^ds/inset/inset';
import { Outset } from '^ds/outset/outset';
import { Screen } from '^ds/screen/screen';

import styles from './map-preview.module.css';


export function MapPreview() {
	let archiveButton, replaceButton, previewSection, noPreviewSection, slotName,
		bigmapPreviewText, bigmapPreview,
		sizeLabel, nameLabel, versionLabel, authorLabel, dateLabel, descriptionLabel;

	const mapPreview = (
		Outset(2).nodes([
			Div().classes('flex flex-spread flex-row no-grow ph-16 pv-8').nodes([
				Heading3('Map preview').class(styles.title),
				Div().class('flex gap-16').nodes([
					archiveButton = StandardButton('ARCHIVE'),
					StandardButton('EDIT DATA'),
					replaceButton = StandardButton('REPLACE'),
				]),
			]),
			Inset(8, [styles.mapPreview, 'fill-all m-8 mt-0']).nodes([
				Screen(MainViewState).nodes([
					previewSection = Div().class(styles.wrapper).nodes([
						Div().class(styles.content).nodes([
							Div().classes('flex flex-row gap-16').nodes([
								bigmapPreviewText = Div().class(styles.bigmapPreviewText).text('[ loading preview ]'),
								bigmapPreview = Img().class(styles.bigmapPreview),
								Div().classes('flex flex-col gap-16 full-width').nodes([
									Div().classes(styles.infoLine, 'flex flex-row flex-spread gap-8').nodes([
										Div().text('name:'),
										nameLabel = Div(),
									]),
									Div().classes(styles.infoLine, 'flex flex-row flex-spread gap-8').nodes([
										Div().text('size:'),
										sizeLabel = Div(),
									]),
									Div().classes(styles.infoLine, 'flex flex-row flex-spread gap-8').nodes([
										Div().text('version:'),
										versionLabel = Div(),
									]),
									Div().classes(styles.infoLine, 'flex flex-row flex-spread gap-8').nodes([
										Div().text('author:'),
										authorLabel = Div(),
									]),
									Div().classes(styles.infoLine, 'flex flex-row flex-spread gap-8').nodes([
										Div().text('date:'),
										dateLabel = Div(),
									]),
								]),
							]),
							Div().nodes([
								descriptionLabel = Div().class(styles.mapDescription),
							]),
						]),
					]),
					noPreviewSection = Div().class(styles.noPreview).nodes([
						Div().classes(styles.noPreviewText, 'flex-center w-80p').nodes([
							Div().classes('flex flex-col gap-16').nodes([
								slotName = P(),
								P().text('This map slot is empty...'),
								P().text(`
									To install a map in this slot, click the REPLACE
									button above and choose a map from the Archive.
								`),
								P().text('Alternatively, use the IMPORT button to import a new map.'),
								P().text('It is also possible to ge_/r!*.. . 0xffff ....'),
							]),
						]),
					]),
				]),
			]),
		])
	);

	replaceButton.addEventListener('click', function goToArchive() {
		ArchiveViewState.showArchiveWindow.set(true);
	});

	archiveButton.addEventListener('click', async function updateSelectedMapHashId() {
		const selectedMapHashId = MainViewState.selectedMapHashId.value;
		if (!selectedMapHashId) return;

		await putMapInArchive(selectedMapHashId);
		ArchiveViewState.selectedMapHashId.set(selectedMapHashId);
		await AppState.update();
		ArchiveViewState.selectedMapHashId.set(selectedMapHashId, true);
	});

	new Effect(async function updateMapPreview() {
		const selectedMapHashId = MainViewState.selectedMapHashId.value;

		if (selectedMapHashId === null || !MainViewState.mapsInfo.value.has(selectedMapHashId)) {
			previewSection.element.style.display = 'none';
			noPreviewSection.element.style.display = 'flex';

			return;
		}

		previewSection.element.style.display = 'block';
		noPreviewSection.element.style.display = 'none';
		bigmapPreviewText.element.style.display = 'flex';
		bigmapPreview.element.style.opacity = '0';

		if (selectedMapHashId) {
			const mapInfo = MainViewState.mapsInfo.value.get(selectedMapHashId);
			if (mapInfo) {
				nameLabel.text(mapInfo.name);
				sizeLabel.text(`${mapInfo.width} x ${mapInfo.height}`);
				versionLabel.text(mapInfo.version);
				authorLabel.text(mapInfo.author);
				dateLabel.text(mapInfo.date);
				descriptionLabel.text(mapInfo.description.replaceAll('\\n', '\n'));
			}
		}

		bigmapPreview.element.src = be(`get-wrl-bigmap/${selectedMapHashId}`);
		bigmapPreviewText.element.style.display = 'none';
		bigmapPreview.element.style.opacity = '1';
	}).on([MainViewState.selectedMapHashId]).update();

	new Effect(function updateEmptySlotMessage() {
		slotName.text(`Map slot #${MainViewState.selectedPlanet.value}_${MainViewState.selectedMapSlotIndex.value + 1}`);
	}).on([MainViewState.selectedMapSlotIndex]);

	return mapPreview;
}

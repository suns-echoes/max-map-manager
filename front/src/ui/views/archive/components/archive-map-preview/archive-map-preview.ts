import { Div, Img, Section } from '^lib/reactive/html-node.elements';
import { Effect } from '^lib/reactive/effect.class';

import { ArchiveViewState } from '^state/archive-view-state';

import { Heading3 } from '^ds/headings/headings';
import { Inset } from '^ds/inset/inset';
import { Outset } from '^ds/outset/outset';
import { Screen } from '^ds/screen/screen';
import { SquareBrokenButton } from '^ds/buttons/square-broken-button';
import { SquareRippedButton } from '^ds/buttons/square-ripped-button';
import { SquareButton } from '^ds/buttons/square-button';

import styles from './archive-map-preview.module.css';


export function ArchiveMapPreview() {
	let bigmapPreviewText, bigmapPreview,
		mapDataScrollDown, mapDataScrollUp, mapInfo,
		sizeLabel, nameLabel, versionLabel, authorLabel, dateLabel, descriptionLabel;

	const archiveMapPreview = (
		Outset(4, [styles.archiveMapPreview]).classes('flex flex-col p-2').style({ display: 'none' }).nodes([
			Section().classes(styles.saveList, 'frame outset frame-2 flex flex-col p-4 full-width relative').nodes([
				Inset(8, ['fill-all s224x224']).nodes([
					Screen(ArchiveViewState, true).nodes([
						Div().classes('s200x200 flex flex-center').nodes([
							bigmapPreviewText = Div().text('[ loading preview ]'),
							bigmapPreview = Img(),
						]),
					]),
				]),
				Div().class(styles.buttons).nodes([
					SquareBrokenButton(40, '2x'),
					SquareBrokenButton(40, '🧨'),
					SquareRippedButton(40),
				]),
			]),
			Section().classes('frame outset frame-2 flex flex-col pv-8 ph-16').nodes([
				Div().classes('flex flex-spread flex-row no-grow').nodes([
					Heading3('Map data').class(styles.title),
					Div().classes('flex right').nodes([
						mapDataScrollDown = SquareButton(32, '▼'),
						mapDataScrollUp = SquareButton(32, '▲'),
					]),
				]),
			]),
			Section().classes(styles.mapList, 'frame outset frame-2 flex flex-col p-4 fill-all').nodes([
				Inset(8, ['fill-all stop']).nodes([
					Screen(ArchiveViewState, true).nodes([
						Div().class(styles.content).nodes([
							Div().classes('flex flex-col gap-16 p-8').nodes([
								mapInfo = Div().classes('flex flex-col gap-8 full-width').nodes([
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
								Div().nodes([
									descriptionLabel = Div().class(styles.mapDescription),
								]),
							]),
						]),
					]),
				]),
			]),
		])
	);

	mapDataScrollDown.addEventListener('click', () => {
		archiveMapPreview.element.querySelector(`.${styles.content}`)?.scrollBy({ top: 100, behavior: 'smooth' });
	});

	mapDataScrollUp.addEventListener('click', () => {
		archiveMapPreview.element.querySelector(`.${styles.content}`)?.scrollBy({ top: -100, behavior: 'smooth' });
	});

	new Effect(async function updateMapPreview() {
		bigmapPreviewText.text('[ loading preview ]');
		bigmapPreviewText.element.style.display = 'flex';
		bigmapPreview.element.style.display = 'none';
		mapInfo.element.style.display = 'none';

		const selectedMapHashId = ArchiveViewState.selectedMapHashId.value;

		if (selectedMapHashId === null) {
			bigmapPreviewText.text('[ select map ]');
			descriptionLabel.text('[ error! missing data..? ]');
			return;
		}

		if (selectedMapHashId) {
			const mapInfo = ArchiveViewState.mapsInfo.value.get(selectedMapHashId);
			if (mapInfo) {
				nameLabel.text(mapInfo.name);
				sizeLabel.text(`${mapInfo.width} x ${mapInfo.height}`);
				versionLabel.text(mapInfo.version);
				authorLabel.text(mapInfo.author);
				dateLabel.text(mapInfo.date);
				descriptionLabel.text(mapInfo.description.replaceAll('\\n', '\n'));
				bigmapPreview.element.src = `be://get-wrl-bigmap/${mapInfo.mapHashId}/192`;
				bigmapPreviewText.element.style.display = 'none';
				bigmapPreview.element.style.display = 'block';
			} else {
				throw new Error(`THIS SHOULD NOT HAPPEN! Map info not found for hash ID: ${selectedMapHashId}`);
			}
		}

		mapInfo.element.style.display = 'flex';
	}).on([ArchiveViewState.selectedMapHashId]);

	return archiveMapPreview;
}

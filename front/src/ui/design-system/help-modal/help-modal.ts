import { Br, Div, P, Span, Strong } from '^lib/reactive/html-node.elements.ts';

import { Modal, type ModalInterface } from '^ds/modal/modal.ts';

import { AppState } from '^state/app-state';
import { HelpViewState } from '^state/help-view-state';

import { getSettings } from '^actions/get-settings';

import { Inset } from '../inset/inset';
import { Outset } from '../outset/outset';
import { Screen } from '../screen/screen';

import { Heading3 } from '../headings/headings';
import { StandardButton } from '../buttons/standard-button';

import styles from './help-modal.module.css';
import { Effect } from '../../../lib/reactive/effect.class';
import { ArchiveViewState } from '../../../app-state/archive-view-state';
import { openDirPathInFileExplorer } from '../../../actions/open-dir-path-in-file-explorer';


export function HelpModal() {
	let maxPath, savesPath, archivePath,
		installedMapCount, installedSavesCount,
		archivedMapCount, archivedSavesCount,
		doneButton;

	const modal = Modal().nodes([
		Outset(8, [styles.helpModal]).classes('fill-all pv-16 ph-24').nodes([
			Heading3('MMM: M.A.X. Map Manager Help').classes('mb-16', 'text-center'),
			Inset(2),
			Div().nodes([
				Inset(8, ['w-full']).nodes([
					Screen(HelpViewState, false, ['p-16']).nodes([
						Strong().text('Paths:'),
						Div().class('indent-32').nodes([
							Div().nodes([
								Div().text('M.A.X. directory path'),
								maxPath = Div().class('pointer indent-32 text-yellow').text(''),
							]),
							Div().nodes([
								Div().text('Save files directory path'),
								savesPath = Div().class('pointer indent-32 text-yellow').text(''),
							]),
							Div().nodes([
								Div().text('Archive directory path'),
								archivePath = Div().class('pointer indent-32 text-yellow').text(''),
							]),
						]),
						Strong().text('Assets:'),
						Div().class('indent-32').nodes([
							Div().text('Installed maps and save files'),
							Div().class('indent-32 text-yellow').nodes([
								installedMapCount = Span().text('0'),
								Span().text(' maps, '),
								installedSavesCount = Span().text('{0xf920c4ad..i-}'),
								Span().text(' save files'),
							]),
							Div().text('Archived maps and save files'),
							Div().class('indent-32 text-yellow').nodes([
								archivedMapCount = Span().text('0'),
								Span().text(' maps, '),
								archivedSavesCount = Span().text('0'),
								Span().text(' save files'),
							]),
						]),
					]),
				]),
			]),
			Inset(2),
			Div().classes('flex flex-row flex-spaced mt-16').nodes([
				doneButton = StandardButton('Done'),
			]),
		]),
	]);

	// --- Logic ---

	maxPath.addEventListener('click', function () {
		openDirPathInFileExplorer(maxPath.element.textContent || '');
	});

	savesPath.addEventListener('click', function () {
		openDirPathInFileExplorer(savesPath.element.textContent || '');
	});

	archivePath.addEventListener('click', function () {
		openDirPathInFileExplorer(archivePath.element.textContent || '');
	});

	// @ts-ignore
	window.addEventListener('app-view-changed', async (event: CustomEvent) => {
		const viewName = event.detail;

		if (viewName === HelpViewState.name) {
			const settings = await getSettings();

			maxPath.element.textContent = settings.maxPath || '/home/user/MAX/';;
			savesPath.element.textContent = settings.savesPath || '/home/user/max-port/';
			archivePath.element.textContent = settings.archivePath || '/home/user/max-custom-maps/';
		}
	});

	const _open = modal.x.open as ModalInterface['open'];

	modal.x.open = function () {
		AppState.focusView(HelpViewState);
		_open();
	};

	modal.x.close = function () {
		modal.element.remove();
		AppState.focusPreviousView();
	};

	doneButton.addEventListener('click', function () {
		modal.x.close();
	});

	new Effect(function updateHelpModalStats() {
		installedMapCount.text(`${AppState.mapsInfo.value.size}`);
		archivedMapCount.text(`${ArchiveViewState.mapsInfo.value.size}`);
		archivedSavesCount.text(`${ArchiveViewState.mapsAndSaves.value.size}`);
	}).on([
		AppState.mapsInfo,
		ArchiveViewState.mapsInfo,
		ArchiveViewState.mapsAndSaves,
	]);

	return modal;
}

export const helpModal = HelpModal();

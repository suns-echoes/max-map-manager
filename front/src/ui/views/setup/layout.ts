import { SetupViewState } from '^state/setup-view-state';

import { B, Br, Div, P, Section, Strong, TextInput } from '^lib/reactive/html-node.elements';

import { openFolderSelectDialog } from '^actions/open-folder-select-dialog';
import { checkGamePath } from '^actions/check-game-path';
import { savePaths } from '^actions/save-paths';
import { checkIfPathIsDirectory } from '^actions/check-if-path-is-directory';
import { getSettings } from '^actions/get-settings';
import { closeApp } from '^actions/close-app';

import { Modal } from '^ds/modal/modal';
import { Outset } from '^ds/outset/outset';
import { Heading1 } from '^ds/headings/headings';
import { Inset } from '^ds/inset/inset';
import { Screen } from '^ds/screen/screen';
import { StandardButton } from '^ds/buttons/standard-button';
import { StandardBrokenButton } from '^ds/buttons/standard-broken-button';
import { SquareButton } from '^ds/buttons/square-button';
import { SquareBrokenButton } from '^ds/buttons/square-broken-button';

import styles from './layout.module.css';


interface ConfirmModal {
	open: () => Promise<void>;
	close: (shouldConfirm?: boolean) => void;
}

export function SetupWindowLayout() {
	let browseForMax, browseForSaves, browseForArchive,
		inputMaxPath, inputSavesPath, inputArchivePath,
		quitButton, continueButton, dbgout;

	const setupWindowLayout = (
		Modal().nodes([
			Section().classes(styles.setupWindowLayout).nodes([
				Outset(8).classes('flex flex-col gap-8 p-16').nodes([
					Div().class(styles.title).nodes([
						Heading1('M.A.X. Map Manager'),
						Inset(4, ['m-16	inline-block']).nodes([
							Screen(SetupViewState, true, ['pv-4 w-256p']).nodes([
								Div().class('text-big text-green').text('SETUP'),
							]),
						]),
					]),
					Inset(4, [styles.content]).classes('p-8').nodes([
						Inset(4).nodes([
							Screen(SetupViewState, true, ['p-8']).nodes([
								Div().classes('flex flex-spread').nodes([
									B().text('M.A.X. directory path'),
									Div().classes('flex gap-16').nodes([
										Div().text('. . . . . . . . . . . . . . . . . .'),
										browseForMax = Div().classes('text-blue pointer text-bold').text('[ BROWSE ]'),
									]),
								]),
								Div().classes('flex flex-spread').nodes([
									Div().html('&lt;&nbsp;'),
									inputMaxPath = TextInput().classes('text-blue').value('/home/user/MAX/'),
								]),
								Br(),
								Div().classes('flex flex-spread').nodes([
									B().text('Save files directory path'),
									Div().classes('flex gap-16').nodes([
										Div().text('. . . . . . . . . . . . . . . . . . .'),
										browseForSaves = Div().classes('text-blue pointer text-bold').text('[ BROWSE ]'),
									]),
								]),
								Div().classes('flex flex-spread').nodes([
									Div().html('&lt;&nbsp;'),
									inputSavesPath = TextInput().classes('text-blue').value('/home/user/max-port/'),
								]),
								Br(),
								Div().classes('flex flex-spread').nodes([
									B().text('Archive directory path'),
									Div().classes('flex gap-16').nodes([
										Div().text('. . . . . . . . . . . . . . . . . . . . .'),
										browseForArchive = Div().classes('text-blue pointer text-bold').text('[ BROWSE ]'),
									]),
								]),
								Div().classes('flex flex-spread').nodes([
									Div().html('&lt;&nbsp;'),
									inputArchivePath = TextInput().classes('text-blue').value('/home/user/max-custom-maps/'),
								]),
								Br(),
								Br(),
								Div().text('> dbgout:'),
								dbgout = Div().classes('text-red').text('Please provide all valid paths before continuing...'),
							]),
						]),
					]),
					Div().classes('flex flex-spread').nodes([
						Inset(4, ['inline-block']).classes('flex flex-spread gap-4 p-1').nodes([
							SquareButton(40, 'F1'),
							SquareBrokenButton(40, 'F:'),
							SquareBrokenButton(40, 'F3'),
							SquareButton(40, 'F4'),
						]),
						Inset(4, ['inline-block']).classes('flex flex-spread gap-4 p-4').nodes([
							quitButton = StandardButton('QUIT'),
							StandardBrokenButton('Opt:ou .'),
							StandardBrokenButton('About'),
							continueButton = StandardButton('Continue'),
						]),
					]),
				]),
			]),
		])
	);

	// == Confirm Modal ==

	let confirmModalCancel, confirmModalConfirm;

	const confirmModal = Modal<ConfirmModal>().nodes([
		Section().classes(styles.confirmModal).nodes([
			Outset(8).classes('flex flex-col gap-8 p-16').nodes([
				Heading1('Confirm changes'),
				Inset(2),
				Div().class(styles.confirmModalContent).nodes([
					Div().text('Are you sure you want to save these paths and continue?'),
					Br(),
					Div().text('The Archive directory will store your map and save files that are not currently used by the game. If you remove this directory you will lose access to those maps and saves!'),
				]),
				Inset(2),
				Div().classes('flex flex-spread').nodes([
					confirmModalCancel = StandardButton('Cancel'),
					confirmModalConfirm = StandardButton('Confirm'),
				]),
			]),
		]),
	]);

	const _confirmModalX = { ...confirmModal.x };
	let _confirmModalPromiseResolvers: PromiseWithResolvers<void> | null = null;
	confirmModal.x = {
		open: () => {
			_confirmModalPromiseResolvers = Promise.withResolvers();
			_confirmModalX.open();
			return _confirmModalPromiseResolvers.promise;
		},
		close: (shouldConfirm?: boolean) => {
			if (shouldConfirm) {
				_confirmModalPromiseResolvers!.resolve();
			} else {
				_confirmModalPromiseResolvers!.reject();
			}
			_confirmModalX.close();
		},
	};

	confirmModalCancel.addEventListener('click', () => {
		confirmModal.x.close(false);
	});

	confirmModalConfirm.addEventListener('click', () => {
		confirmModal.x.close(true);
	});

	// == Setup Window Logic ==

	quitButton.addEventListener('click', () => {
		closeApp();
	});

	// @ts-ignore
	window.addEventListener('app-view-changed', async (event: CustomEvent) => {
		const viewName = event.detail;
		const settings = await getSettings();

		if (viewName === SetupViewState.name) {
			inputMaxPath.element.value = settings.maxPath || '/home/user/MAX/';
			inputSavesPath.element.value = settings.savesPath || '/home/user/max-port/';
			inputArchivePath.element.value = settings.archivePath || '/home/user/max-custom-maps/';
		}
	});

	browseForMax.addEventListener('mouseenter', () => {
		dbgout.text('Browse for M.A.X. game installation directory.');
	});

	browseForMax.addEventListener('click', async () => {
		const selectedPath = await openFolderSelectDialog('Select M.A.X. game directory', inputMaxPath.element.value);
		if (selectedPath) {
			inputMaxPath.element.value = selectedPath;
		}
	});

	inputMaxPath.addEventListener('mouseenter', () => {
		dbgout.text('Input the path to the M.A.X. game installation directory.');
	});

	browseForSaves.addEventListener('mouseenter', () => {
		dbgout.text('Browse for save files directory.');
	});

	browseForSaves.addEventListener('click', async () => {
		const selectedPath = await openFolderSelectDialog('Select save files directory', inputSavesPath.element.value);
		if (selectedPath) {
			inputSavesPath.element.value = selectedPath;
		}
	});

	inputSavesPath.addEventListener('mouseenter', () => {
		dbgout.text(`
			Input the path to the save files directory.
			This is usually the "MAX" game or "max-port" directory.
		`);
	});

	browseForArchive.addEventListener('mouseenter', () => {
		dbgout.text('Browse for Archive directory.');
	});

	browseForArchive.addEventListener('click', async () => {
		const selectedPath = await openFolderSelectDialog('Select Archive directory', inputArchivePath.element.value);
		if (selectedPath) {
			inputArchivePath.element.value = selectedPath;
		}
	});

	inputArchivePath.addEventListener('mouseenter', () => {
		dbgout.nodes([
			P().text('Input the path to the Archive directory.'),
			P().text('(it should be manually created)'),
			P().text('This is where map files are stored for later use.'),
			Strong().text('Warning: removing this directory may lead to loss of maps and saves!'),
		]);
	});

	continueButton.addEventListener('click', async () => {
		try {
			console.log('Opening confirm modal...');
			await confirmModal.x.open();
			console.log('Confirm modal closed with confirm.');
		} catch {
			console.log('Confirm modal was cancelled.');
			return;
		}

		const maxPath = inputMaxPath.element.value;
		const savesPath = inputSavesPath.element.value;
		const archivePath = inputArchivePath.element.value;

		dbgout.text('Validating provided paths...');

		const isGamePathValid = await checkGamePath(maxPath);
		const isSavesPathValid = await checkIfPathIsDirectory(savesPath);
		const isArchivePathValid = await checkIfPathIsDirectory(archivePath);

		if (!isGamePathValid) {
			dbgout.text('Error: The provided M.A.X. game directory path is invalid. Please correct it before continuing.');
			return;
		}

		if (!isSavesPathValid) {
			dbgout.text('Error: The provided save files directory path is invalid. Please correct it before continuing.');
			return;
		}

		if (!isArchivePathValid) {
			dbgout.text('Error: The provided Archive directory path is invalid. Please correct it before continuing.');
			return;
		}

		if (!await savePaths({
			maxPath,
			savesPath,
			archivePath,
		})) {
			dbgout.text(`Error: Failed to save paths configuration. Please check debug output for details.`);
			return;
		}

		dbgout.text('All paths are valid. Saving configuration and continuing...');

		setTimeout(() => {
			top!.location.reload();
		}, 200);
	});

	return setupWindowLayout;
}

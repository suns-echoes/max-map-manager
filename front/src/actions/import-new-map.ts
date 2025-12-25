import { MainViewState } from '^state/main-view-state';

import { ErrorModal } from '^ds/error-modal/error-modal';

import { openFileSelectDialog } from './open-file-select-dialog';
import { api } from '../api';
import { putMapInArchive } from './put-map-in-archive';


/**
 * Opens a file picker and copies the selected map into the archive.
 * Note: If one file is selected, and installation slot is empty, map will be installed instead.
 * @returns {boolean} Returns `true` if map(s) was imported, and `false` otherwise.
 */
export async function importNewMap(selectedSlotName: string): Promise<boolean> {
	const fileHandles = await openFileSelectDialog(
		'Select M.A.X. WRL Map Files to Import',
		undefined,
		true,
		false,
		[
			{
				name: 'M.A.X. WRL Map Files',
				extensions: ['WRL'],
			},
		],
	);

	if (!Array.isArray(fileHandles)) {
		return Promise.resolve(false);
	}

	if (fileHandles.length === 0) {
		return Promise.resolve(false);
	}

	if (fileHandles.length === 1) {
		const selectedSlotMapHashId = MainViewState.selectedMapHashId.value;
		if (selectedSlotMapHashId !== null) {
			if ((await putMapInArchive(selectedSlotMapHashId))) {
				console.log(`Archived existing map in slot ${selectedSlotName} before importing new map.`);
			} else {
				console.error(`Failed to archive existing map in slot ${selectedSlotName}. Aborting import.`);
				ErrorModal({
					title: 'Import Error',
					message: `Failed to archive existing map in slot ${selectedSlotName}. Aborting import.`,
				}).x.open();
				return false;
			}
		}

		const result = await api.installImportedMap(fileHandles[0], selectedSlotName);
		if (!result.ok) {
			console.error('Failed to import map:', result.error);
			ErrorModal({
				title: 'Import Error',
				message: `Failed to import map: ${result.error}`,
			}).x.open();
			return false;
		}
		return result.ok;
	} else {
		// TODO: Implement importing multiple maps to archive
		// const result = await api.importMapsToArchive(fileHandles);
		// return result.ok;
		ErrorModal({
			title: 'Import Error',
			message: 'Importing multiple maps at once is not yet implemented.',
		}).x.open();
		return Promise.resolve(false);
	}
}

import { AppState } from '^state/app-state';
import { ArchiveViewState } from '^state/archive-view-state';

import { restoreMapAndSaves } from '^api/restore-map-and-saves-command';

import { putMapInArchive } from './put-map-in-archive';


export async function swapMaps(
	toInstallMapHashId: MapHashId | null,
	toArchiveMapHashId: MapHashId | null,
): Promise<void> {
	if (toArchiveMapHashId !== null) {
		await putMapInArchive(toArchiveMapHashId);
	}

	if (toInstallMapHashId !== null) {
		await restoreMapAndSaves(toInstallMapHashId);
	}

	ArchiveViewState.selectedMapHashId.value = toArchiveMapHashId;
	await AppState.update();
	ArchiveViewState.selectedMapHashId.emit();
}

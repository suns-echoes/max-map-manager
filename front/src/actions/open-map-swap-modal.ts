import { mapSwapModal } from '../ui/design-system/map-swap-modal/map-swap-modal';


export function openMapSwapModal(
	installedMapInfo: MapInfo | null,
	archivedMapInfo: MapInfo | null,
): boolean {
	if (installedMapInfo === null && archivedMapInfo === null) {
		return false;
	}
	mapSwapModal.x.open(
		installedMapInfo,
		archivedMapInfo,
	);
	return true;
}

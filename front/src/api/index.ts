import { archiveMapAndSaves } from './archive-map-and-saves';
import { generateBigMapPreview } from './generate-bigmap-preview';
import { getArchivedMapsAndSaves } from './get-archived-maps-and-saves';
import { isSetupRequired } from './is-setup-required';
import { readArchivedMapsMetadata } from './read-archived-maps-metadata';
import { readInstalledMapsMetadata } from './read-installed-maps-metadata';
import { readSaveFilesMetadata } from './read-save-files-metadata';
import { readSettings } from './read-settings';
import { restoreMapAndSaves } from './restore-map-and-saves-command';
import { getInstalledMapsAndSaves } from './get-installed-maps-and-saves';
import { verifyDirPath } from './verify-dir-path';
import { verifyGamePath } from './verify-max-path';
import { setAppPaths } from './set-app-paths';


export const api = {
	archiveMapAndSaves,
	generateBigMapPreview,
	getArchivedMapsAndSaves,
	isSetupRequired,
	readArchivedMapsMetadata,
	readInstalledMapsMetadata,
	readSaveFilesMetadata,
	readSettings,
	restoreMapAndSaves,
	setAppPaths,
	scanMapsAndSaves: getInstalledMapsAndSaves,
	verifyDirPath,
	verifyGamePath,
};

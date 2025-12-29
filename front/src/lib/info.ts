import { resourceDir, appDataDir } from '@tauri-apps/api/path';
import packageJson from '../../../package.json' assert { type: 'json' };

type PackageJson = { version?: string };

export function getAppVersion(): string {
	return (packageJson as PackageJson).version || 'unknown';
}

export async function getInstallationPath(): Promise<string> {
	return resourceDir();
}

export async function getAppDataPath(): Promise<string> {
	return appDataDir();
}

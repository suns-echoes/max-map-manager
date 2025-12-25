import { open } from '@tauri-apps/plugin-dialog';


export async function openFileSelectDialog(
	title: string,
	defaultPath?: string,
	multiple = false,
	canCreateDirectories = false,
	filters?: { name: string; extensions: string[] }[],
): Promise<string[] | null> {
	const selected = await open({
		title,
		directory: false,
		multiple,
		canCreateDirectories,
		filters: filters ?? [
			{
				name: 'All Files',
				extensions: ['*'],
			},
		],
		defaultPath,
	});

	if (Array.isArray(selected) || selected === null) {
		return selected;
	} else {
		return [selected];
	}
};

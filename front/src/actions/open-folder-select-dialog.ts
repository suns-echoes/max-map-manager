import { open } from '@tauri-apps/plugin-dialog';


export async function openFolderSelectDialog(
	title: string,
	defaultPath?: string,
): Promise<string | null> {
  const selected = await open({
	title,
	directory: true,
	defaultPath,
  });

  if (Array.isArray(selected)) {
	return selected[0] || null;
  }

  return selected || null;
};

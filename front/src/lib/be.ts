import { convertFileSrc } from '@tauri-apps/api/core';


export function be(resourcePath: string): string {
	return convertFileSrc(resourcePath, 'be')
		.replaceAll('%2f', '/')
		.replaceAll('%2F', '/');
}

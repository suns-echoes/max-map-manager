import { convertFileSrc } from '@tauri-apps/api/core';


export function be(resourcePath: string): string {
	return decodeURIComponent(convertFileSrc(resourcePath, 'be'));
}

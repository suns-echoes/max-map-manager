// == Global Exposure for Hacks ==

import { HTMLNode } from '^lib/reactive/html-node.class';
import * as HTMLNodeElements from '^lib/reactive/html-node.elements';


// == Application Entry Point ==

import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalSize } from '@tauri-apps/api/dpi';

import { listen } from '@tauri-apps/api/event';

import { be } from '^lib/be';
import { sleep } from '^lib/flow/sleep';

import '^events/window-resize-event';

import { AppState } from '^state/app-state';
import { SetupViewState } from '^state/setup-view-state';
import { DisclaimerViewState } from '^state/disclaimer-view-state';
import { MainViewState } from '^state/main-view-state';

import { checkIfSetupIsRequired } from '^actions/check-is-setup-is-required';

import { loadingModal } from '^ds/loading-modal/loading-modal';

import { MainWindowLayout } from './ui/views/main/layout';
import { ArchiveWindowLayout } from './ui/views/archive/layout';

import { SetupWindowLayout } from './ui/views/setup/layout';
import { DisclaimerWindowLayout } from './ui/views/disclaimer/layout';


// == Initial Window Setup ==
async function resizeWindowLogical(width: number, height: number) {
	try {
		const appWindow = getCurrentWindow();
		const newSize = new LogicalSize(width, height);
		await appWindow.setSize(newSize);
	} catch (error) {
		console.error('Failed to resize window:', error);
	}
}

function showSetupView() {
	const setupWindowLayout = SetupWindowLayout();
	document.body.appendChild(setupWindowLayout.element);
	AppState.focusView(SetupViewState);

	setTimeout(() => {
		const disclaimerWindowLayout = DisclaimerWindowLayout();
		document.body.appendChild(disclaimerWindowLayout.element);
		AppState.focusView(DisclaimerViewState);
	}, 100);
}

resizeWindowLogical(1128, 832);


// == Install hacks ==
window._ = [
	HTMLNode,
	HTMLNodeElements,
	void 0,
	closeApp,
	'',
	() => {
		AppState.focusView(MainViewState);
		setTimeout(() => {
			AppState.focusView(SetupViewState);
		}, 300);
	},
	be,
	showSetupView,
	getAppVersion,
	reloadUI,
];

// @ts-ignore
await import('./hacks.js').then(() => {
	console.log('Hacks loaded');
}).catch((error) => {
	console.error('Failed to load hacks:', error);
});




const SHOULD_SHOW_SETUP_VIEW = await checkIfSetupIsRequired();

console.log('SHOULD_SHOW_SETUP_VIEW', SHOULD_SHOW_SETUP_VIEW);


// == Application Setup ==

if (SHOULD_SHOW_SETUP_VIEW) {
	showSetupView();
}

// == Application Initialization ==

else {
	const unlisten = await listen('backend-message:progress', (event) => {
		AppState.progress.set(event.payload as number);
	});

	{
		loadingModal.x.open();
		loadingModal.text('loading maps data');
		loadingModal.x.break(Math.random() * 2000 + 1000);

		await sleep(100);

		const mainWindowLayout = MainWindowLayout();
		document.body.appendChild(mainWindowLayout.element);

		const archiveWindowLayout = ArchiveWindowLayout();
		document.body.appendChild(archiveWindowLayout.element);

		await sleep(100);

		await AppState.update();

		await sleep(100);

		AppState.progress.set(100);
		setTimeout(() => {
			loadingModal.x.close();
			AppState.focusView(MainViewState);
		}, 800);
	}

	unlisten();
}

// == Development Tools ==
import { invoke } from '@tauri-apps/api/core';
import { closeApp } from './actions/close-app.js';
import { getAppVersion } from './lib/info.js';
import { reloadUI } from './actions/reload-ui.js';

if (!import.meta.env.DEV) {
	window.addEventListener('contextmenu', (e) => {
		e.preventDefault();
	});
} else {
	(window as any).openTauriConsole = () => {
		invoke('open_devtools_command');
	};

	window.openTauriConsole();
}

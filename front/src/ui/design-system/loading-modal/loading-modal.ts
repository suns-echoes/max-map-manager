import { AppState } from '^state/app-state';
import { LoadingModalViewState } from '^state/loading-modal-view-state';

import { Effect } from '^lib/reactive/effect.class.ts';
import { Div } from '^lib/reactive/html-node.elements.ts';

import { Modal } from '^ds/modal/modal.ts';

import { Outset } from '../outset/outset';
import { Inset } from '../inset/inset';
import { Screen } from '../screen/screen';

import styles from './loading-modal.module.css';


interface LoadingModal {
	setProgress: (value: number) => void;
	getProgress: () => number;
	break: (delay: number) => void;
}

export function LoadingModal() {
	let progressValue = 0;
	let screen, loadingText, loadingBar;

	const modal = Modal<LoadingModal>().nodes([
		Outset(8, [styles.loadingModal]).classes('flex flex-col gap-8 p-8').nodes([
			Inset(8).nodes([
				screen = Screen(LoadingModalViewState, true, ['p-8']).nodes([
					loadingText = Div().class(styles.loadingText).text('Loading ...'),
				]),
			]),
			Inset(4, ['m-4']).classes(styles.loadingSlotCradle, 'relative').nodes([
				Inset(4, [styles.loadingSlot]).nodes([
					loadingBar = Div().class(styles.loadingBar),
				]),
			]),
		]),
	]);

	modal.text = function (text: string) {
		loadingText.text(text);
		return modal;
	};

	modal.x.setProgress = function (value: number) {
		if (value < 0 || value > 100) {
			throw new Error('Progress value must be between 0 and 100');
		}
		progressValue = value;
		loadingBar.element.style.width = `${value}%`;
	};

	modal.x.getProgress = function () {
		return progressValue;
	};

	modal.x.break = function (duration: number) {
		screen.x.break(duration);
	};

	const _open = modal.x.open;
	const _close = modal.x.close;

	modal.x.open = function () {
		AppState.focusView(LoadingModalViewState);
		_open();
	};

	modal.x.close = function () {
		AppState.focusPreviousView();
		_close();
	};

	new Effect(function () {
		modal.x.setProgress(AppState.progress.value);
	}).on([AppState.progress]);

	return modal;
}


export const loadingModal = LoadingModal();

import { Div } from '^lib/reactive/html-node.elements';

import { ErrorViewState } from '^state/error-view-state';
import { AppState } from '^state/app-state';

import { StandardButton } from '^ds/buttons/standard-button';
import { Heading3 } from '^ds/headings/headings';
import { Modal, ModalInterface } from '^ds/modal/modal';
import { Inset } from '^ds/inset/inset';
import { Screen } from '^ds/screen/screen';
import { Outset } from '^ds/outset/outset';

import styles from './error-modal.module.css';


interface ErrorModalOptions {
	title: string;
	message: string;
	buttons?: {
		custom?: {
			title: string;
			action: () => void;
		};
		done?: boolean;
	};
}


export function ErrorModal(options: ErrorModalOptions) {
	let customButton, doneButton;

	const modal = Modal().nodes([
		Outset(8, [styles.errorModal]).nodes([
			Heading3(options.title).class(styles.title),
			Inset(2),
			Inset(2),
			Inset(2),
			Inset(8).nodes([
				Screen(ErrorViewState).nodes([
					Div().class('p-16').text(options.message),
				]),
			]),
			Inset(2),
			Inset(2),
			Div().class(styles.buttons).nodes([
				customButton = options.buttons?.custom && StandardButton(options.buttons.custom.title),
				doneButton = (options.buttons?.done || options.buttons === undefined) && StandardButton('DONE'),
			]),
		]),
	]);

	const _open = modal.x.open as ModalInterface['open'];

	modal.x.open = function () {
		AppState.focusView(ErrorViewState);
		_open();
	};

	modal.x.close = function () {
		modal.element.remove();
		AppState.focusPreviousView();
	};

	if (customButton && options.buttons?.custom) {
		customButton.addEventListener('click', () => {
			options.buttons?.custom!.action();
		});
	}
	if (doneButton) {
		doneButton.addEventListener('click', () => {
			modal.x.close();
			modal.destroy();
		});
	}

	return modal;
}

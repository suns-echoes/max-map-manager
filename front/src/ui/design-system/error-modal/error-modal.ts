import { Div } from '^lib/reactive/html-node.elements';

import { StandardButton } from '^ds/buttons/standard-button';
import { Heading3 } from '^ds/headings/headings';
import { Modal } from '^ds/modal/modal';

import styles from './error-modal.module.css';


interface ErrorModalOptions {
	title: string;
	message: string;
	buttons: {
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
		Div().class(styles.errorModal).nodes([
			Heading3(options.title).class(styles.title),
			Div().class(styles.message).text(options.message),
			Div().class(styles.buttons).nodes([
				customButton = options.buttons.custom && StandardButton(options.buttons.custom.title),
				doneButton = options.buttons.done && StandardButton('DONE'),
			]),
		]),
	]);

	if (customButton && options.buttons.custom) {
		customButton.addEventListener('click', () => {
			options.buttons.custom!.action();
		});
	}
	if (doneButton) {
		doneButton.addEventListener('click', () => {
			modal.x.close();
		});
	}

	return modal;
}

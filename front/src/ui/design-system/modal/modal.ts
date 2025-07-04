import { type HTMLNodeExtendedProps } from '^lib/reactive/html-node.class';
import { Div } from '^lib/reactive/html-node.elements';

import styles from './modal.module.css';


export interface ModalInterface {
	open: () => void;
	close: () => void;
}

export function Modal<T extends HTMLNodeExtendedProps>() {
	const modal = (
		Div<X<T>>().class(styles.modal)
	);

	modal.x.open = function () {
		document.body.appendChild(modal.element);
	};

	modal.x.close = function () {
		modal.element.remove();
	};

	return modal;
}

type X<T> = ('open' | 'close') extends keyof T
	? Omit<ModalInterface, 'open' | 'close'> & T
	: 'open' extends keyof T
		? Omit<ModalInterface, 'open'> & T
		: 'close' extends keyof T
			? Omit<ModalInterface, 'close'> & T
			: ModalInterface & T;

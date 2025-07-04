import { Button, Div } from '^lib/reactive/html-node.elements';

import styles from './square-broken-button.module.css';


export function SquareBrokenButton(size: number = 32, title?: string) {
	const button = Button().class(styles.squareBrokenButton).style({
		width: `${size}px`,
		height: `${size}px`,
	});
	if (title) {
		button.nodes([
			Div().text(title),
		]);
	}
	return button;
}

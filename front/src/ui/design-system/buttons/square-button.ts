import { Button, Div } from '^lib/reactive/html-node.elements';

import styles from './square-button.module.css';


export function SquareButton(size: number = 32, title?: string) {
	const button = Button().class(styles.squareButton).style({
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

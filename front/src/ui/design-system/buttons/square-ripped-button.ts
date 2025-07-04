import { Button } from '^lib/reactive/html-node.elements';

import styles from './square-ripped-button.module.css';


export function SquareRippedButton(size: number = 32) {
	return (
		Button().class(styles.squareRippedButton).style({
			width: `${size}px`,
			height: `${size}px`,
		})
	);
}

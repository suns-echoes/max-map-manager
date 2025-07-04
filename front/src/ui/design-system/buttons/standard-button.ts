import { Button } from '^lib/reactive/html-node.elements';

import styles from './standard-button.module.css';


export function StandardButton(title?: string) {
	const button = Button().baseClass(styles.standardButton);
	if (title) {
		button.text(title);
	}
	return button;
}

import { Button } from '^lib/reactive/html-node.elements';

import styles from './standard-broken-button.module.css';


export function StandardBrokenButton(title?: string) {
	const button = Button().class(styles.standardBrokenButton);
	if (title) {
		button.text(title);
	}
	return button;
}

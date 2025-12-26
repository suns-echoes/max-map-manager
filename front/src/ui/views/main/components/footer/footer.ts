import { Div, Section } from '^lib/reactive/html-node.elements';

import styles from './footer.module.css';


export function Footer() {
	return (
		Section().classes(styles.footer, 'flex flex-row flex-spread').nodes([
			Div().text('M.A.X. Map Manager © 2025 Aneta Suns'),
			Div().text('v0.9.4'),
		])
	);
}

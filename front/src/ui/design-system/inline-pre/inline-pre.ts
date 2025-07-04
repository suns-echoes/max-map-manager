import { Div } from '^lib/reactive/html-node.elements';

import styles from './inline-pre.module.css';


export function InlinePre(text: string) {
	return Div().text(text).baseClass(styles.inlinePre);
}

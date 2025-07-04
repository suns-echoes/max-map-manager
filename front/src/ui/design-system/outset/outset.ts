import { type HTMLNodeExtendedProps } from '../../../lib/reactive/html-node.class';
import { Div } from '^lib/reactive/html-node.elements';

import styles from './outset.module.css';


export function Outset<X extends HTMLNodeExtendedProps>(size: number, classes?: string[]) {
	let content;

	const outset = (
		Div().classes('relative', `p-${size}`, ...(classes ?? [])).nodes([
			Div().classes(styles.outset, `frame frame-${size}`),
			content = Div<X>().baseClass('fill-all'),
		])
	);

	outset.interface = content;

	return outset;
}

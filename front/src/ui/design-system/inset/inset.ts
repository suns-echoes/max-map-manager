import { type HTMLNodeExtendedProps } from '../../../lib/reactive/html-node.class';
import { Div } from '^lib/reactive/html-node.elements';

import styles from './inset.module.css';


export function Inset<X extends HTMLNodeExtendedProps>(size: number, classes?: string[]) {
	let content;

	const inset = (
		Div().classes('relative', `p-${size}`, ...(classes ?? [])).nodes([
			Div().classes(styles.inset, `frame frame-${size}`),
			content = Div<X>().baseClass('fill-all'),
		])
	);

	inset.interface = content;

	return inset;
}

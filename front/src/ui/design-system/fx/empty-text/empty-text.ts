import { Span } from '^lib/reactive/html-node.elements'


export function EmptyText() {
	const variant = (Math.random() * 5) | 0;
	const darken = '0.5';

	return Span().nodes([
		Span().text('['),
		Span().text('e').style({ opacity: variant === 0 ? '1' : darken }),
		Span().text('m').style({ opacity: variant === 4 ? darken : '1' }),
		Span().text('p').style({ opacity: variant  >  2 ? darken : '1' }),
		Span().text('t').style({ opacity: variant === 4 ? darken : '1' }),
		Span().text('y').style({ opacity: variant  >  1 ? darken : '1' }),
		Span().text(']'),
	]);
}

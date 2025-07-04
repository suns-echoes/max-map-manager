import { Div, H1, H2, H3, H4 } from '^lib/reactive/html-node.elements';


export function Heading1(text: string) {
	const h1 = (
		H1().nodes([
			Div().text(text),
			Div().text(text),
		])
	);

	h1.text = function (text: string) {
		const labels = h1.element.children;
		labels[0].textContent = text;
		labels[1].textContent = text;
		return h1;
	}

	return h1;
}

export function Heading2(text: string) {
	const h2 = (
		H2().nodes([
			Div().text(text),
			Div().text(text),
		])
	);

	h2.text = function (text: string) {
		const labels = h2.element.children;
		labels[0].textContent = text;
		labels[1].textContent = text;
		return h2;
	}

	return h2;
}

export function Heading3(text: string) {
	const h3 = (
		H3().nodes([
			Div().text(text),
			Div().text(text),
		])
	);

	h3.text = function (text: string) {
		const labels = h3.element.children;
		labels[0].textContent = text;
		labels[1].textContent = text;
		return h3;
	}

	return h3;
}

export function Heading4(text: string, classes?: string[]) {
	const h4 = (
		H4().nodes([
			Div().text(text),
			Div().text(text),
		]).classes(...(classes || []))
	);

	h4.text = function (text: string) {
		const labels = h4.element.children;
		labels[0].textContent = text;
		labels[1].textContent = text;
		return h4;
	}

	return h4;
}

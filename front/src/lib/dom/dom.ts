export function getElementAbsolutePosition(element: HTMLElement) {
	const rect = element.getBoundingClientRect();
	const scrollLeft = window.pageXOffset || document.documentElement.scrollLeft;
	const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
	return { top: rect.top + scrollTop, left: rect.left + scrollLeft };
}

export function getElementVisibleHeight(element: HTMLElement) {
	const rect = element.getBoundingClientRect();
	const windowHeight = (window.innerHeight || document.documentElement.clientHeight);

	if (rect.bottom <= 0 || rect.top >= windowHeight) {
		return 0;
	}

	const visibleTop = Math.max(rect.top, 0);
	const visibleBottom = Math.min(rect.bottom, windowHeight);

	return visibleBottom - visibleTop;
}

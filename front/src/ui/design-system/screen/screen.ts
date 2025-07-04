import { Div } from '^lib/reactive/html-node.elements';

import { ViewState } from '^state/app-state';

import { getElementAbsolutePosition, getElementVisibleHeight } from '^lib/dom/dom';

import styles from './screen.module.css';


export interface Screen {
	break: (duration: number) => void;
	turnOff: () => void;
	turnOn: () => void;
	onBreak?: () => void;
}

function getNextAnimationDelay(element: HTMLElement) {
	let { top, left } = getElementAbsolutePosition(element);
	return (top * 14 + left * 1.33) + 'ms';
}

function getNextAnimationDuration(element: HTMLElement) {
	let height = getElementVisibleHeight(element);
	return (Math.log(height) * 4000) + 'ms';
}

export function Screen(viewState: ViewState, breakable: boolean = false, classes: string[] = []) {
	let glass, content, staticImg;

	const screen = (
		Div<Screen>().class(styles.screen).nodes([
			glass = Div().baseClass(styles.glass).nodes([
				content = Div().classes(styles.content, ...classes),
				staticImg = breakable ? Div().class(styles.static).style({ display: 'none' }) : null,
			]),
		])
	);

	setTimeout(() => {
		glass.element.style.animationDelay = getNextAnimationDelay(glass.element);
		glass.element.style.animationDuration = getNextAnimationDuration(glass.element);
		glass.element.classList.add(styles.glassAnimation);
	}, 500);

	screen.interface = content;

	let timer: any = null;
	let clickCount = 0;
	let showStatic = false;
	let isBroken = false;

	viewState.viewScreens.push(screen);

	screen.onDestroy(() => {
		viewState.viewScreens = viewState.viewScreens.filter((s) => s !== screen);
		if (timer) {
			clearTimeout(timer);
			timer = null;
		}
	});

	function enableStatic() {
		showStatic = true;
		content!.element.style.visibility = 'hidden';
		staticImg!.element.style.display = 'block';
	}

	function disableStatic() {
		showStatic = false;
		content!.element.style.visibility = 'visible';
		staticImg!.element.style.display = 'none';
	}

	let timeoutId: ReturnType<typeof setTimeout> | null = null;
	screen.x.break = (duration: number) => {
		if (timeoutId) {
			clearTimeout(timeoutId);
			timeoutId = null;
		}
		enableStatic();
		timeoutId = setTimeout(() => {
			disableStatic();
			timeoutId = null;
		}, duration);
	};

	screen.x.turnOff = () => {
		content.element.classList.add(styles.flicker);
		setTimeout(function () {
			glass.element.classList.remove(styles.glassAnimation);
			content.element.classList.remove(styles.flicker);
			content.element.style.visibility = 'hidden';
			setTimeout(() => {
				glass.baseClass(styles.dummyGlass);
			}, 500);
		}, (Math.random() * 300) | 0);
	};

	screen.x.turnOn = () => {
		glass.baseClass(styles.glass);
		glass.element.classList.add(styles.glassAnimation);
		content.element.classList.add(styles.flicker);
		setTimeout(function () {
			// glass.element.style.animationDelay = getNextAnimationDelay(glass.element);
			content.element.style.visibility = 'visible';
			setTimeout(function () {
				content.element.classList.remove(styles.flicker);
			}, (Math.random() * 400) | 0);
		}, (Math.random() * 300) | 0);
	};

	if (breakable) {
		screen.addEventListener('click', () => {
			if (showStatic) {
				return;
			}

			if (timer) {
				clearTimeout(timer);
			}

			clickCount++;
			content.element.classList.add(styles.flicker);

			timer = setTimeout(function () {
				content.element.classList.remove(styles.flicker);
				clickCount--;
			}, (Math.random() * 1000) | 0);

			if (clickCount > 5) {
				enableStatic();
				setTimeout(() => {
					disableStatic();
					if (!isBroken) {
						isBroken = false;
						screen.x?.onBreak?.();
					}
					clickCount = 0;
				}, (Math.random() * 5000) | 0);
			}
		});
	}

	return screen;
}

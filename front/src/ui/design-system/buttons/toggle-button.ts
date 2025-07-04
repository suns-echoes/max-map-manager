import { Button } from '^lib/reactive/html-node.elements';

import styles from './toggle-button.module.css';


interface ToggleButton {
	toggle(isActive?: boolean): void;
}

export function ToggleButton(title?: string) {
	const button = Button<ToggleButton>().class(styles.toggleButton);

	if (title) {
		button.text(title);
	}

	let isButtonActive = false;

	button.x.toggle = function (isActive?: boolean) {
		const active = isActive !== undefined ? isActive : !isButtonActive;
		if (active) {
			button.element.classList.add(styles.active);
		} else {
			button.element.classList.remove(styles.active);
		}
	};

	button.addEventListener('click', function () {
		button.x.toggle(!button.element.classList.contains(styles.active));
	});

	return button;
}

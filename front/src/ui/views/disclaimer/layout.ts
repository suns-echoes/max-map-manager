import { exit } from '@tauri-apps/plugin-process';

import { DisclaimerViewState } from '^state/disclaimer-view-state';

import { Modal } from '../../design-system/modal/modal';

import styles from './layout.module.css';
import { Div, Section } from '../../../lib/reactive/html-node.elements';
import { Outset } from '../../design-system/outset/outset';
import { Heading1 } from '../../design-system/headings/headings';
import { Inset } from '../../design-system/inset/inset';
import { Screen } from '../../design-system/screen/screen';
import { StandardButton } from '../../design-system/buttons/standard-button';
import { StandardBrokenButton } from '../../design-system/buttons/standard-broken-button';
import { SquareButton } from '../../design-system/buttons/square-button';
import { SquareBrokenButton } from '../../design-system/buttons/square-broken-button';
import { AppState } from '../../../app-state/app-state';
import { SetupViewState } from '../../../app-state/setup-view-state';


export function DisclaimerWindowLayout() {
	const DISCLAIMER_TEXT = window._[4];

	let textScrollToTop, textScrollUp, textScrollDown,
		rejectButton, acceptButton;

	const disclaimerWindowLayout = (
		Modal().nodes([
			Section().classes(styles.disclaimerWindowLayout).nodes([
				Outset(8).classes('flex flex-col gap-8 p-16').nodes([
					Div().class(styles.title).nodes([
						Heading1('M.A.X. Map Manager'),
						Inset(4, ['m-16	inline-block']).nodes([
							Screen(DisclaimerViewState, true, ['pv-4 w-256p']).nodes([
								Div().class('text-big text-red').text('DISCLAIMER'),
							]),
						]),
					]),
					Inset(4, [styles.content]).classes('p-8').nodes([
						Inset(4).nodes([
							Screen(DisclaimerViewState, true, ['p-8']).nodes([
								Div().id('disclaimer-text').class(styles.disclaimerText).html(DISCLAIMER_TEXT),
							]),
						]),
					]),
					Div().classes('flex flex-spread').nodes([
						Inset(4, ['inline-block']).classes('flex flex-spread gap-4 p-1').nodes([
							textScrollToTop = SquareButton(32, '■'),
							textScrollDown = SquareButton(32, '▲'),
							textScrollUp = SquareButton(32, '▼'),
							SquareBrokenButton(32, '☣'),
						]),
						Inset(4, ['inline-block']).classes('flex flex-spread gap-4 p-4').nodes([
							rejectButton = StandardButton('REJECT').id('reject-button'),
							Div().nodes([
								StandardBrokenButton('HESI_/ E'),
							]),
							acceptButton = StandardButton('I ACCEPT'),
						]),
					]),
				]),
			]),
		])
	);

	textScrollToTop.addEventListener('click', () => {
		disclaimerWindowLayout.element.querySelector(`.${styles.disclaimerText}`)?.scrollTo({ top: 0, behavior: 'smooth' });
	});

	textScrollUp.addEventListener('click', () => {
		disclaimerWindowLayout.element.querySelector(`.${styles.disclaimerText}`)?.scrollBy({ top: 100, behavior: 'smooth' });
	});

	textScrollDown.addEventListener('click', () => {
		disclaimerWindowLayout.element.querySelector(`.${styles.disclaimerText}`)?.scrollBy({ top: -100, behavior: 'smooth' });
	});

	rejectButton.addEventListener('click', () => {
		if (rejectButton.element.dataset.hacked === 'true') return;
		exit(0);
	});

	acceptButton.addEventListener('click', () => {
		disclaimerWindowLayout.x.close();
		AppState.focusView(SetupViewState);
	});

	return disclaimerWindowLayout;
}

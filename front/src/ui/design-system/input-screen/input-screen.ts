import { TextInput } from '^lib/reactive/html-node.elements';

import { ViewState } from '^state/app-state';

import { Screen } from '../screen/screen';


export function InputScreen(viewState: ViewState, value?: string) {
	let input;

	const inputScreen = (
		Screen(viewState, false).nodes([
			input = TextInput().value(value ?? '')
		])
	);

	inputScreen.interface = input;

	return inputScreen;
}

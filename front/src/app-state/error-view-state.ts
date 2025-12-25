import { HTMLNode } from '^lib/reactive/html-node.class';

import { Screen } from '^ds/screen/screen';
import { Value } from '../lib/reactive/value.class';


export class ErrorViewState {
	static name = 'ERROR_VIEW_STATE';

	static selectedMapHashId = new Value<MapHashId | null>(null);
	static selectedMapSlotIndex = new Value<number>(0);

	static viewScreens: HTMLNode<any, Screen>[] = [];

	static blur() {
		this.viewScreens.forEach(screenNode => {
			screenNode.x.turnOff();
		});
	}

	static focus() {
		this.viewScreens.forEach(screenNode => {
			screenNode.x.turnOn();
		});
	}
}

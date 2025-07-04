import type { ReactiveEventMessage, ReactiveTargetCallback } from '../types/reactive.types.ts';


export class ReactiveEventTarget<E = any, P = any> {
	public destroy(): void {
		this.eventTargetMap.clear();
	}

	public eventTargetMap: Map<E, Set<ReactiveTargetCallback<E, P>>> = new Map();

	public dispatch(eventMessage: ReactiveEventMessage): void {
		const targets = this.eventTargetMap.get(eventMessage.type);
		if (!targets || targets.size === 0) {
			return;
		}
		for (const updater of [...targets]) {
			updater(eventMessage);
		}
	}

	public addEventHandler(type: E, target: ReactiveTargetCallback<E, P>): void {
		if (!this.eventTargetMap.has(type)) {
			this.eventTargetMap.set(type, new Set());
		}
		this.eventTargetMap.get(type)!.add(target);
	}

	public removeEventHandler(type: E, updater: ReactiveTargetCallback<E, P>): void {
		const targets = this.eventTargetMap.get(type);
		if (targets) {
			targets.delete(updater);
			if (targets.size === 0) {
				this.eventTargetMap.delete(type);
			}
		}
	}

	public removeAllEventHandlers(type?: E): void {
		if (type === undefined) {
			this.eventTargetMap.clear();
			return;
		} else {
			this.eventTargetMap.delete(type);
		}
	}
}

import type { ReactiveEventMessage, ReactiveTargetCallback } from './types/reactive.types.ts';
import type { ReactiveSource } from './base/reactive-source.class.ts';
import type { ReactiveTarget } from './base/reactive-target.class.ts';
import type { ReactiveScope } from './base/reactive-scope.class.ts';
import { ReactiveEventTarget } from './base/reactive-event-target.ts';
import { appReactiveScope } from './base/reactive-scope.class.ts';


/**
 * Reactive Signal Class.
 *
 * This class implements the ReactiveSource and ReactiveTarget interfaces,
 * allowing it to act as both a source and a target for reactions.
 */
export class Signal<E = any, P = any> extends ReactiveEventTarget<E, P> implements ReactiveSource, ReactiveTarget {
	constructor(updateCallback?: ReactiveTargetCallback<E, P>) {
		super();
		if (updateCallback) {
			this.executorCallback = () => {
				updateCallback();
				this._notify();
			};
		}
	}

	public destroy(): void {
		this.observers.clear();
		if (this.scope) {
			this.scope.reactiveObjects.delete(this);
			this.scope = null!;
		}
		this.executorCallback = null;
	}


	public scope: ReactiveScope = appReactiveScope;

	public executorCallback: ReactiveTargetCallback<E, P> | null = null;

	public observers = new Set<ReactiveTargetCallback<E, P>>();


	public emit(): this {
		this._notify();
		return this;
	}


	public update = (eventMessage?: ReactiveEventMessage): this => {
		this.scope.taskQ.push(() => {
			if (this.executorCallback) {
				this.executorCallback(eventMessage);
			}
		});
		return this;
	}

	public on(sources: ReactiveSource[]): this {
		for (const source of sources) {
			source.observers.add(this.update);
		}
		return this;
	}

	public off(sources: ReactiveSource[]): this {
		for (const source of sources) {
			source.observers.delete(this.update);
		}
		return this;
	}


	private _notify = (): void => {
		for (const observer of this.observers) {
			observer();
		}
	}
}

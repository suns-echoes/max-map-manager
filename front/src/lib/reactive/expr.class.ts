import type { ReactiveComputeCallback, ReactiveEventMessage, ReactiveTargetCallback } from './types/reactive.types.ts';
import type { ReactiveSource } from './base/reactive-source.class.ts';
import type { ReactiveTarget } from './base/reactive-target.class.ts';
import type { ReactiveScope } from './base/reactive-scope.class.ts';
import { ReactiveEventTarget } from './base/reactive-event-target.ts';
import { appReactiveScope } from './base/reactive-scope.class.ts';


/**
 * Reactive Expression Class.
 *
 * Implements both ReactiveSource and ReactiveTarget, enabling it to act as a reactive intermediary.
 * Computes derived values from multiple sources and updates automatically when dependencies change.
 */
export class Expr<T, E = any, P = any> extends ReactiveEventTarget<E, P> implements ReactiveSource, ReactiveTarget {
	constructor(recomputeCallback: ReactiveComputeCallback<T, E, P>, initialValue: T) {
		super();
		this.value = initialValue;
		this.executorCallback = () => {
			this.value = recomputeCallback(this.value);
			this._notify();
		};
	}

	public destroy(): void {
		this.observers.clear();
		if (this.scope) {
			this.scope.reactiveObjects.delete(this);
			this.scope = null!;
		}
		this.executorCallback = null!;
	}


	public scope: ReactiveScope = appReactiveScope;

	public value: T;

	public executorCallback: ReactiveTargetCallback<E, P>;

	public observers = new Set<ReactiveTargetCallback<E, P>>();


	public emit(): this {
		this._notify();
		return this;
	}


	public update = (eventMessage?: ReactiveEventMessage<E, P>): this => {
		this.scope.taskQ.push(() => {
			this.executorCallback(eventMessage);
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

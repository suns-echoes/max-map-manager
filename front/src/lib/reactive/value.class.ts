import type { ReactiveTargetCallback } from './types/reactive.types.ts';
import type { ReactiveSource } from './base/reactive-source.class.ts';
import type { ReactiveScope } from './base/reactive-scope.class.ts';
import { ReactiveEventTarget } from './base/reactive-event-target.ts';
import { appReactiveScope } from './base/reactive-scope.class.ts';


/**
 * Reactive Value Class.
 *
 * This class implements the ReactiveSource interfaces,
 * allowing it to act as a source of reactive data.
 */
export class Value<T, E = any, P = any> extends ReactiveEventTarget<E, P> implements ReactiveSource {
	constructor(value: T) {
		super();
		this.value = value;
	}

	public destroy(): void {
		this.observers.clear();
		if (this.scope) {
			this.scope.reactiveObjects.delete(this);
			this.scope = null!;
		}
	}


	public value: T;

	public scope: ReactiveScope = appReactiveScope;

	public observers = new Set<ReactiveTargetCallback<E, P>>();


	public set(newValue: T, force: boolean = false): this {
		if (force ||!Object.is(this.value, newValue)) {
			this.value = newValue;
			this.emit();
		}
		return this;
	}

	public apply(fn: (currentValue: T) => T, force: boolean = false): this {
		this.set(fn(this.value), force);
		return this;
	}

	public emit(): this {
		this._notify();
		return this;
	}


	private _notify = (): void => {
		for (const observer of this.observers) {
			observer();
		}
	}
}

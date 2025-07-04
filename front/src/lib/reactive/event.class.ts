import { ReactiveScope, appReactiveScope } from './base/reactive-scope.class.ts';


/**
 * Reactive Event Class.
 *
 * This class implements the ReactiveSource and ReactiveTarget interfaces,
 * allowing it to act as both a source and a target for reactions.
 */
export class Event<T = string, M = any> {
	constructor(type: T) {
		this.type = type;
	}

	destroy(): void {
		if (this.scope) {
			this.scope.reactiveObjects.delete(this);
			this.scope = null!;
		}
		this.type = null!;
	}

	public type: T;

	/**
	 * Emits a signal to all registered observers, informing them of a potential state change.
	 * Observers should independently retrieve necessary data directly from this source.
	 */
	public publish(payload: M): void {
		this.scope.dispatch({ type: this.type, payload });
	}

	public scope: ReactiveScope = appReactiveScope;
}

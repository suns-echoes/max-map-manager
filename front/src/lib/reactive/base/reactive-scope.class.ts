import type { ReactiveComputeCallback, ReactiveTargetCallback } from '../types/reactive.types.ts';
import { ReactiveEventTarget } from './reactive-event-target.ts';
import { Event } from '../event.class.ts';
import { Signal } from '../signal.class.ts';
import { TaskQueue } from './queue.class.ts';
import { Value } from '../value.class.ts';
import { Expr } from '../expr.class.ts';
import { Effect } from '../effect.class.ts';


export class ReactiveScope<C = null> extends ReactiveEventTarget {
	public constructor(context?: C) {
		super();
		this.context = context || null;
	}

	public destroy(): void {
		for (const obj of this.reactiveObjects) {
			obj.destroy();
		}
		if (this.reactiveObjects.size > 0) {
			throw new Error('Assertion Error: Failed to destroy ReactiveScope: reactive objects still exist.');
		}

		for (const childScope of this.childScopes) {
			childScope.destroy();
		}
		if (this.childScopes.size > 0) {
			throw new Error('Assertion Error: Failed to destroy ReactiveScope: child scopes still exist.');
		}

		if (typeof this.context?.destroy === 'function') {
			this.context.destroy();
		}
		this.context = null;

		if (this.parentScope) {
			this.parentScope.childScopes.delete(this);
			this.parentScope = null;
		}

		super.destroy();
	}


	public context: any;

	public parentScope: ReactiveScope | null = null;

	public childScopes: Set<ReactiveScope> = new Set();

	public reactiveObjects: Set<any> = new Set();


	public createChildScope<C = void>(context?: C): ReactiveScope {
		const childScope = new ReactiveScope(context);
		childScope.parentScope = this;
		this.childScopes.add(childScope);
		return childScope;
	}


	public createEvent<T = string, M = any>(type: T): Event<T, M> {
		const scopedEvent = new Event(type);
		this.reactiveObjects.add(scopedEvent);
		return scopedEvent;
	}

	public createSignal<E = any, P = any>(updateCallback?: ReactiveTargetCallback<E, P>): Signal<E, P> {
		const scopedSignal = new Signal(updateCallback);
		this.reactiveObjects.add(scopedSignal);
		return scopedSignal;
	}

	public createValue<T, E = any, P = any>(value: T): Value<T, E, P> {
		const scopedValue = new Value<T, E, P>(value);
		this.reactiveObjects.add(scopedValue);
		return scopedValue;
	}

	public createExpr<T, E = any, P = any>(recomputeCallback: ReactiveComputeCallback<T, E, P>, initialValue: T): Expr<T, E, P> {
		const scopedExpr = new Expr<T, E, P>(recomputeCallback, initialValue);
		this.reactiveObjects.add(scopedExpr);
		return scopedExpr;
	}

	public createEffect<E = any, P = any>(executorCallback: ReactiveTargetCallback<E, P>): Effect<E, P> {
		const scopedEffect = new Effect<E, P>(executorCallback);
		this.reactiveObjects.add(scopedEffect);
		return scopedEffect;
	}


	/**
	 * Queue for asynchronous tasks that will be executed in the next microtask.
	 * This queue is intended for tasks that may trigger further changes in the reactive system.
	 */
	public asyncQ = new TaskQueue();

	/**
	 * Final queue for tasks that need to be executed after all other queues.
	 * This queue is intended for effects and other tasks that will not trigger
	 * further changes in the reactive system.
	 */
	public finalQ = new TaskQueue();

	/**
	 * Queue for tasks that may trigger further changes in the reactive system.
	 * First execution will be scheduled for the microtask queue,
	 * to deduplicate subsequent tasks calls.
	 */
	public taskQ = new TaskQueue({
		autoProcess: true,
		subQueue: this.finalQ,
	});
}


/**
 * The application root reactive scope.
 */
export const appReactiveScope = new ReactiveScope();

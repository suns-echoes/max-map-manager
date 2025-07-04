import { ReactiveEventMessage, ReactiveTargetCallback } from '../types/reactive.types.ts';
import { ReactiveRealmObject } from './reactive-realm-object.ts';
import { ReactiveSource } from './reactive-source.class.ts';


/**
 * Base interface for reactive targets.
 * This interface allows objects to react to changes in reactive sources.
 * Reactive targets can subscribe to multiple reactive sources
 * and will be notified when any of them emit a signal.
 */
export abstract class ReactiveTarget extends ReactiveRealmObject {
	/**
	 * Callback that executes the reaction logic to a notification.
	 * This callback should retrieve necessary data directly from the associated ReactiveSource.
	 */
	public abstract executorCallback: ReactiveTargetCallback | null;

	/**
	 * Destroys the target, cleaning up any resources and observers.
	 * This method should be called when the target is no longer needed.
	 */
	public abstract destroy(): void;

	/**
	 * Callback that executes the reaction logic to a notification.
	 * This callback should retrieve necessary data directly from the associated ReactiveSource.
	 */
	public abstract update(eventMessage?: ReactiveEventMessage): this;

	/**
	 * Subscribes this target to the provided reactive sources.
	 * @param sources - An array of reactive sources to subscribe to.
	 */
	public abstract on(sources: ReactiveSource[]): this;

	/**
	 * Unsubscribes this target from the provided reactive sources.
	 * @param sources - An array of reactive sources to unsubscribe from.
	 */
	public abstract off(sources: ReactiveSource[]): this;
}

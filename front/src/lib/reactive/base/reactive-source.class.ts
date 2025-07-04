import { ReactiveTargetCallback } from '../types/reactive.types.ts';
import { ReactiveRealmObject } from './reactive-realm-object.ts';


/**
 * Base class enabling objects to notify registered observers of state changes.
 * Observers are responsible for retrieving updated data from the source.
 */
export abstract class ReactiveSource extends ReactiveRealmObject {
	/**
	 * List of observers for this source.
	 */
	public abstract observers: Set<ReactiveTargetCallback>;

	/**
	 * Emits a signal to all registered observers, informing them of a potential state change.
	 * Observers should independently retrieve necessary data directly from this source.
	 */
	public abstract emit(): this;
}

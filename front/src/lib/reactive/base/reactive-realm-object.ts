import { ReactiveScope } from './reactive-scope.class.ts';


export abstract class ReactiveRealmObject {
	public abstract destroy(): void;

	public abstract scope: ReactiveScope;
}

export type TaskCallback = () => void;

export type AsyncTaskCallback = () => Promise<void>;


export type ReactiveComputeCallback<T = any, E = any, P = any> = (currentValue: T, eventMessage?: ReactiveEventMessage<E, P>) => T;

export type ReactiveTargetCallback<E = any, P = any> = (eventMessage?: ReactiveEventMessage<E, P>) => void;


export type ReactiveEventMessage<T = any, M = any> = {
	type: T;
	payload: M;
};

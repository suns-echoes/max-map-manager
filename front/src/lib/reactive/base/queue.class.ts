import type { ReactiveTargetCallback } from '../types/reactive.types.ts';


/**
 * Notes:
 *
 * Adding a first task to the taskQ, the process microtask will be scheduled.
 * This will allow to collect the rest of the tasks, deduplicate them,
 * and execute them in a single microtask.
 *
 * When first task is added to the finalQ the same process is applied,
 * thus there is no need to schedule a microtask for the finalQ.
 */


interface TaskQueueOptions {
	autoProcess?: boolean;
	subQueue?: TaskQueue | null;
}


export class TaskQueue {
	constructor(options: TaskQueueOptions = {}) {
		this._autoProcess = options.autoProcess ?? false;
		this._subQueue = options.subQueue ?? null;
	}

	public push(task: ReactiveTargetCallback): void {
		if (!this._taskList.has(task)) {
			this._taskList.add(task);
		} else {
			this._taskList.delete(task);
			this._taskList.add(task);
		}
		if (this._autoProcess && !this._isProcessingScheduled) {
			this._scheduleProcessing();
		}
	}

	public sync(): Promise<void> {
		if (this._syncObject === null) {
			if (this._taskList.size === 0) {
				return Promise.resolve();
			} else {
				this._syncObject = Promise.withResolvers<void>();
			}
		}
		return this._syncObject.promise;
	}

	public process(onDone?: () => void): void {
		const tasks = [...this._taskList];
		this._taskList.clear();
		for (const task of tasks) {
			task();
		}
		if (this._taskList.size > 0) {
			this.process();
		} else {
			this._subQueue?.process(() => {
				if (this._syncObject) {
					this._syncObject.resolve();
				}
			});
			onDone?.();
		}
	}

	private _autoProcess = false;

	private _subQueue: TaskQueue | null = null;

	private _syncObject: PromiseWithResolvers<void> | null = null;

	private _taskList = new Set<ReactiveTargetCallback>();

	private _isProcessingScheduled = false;

	private _scheduleProcessing(): void {
		this._isProcessingScheduled = true;
		queueMicrotask(() => {
			this._isProcessingScheduled = false;
			this.process();
		});
	}
}

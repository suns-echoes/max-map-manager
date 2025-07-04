export class Interval {
	callback: (interval: Interval) => void;
	intervalId: ReturnType<typeof setInterval> | null = null;
	delay: number;

	constructor(callback: (interval: Interval) => void, delay: number = 100) {
		this.callback = callback;
		this.delay = delay;
	}

	start() {
		if (this.intervalId !== null) return;
		this.intervalId = setInterval(() => {
			this.callback(this);
		}, this.delay);
	}

	stop() {
		if (this.intervalId === null) return;
		clearInterval(this.intervalId);
		this.intervalId = null;
	}
}

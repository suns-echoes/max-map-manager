import { describe, it, mock } from 'node:test';
import assert from 'node:assert';

import { Signal } from './signal.class.ts';


describe('Signal Class', () => {
	function waitForMicrotask(): Promise<void> {
		return new Promise<void>((resolve) => queueMicrotask(resolve));
	}

	describe('Initialization', () => {
		it('should initialize with no observers', () => {
			// Arrange & Act
			const signal = new Signal();

			// Assert
			assert.strictEqual(signal.observers.size, 0);

			// Clean up
			signal.destroy();
		});
	});

	describe('Reactive Source Functionality', () => {
		it('should notify observers when emit is called', async () => {
			// Arrange
			const signal = new Signal();
			const observer1 = mock.fn();
			const observer2 = mock.fn();
			signal.observers.add(observer1);
			signal.observers.add(observer2);

			// Act
			signal.emit();

			// Assert
			assert.strictEqual(observer1.mock.callCount(), 1);
			assert.strictEqual(observer2.mock.callCount(), 1);

			// Clean up
			signal.destroy();
		});
	});

	describe('Observer Management', () => {
		it('should add observers correctly', () => {
			// Arrange
			const signal = new Signal();
			const observer = mock.fn();

			// Act
			signal.observers.add(observer);

			// Assert
			assert.strictEqual(signal.observers.size, 1);
			assert(signal.observers.has(observer));

			// Clean up
			signal.destroy();
		});

		it('should remove observers correctly', () => {
			// Arrange
			const signal = new Signal();
			const observer = mock.fn();
			signal.observers.add(observer);

			// Act
			signal.observers.delete(observer);

			// Assert
			assert.strictEqual(signal.observers.size, 0);
			assert(!signal.observers.has(observer));

			// Clean up
			signal.destroy();
		});
	});

	describe('Reactive Target Functionality', () => {
		it('should execute update callback when notified', async () => {
			// Arrange
			const updateCallback = mock.fn();
			const signal = new Signal(updateCallback);

			// Act
			signal.update();

			// Assert
			assert.strictEqual(updateCallback.mock.callCount(), 0);
			await waitForMicrotask();
			assert.strictEqual(updateCallback.mock.callCount(), 1);

			// Clean up
			signal.destroy();
		});
	});

	describe('Reactive Propagation', () => {
		it('should propagate updates to targets', async () => {
			// Arrange
			const updateCb = mock.fn();
			const sourceSignal = new Signal();
			const targetSignal = new Signal(updateCb);
			targetSignal.on([sourceSignal]);

			// Act
			sourceSignal.emit();

			// Assert
			assert.strictEqual(updateCb.mock.callCount(), 0);
			await waitForMicrotask();
			assert.strictEqual(updateCb.mock.callCount(), 1);

			// Clean up
			sourceSignal.destroy();
			targetSignal.destroy();
		});

		it('should propagate updates multiple levels deep in one microtask', async () => {
			// Arrange
			const sourceSignal = new Signal();
			const interUpdate = mock.fn();
			const interSignal = new Signal(interUpdate);
			const finalUpdate = mock.fn();
			const targetSignal = new Signal(finalUpdate);

			interSignal.on([sourceSignal]);
			targetSignal.on([interSignal]);

			// Act
			sourceSignal.emit();

			// Assert
			assert.strictEqual(interUpdate.mock.callCount(), 0);
			assert.strictEqual(finalUpdate.mock.callCount(), 0);
			await waitForMicrotask();
			assert.strictEqual(interUpdate.mock.callCount(), 1);
			assert.strictEqual(finalUpdate.mock.callCount(), 1);

			// Clean up
			sourceSignal.destroy();
			targetSignal.destroy();
		});
	});

	describe('Destroy Functionality', () => {
		it('should clear observers on destroy', () => {
			// Arrange
			const signal = new Signal();
			const observer = mock.fn();
			signal.observers.add(observer);

			// Act
			signal.destroy();

			// Assert
			assert.strictEqual(signal.observers.size, 0);
			assert.strictEqual(signal.scope, null);
		});
	});
});

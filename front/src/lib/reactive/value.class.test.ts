import { describe, it, mock } from 'node:test';
import assert from 'node:assert';

import { Value } from './value.class.ts';


describe('Value Class', () => {
	describe('Initialization', () => {
		it('should initialize with a value', () => {
			// Arrange
			const initialValue = 42;
			const val = new Value(initialValue);

			// Assert
			assert.strictEqual(val.value, initialValue);

			// Clean up
			val.destroy();
		});
	});

	describe('Reactive Source Functionality', () => {
		it('should not notify observers when value is set directly', () => {
			// Arrange
			const initialValue = 42;
			const val = new Value(initialValue);
			const observer = mock.fn();
			val.observers.add(observer);

			// Act
			val.value = 100;

			// Assert
			assert.strictEqual(val.value, 100);
			assert.strictEqual(observer.mock.callCount(), 0);

			// Clean up
			val.destroy();
		});

		it('should set a new value and notify observers', () => {
			// Arrange
			const initialValue = 42;
			const newValue = 100;
			const val = new Value(initialValue);
			const observer = mock.fn();

			val.observers.add(observer);

			// Act
			val.set(newValue);

			// Assert
			assert.strictEqual(val.value, newValue);
			assert.strictEqual(observer.mock.callCount(), 1);

			// Clean up
			val.destroy();
		});

		it('should apply a function to the current value and notify observers', () => {
			// Arrange
			const initialValue = 42;
			const val = new Value(initialValue);
			const observer = mock.fn();

			val.observers.add(observer);

			// Act
			val.apply((currentValue) => currentValue + 10);

			// Assert
			assert.strictEqual(val.value, 52);
			assert.strictEqual(observer.mock.callCount(), 1);

			// Clean up
			val.destroy();
		});

		it('should emit a signal to notify observers', () => {
			// Arrange
			const initialValue = 42;
			const val = new Value(initialValue);
			let notified = false;

			val.observers.add(() => {
				notified = true;
			});

			// Act
			val.emit();

			// Assert
			assert.strictEqual(notified, true);
		});
	});

	describe('Destroy Functionality', () => {
		it('should clear observers on destroy', () => {
			// Arrange
			const initialValue = 42;
			const val = new Value(initialValue);

			val.observers.add(() => {});

			// Act
			val.destroy();

			// Assert
			assert.strictEqual(val.observers.size, 0);
			assert.strictEqual(val.scope, null);
		});
	});
});

import { describe, it, mock } from 'node:test';
import assert from 'node:assert';

import { Expr } from './expr.class.ts';


describe('Expr Class', () => {
	function waitForMicrotask(): Promise<void> {
		return new Promise<void>((resolve) => queueMicrotask(resolve));
	}

	describe('Initialization', () => {
		it('should initialize with no observers', () => {
			// Arrange & Act
			const expr = new Expr((value) => value, 0);

			// Assert
			assert.strictEqual(expr.observers.size, 0);
			assert.strictEqual(expr.value, 0);

			// Clean up
			expr.destroy();
		});
	});

	describe('Reactive Source Functionality', () => {
		it('should not notify observers when value is set directly', () => {
			// Arrange
			const initialValue = 0;
			const expr = new Expr((value) => value, initialValue);
			const observer = mock.fn();
			expr.observers.add(observer);

			// Act
			expr.value = 100;

			// Assert
			assert.strictEqual(expr.value, 100);
			assert.strictEqual(observer.mock.callCount(), 0);

			// Clean up
			expr.destroy();
		});

		it('should recompute value and notify observers', async () => {
			// Arrange
			const initialValue = 2;
			const newValue = 40;
			const recomputeCallback = mock.fn((value) => value + newValue);
			const expr = new Expr(recomputeCallback, initialValue);
			const observer = mock.fn();

			expr.observers.add(observer);

			// Act
			expr.update();
			await waitForMicrotask();

			// Assert
			assert.strictEqual(expr.value, newValue);
			assert.strictEqual(recomputeCallback.mock.callCount(), 1);
			assert.strictEqual(recomputeCallback.mock.calls[0].arguments[0], initialValue);
			assert.strictEqual(recomputeCallback.mock.calls[0].result, newValue);
			assert.strictEqual(observer.mock.callCount(), 1);

			// Clean up
			expr.destroy();
		});

		it('should emit a signal to notify observers', () => {
			// Arrange
			const expr = new Expr((value) => value, 0);
			const observer1 = mock.fn();
			const observer2 = mock.fn();
			expr.observers.add(observer1);
			expr.observers.add(observer2);

			// Act
			expr.emit();

			// Assert
			assert.strictEqual(observer1.mock.callCount(), 1);
			assert.strictEqual(observer2.mock.callCount(), 1);

			// Clean up
			expr.destroy();
		});
	});

	describe('Observer Management', () => {
		it('should add observers correctly', () => {
			// Arrange
			const expr = new Expr((value) => value, 0);
			const observer = mock.fn();

			// Act
			expr.observers.add(observer);

			// Assert
			assert.strictEqual(expr.observers.size, 1);
			assert(expr.observers.has(observer));

			// Clean up
			expr.destroy();
		});

		it('should remove observers correctly', () => {
			// Arrange
			const expr = new Expr((value) => value, 0);
			const observer = mock.fn();
			expr.observers.add(observer);

			// Act
			expr.observers.delete(observer);

			// Assert
			assert.strictEqual(expr.observers.size, 0);
			assert(!expr.observers.has(observer));

			// Clean up
			expr.destroy();
		});
	});

	describe('Reactive Target Functionality', () => {
		it('should execute recompute callback when notified', async () => {
			// Arrange
			const initialValue = 0;
			const newValue = 42;
			const recomputeCallback = mock.fn(() => newValue);
			const expr = new Expr(recomputeCallback, initialValue);

			// Act
			expr.update();

			await waitForMicrotask();

			// Assert
			assert.strictEqual(expr.value, newValue);
			assert.strictEqual(recomputeCallback.mock.callCount(), 1);

			// Clean up
			expr.destroy();
		});
	});

	describe('Reactive Propagation', () => {
			it('should propagate updates to targets', async () => {
				// Arrange
				const expr = new Expr(() => 0, 0);
				const observer1 = mock.fn();
				const observer2 = mock.fn();

				expr.observers.add(observer1);
				expr.observers.add(observer2);

				// Act
				expr.update();
				await waitForMicrotask();

				// Assert
				assert.strictEqual(observer1.mock.callCount(), 1);
				assert.strictEqual(observer2.mock.callCount(), 1);

				// Clean up
				expr.destroy();
			});

			it('should propagate updates multiple levels deep in one microtask', async () => {
				// Arrange
				const sourceRecalc = mock.fn(() => 0);
				const sourceExpr = new Expr(sourceRecalc, 0);
				const interRecalc = mock.fn(() => 50);
				const interExpr = new Expr(interRecalc, 0);
				const finalRecalc = mock.fn(() => 100);
				const finalExpr = new Expr(finalRecalc, 0);

				interExpr.on([sourceExpr]);
				finalExpr.on([interExpr]);

				// Act
				sourceExpr.update();
				await waitForMicrotask();

				// Assert
				assert.strictEqual(sourceRecalc.mock.callCount(), 1);
				assert.strictEqual(interRecalc.mock.callCount(), 1);
				assert.strictEqual(finalRecalc.mock.callCount(), 1);
			});
	});

	describe('Destroy Functionality', () => {
		it('should clear observers on destroy', () => {
			// Arrange
			const expr = new Expr((value) => value, 0);
			const observer = mock.fn();
			expr.observers.add(observer);

			// Act
			expr.destroy();

			// Assert
			assert.strictEqual(expr.observers.size, 0);
			assert.strictEqual(expr.scope, null);
		});
	});
});

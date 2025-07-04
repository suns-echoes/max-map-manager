import { describe, it, mock } from 'node:test';
import assert from 'node:assert';

import type { ReactiveTargetCallback } from './types/reactive.types.ts';
import { Event } from './event.class.ts';


describe('Event Class', () => {
	it('should initialize with a type', () => {
		// Arrange
		const eventType = 'testEvent';
		const event = new Event(eventType);

		// Assert
		assert.strictEqual(event.type, eventType);
	});

	it('should publish messages to the realm', () => {
		// Arrange
		const eventType = 'testEvent';
		const payload = { data: 'testData' };
		const event = new Event(eventType);
		const eventTarget1 = mock.fn();
		const eventTarget2 = mock.fn();

		event.scope.addEventHandler(eventType, eventTarget1 as ReactiveTargetCallback);
		event.scope.addEventHandler(eventType, eventTarget2 as ReactiveTargetCallback);

		// Act
		event.publish(payload);

		// Assert
		assert.strictEqual(eventTarget1.mock.callCount(), 1);
		assert.deepStrictEqual(eventTarget1.mock.calls[0].arguments[0], { type: eventType, payload });
		assert.strictEqual(eventTarget2.mock.callCount(), 1);
		assert.deepStrictEqual(eventTarget2.mock.calls[0].arguments[0], { type: eventType, payload });

		// Clean up
		event.scope.removeAllEventHandlers();
		event.destroy();
	});

	it('should publish one instance of the payload to all targets', () => {
		// Arrange
		const eventType = 'testEvent';
		const message = { data: 'testData' };
		const event = new Event(eventType);
		const eventTarget1 = mock.fn();
		const eventTarget2 = mock.fn();

		event.scope.addEventHandler(eventType, eventTarget1 as ReactiveTargetCallback);
		event.scope.addEventHandler(eventType, eventTarget2 as ReactiveTargetCallback);

		// Act
		event.publish(message);

		// Assert
		const payload1 = eventTarget1.mock.calls[0].arguments[0];
		const payload2 = eventTarget2.mock.calls[0].arguments[0];
		assert.strictEqual(payload1, payload2);

		// Clean up
		event.destroy();
	});

	describe('Destroy Functionality', () => {
		console.log('TODO: Destroy Functionality');
	});
});

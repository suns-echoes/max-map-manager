use log::{LevelFilter, Log, Metadata, Record};
use std::cell::RefCell;
use std::sync::Once;

// This module provides a custom logger for tests that captures log messages
// in a thread-local storage, allowing tests to inspect logs without interference
// from other tests. It also provides utility functions to initialize the logger,
// clear captured logs, and retrieve them for assertions.
//
// Usage:
//
// Wrap your test code in the `run_test!` macro to ensure logs are captured and cleared
// for each test. This macro also catches panics to allow inspection of logs even if
// the test fails.
//
// Example:
// ```
// #[test]
// fn my_test() {
//     run_test!({
//         // Your test code here
//         log::info!("This is a test log message");
//         assert!(true);
//         // Logs can be inspected using `get_captured_logs()`
//         let logs = get_captured_logs();
//         assert!(logs.contains("[INFO] This is a test log message"));
//         clear_captured_logs(); // Clear logs after inspection if needed
//     });
// }
// ```

// --- Custom Logger Implementation for Tests ---

thread_local! {
	// This stores the log messages specific to the current test thread.
	static CAPTURED_LOGS: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

struct TestLogger;

impl Log for TestLogger {
	fn enabled(&self, metadata: &Metadata) -> bool {
		// Capture all levels for testing flexibility
		metadata.level() <= LevelFilter::Trace
	}

	fn log(&self, record: &Record) {
		if self.enabled(record.metadata()) {
			CAPTURED_LOGS.with(|cell| {
				cell.borrow_mut().push(format!("[{}] {}", record.level(), record.args()));
			});
		}
	}

	fn flush(&self) {}
}

static TEST_LOGGER: TestLogger = TestLogger;
static INIT_LOGGER_ONCE: Once = Once::new();

pub fn init_test_logger() {
	INIT_LOGGER_ONCE.call_once(|| {
		// Set the logger globally. This will capture logs from any thread that calls
		// log! macros, but our thread_local storage will ensure logs are isolated
		// per test when clear_captured_logs is used.
		log::set_logger(&TEST_LOGGER).expect("Failed to set logger");
		log::set_max_level(LevelFilter::Trace); // Capture all log levels
	});
}

/// Clears the captured logs from the thread-local storage for the current test.
pub fn clear_captured_logs() {
	CAPTURED_LOGS.with(|cell| {
		cell.borrow_mut().clear();
	});
}

/// Retrieves a clone of the captured logs for the current test.
pub fn get_captured_logs() -> Vec<String> {
	CAPTURED_LOGS.with(|cell| {
		cell.borrow().clone()
	})
}

// --- Test Helper Macro/Function ---

/// A helper to run a test with proper log capture and clearing.
/// It also catches panics to allow logging inspection even on test failures.
#[macro_export]
macro_rules! run_test {
	($test_block:expr) => {{
		use std::panic;

		use crate::test_utils::init_test_logger;
		use crate::test_utils::clear_captured_logs;
		use crate::test_utils::get_captured_logs;

		init_test_logger(); // Ensure logger is set
		clear_captured_logs(); // Crucial: Clear logs for this specific test

		// Catch unwind (panic) to ensure logs can be inspected even if the test fails.
		let result = panic::catch_unwind(|| {
			$test_block
		});

		// If the test panicked, print the captured logs for debugging.
		if result.is_err() {
			eprintln!("\x1b[31m\n--- Captured Logs (on test panic) ---\x1b[0m");
			for log_line in get_captured_logs() {
				eprintln!("{}", log_line);
			}
			eprintln!("---------------------------------------\n");
		}

		// Re-throw the panic to let the test harness report the failure.
		result.unwrap();
	}};
}

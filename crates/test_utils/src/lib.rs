use colored::Colorize;
extern crate log;

pub mod fs_helpers;
pub mod logger;

pub use fs_helpers::*;
pub use logger::*;

static mut TEST_FS: Option<fs_helpers::TestFileSystem> = None;

#[ctor::ctor]
fn init() {
    init_test_logger();

	unsafe {
		TEST_FS = Some(TestFileSystem::new());
		if let Some(ref test_fs) = TEST_FS {
			let message: String = format!("Test file system initialized at: {}", test_fs.root.display()).green().to_string();
			println!("{}", message);
		}
	}
}

#[dtor::dtor]
fn cleanup() {
	if !is_any_test_failed() {
		unsafe {
			if let Some(ref mut test_fs) = TEST_FS {
				test_fs.cleanup();
				let message: String = format!("Test file system cleaned up at: {}", test_fs.root.display()).green().to_string();
				println!("{}", message);
				test_fs.cleanup();
			}
		}
	} else {
		let message: String = "Some tests failed, skipping cleanup.".red().to_string();
		println!("{}", message);
	}
}

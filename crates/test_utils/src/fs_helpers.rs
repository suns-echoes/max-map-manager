use std::path::PathBuf;
use uuid::Uuid;

const TEST_DIR_ROOT: &str = "test_files/temp";

pub struct TestFileSystem {
	pub root: PathBuf,
}

impl TestFileSystem {
	pub fn new() -> Self {
		let dir_name = Uuid::new_v4().to_string();
		let root = PathBuf::from(TEST_DIR_ROOT).join(dir_name);
		if !root.exists() {
			std::fs::create_dir_all(&root).expect("Failed to create test directory root");
		}
		TestFileSystem { root }
	}

	pub fn cleanup(&self) {
		if self.root.exists() {
			std::fs::remove_dir_all(TEST_DIR_ROOT).expect("Failed to clean up test directory");
		}
	}

	pub fn get_test_dir_path(&self, sub_dir: &str) -> PathBuf {
		self.root.join(sub_dir)
	}

	pub fn get_test_file_path(&self, file_name: &str) -> PathBuf {
		self.root.join(file_name)
	}

	pub fn create_test_dir(&self, sub_dir: &str) -> PathBuf {
		let dir_path = self.get_test_dir_path(sub_dir);
		std::fs::create_dir_all(&dir_path).expect("Failed to create test directory");
		dir_path
	}

	pub fn create_test_file(&self, file_name: &str, content: &str) -> PathBuf {
		let file_path = self.get_test_file_path(file_name);
		let dir_path = file_path.parent().unwrap();
		if !dir_path.exists() {
			std::fs::create_dir_all(dir_path).expect("Failed to create parent directories for test file");
		}
		std::fs::write(&file_path, content).expect("Failed to create test file");
		file_path
	}

	pub fn file_exist(&self, file_path: &PathBuf) -> bool {
		file_path.exists() && file_path.is_file()
	}

	pub fn is_path_directory(&self, path: &PathBuf) -> bool {
		path.is_dir()
	}

	pub fn is_path_file(&self, path: &PathBuf) -> bool {
		path.is_file()
	}

	pub fn file_has_content(&self, file_path: &PathBuf, content: &str) -> bool {
		if let Ok(file_content) = std::fs::read_to_string(file_path) {
			file_content == content
		} else {
			false
		}
	}
}

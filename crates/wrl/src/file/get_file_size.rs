use std::fs::metadata;
use std::path::Path;

/// Gets the size of the file at the specified path.
pub fn get_file_size(file_path: &Path) -> Result<u64, ()> {
	match metadata(file_path) {
		Ok(meta) => {
			if meta.is_file() {
				Ok(meta.len())
			} else {
				log::error!("Expected a file: {}", file_path.display());
				Err(())
			}
		}
		Err(_) => {
			log::error!("Failed to get file metadata: {}", file_path.display());
			return Err(());
		},
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	use crate::run_test;

	#[test]
	fn test_get_file_size_existing_file() {
		run_test!({
			// Arrange
			let path = PathBuf::from("test_files/reference/REF.WRL");

			// Act
			let len = get_file_size(&path).unwrap();

			// Assert
			assert_eq!(len, 263755);
		});
	}

	#[test]
	fn test_get_file_size_nonexistent_file() {
		run_test!({
			// Arrange
			let path = PathBuf::from("test_files/reference/NON_EXISTENT_FILE.WRL");
			let expected_log = "[ERROR] Failed to get file metadata: test_files/reference/NON_EXISTENT_FILE.WRL";

			// Act
			let result = get_file_size(&path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], expected_log);
		});
	}

	#[test]
	fn test_get_file_size_empty_file() {
		run_test!({
			// Arrange
			let path = PathBuf::from("test_files/reference/empty_file");

			// Act
			let len = get_file_size(&path).unwrap();

			// Assert
			assert_eq!(len, 0);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_get_file_size_directory() {
		run_test!({
			// Arrange
			let path = PathBuf::from("test_files");
			let expected_log = "[ERROR] Expected a file: test_files";

			// Act
			let result = get_file_size(&path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], expected_log);
		});
	}
}

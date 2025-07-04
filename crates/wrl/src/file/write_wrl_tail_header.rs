use std::path::Path;
use std::io::Write;

use crate::file::copy_wrl_file_without_tail;
use crate::structs::WRLTailHeader;

// Writes the WRL tail header to a file.
// This function performs the atomic operation of writing the tail header by:
// 1. Copying the original file without the tail header to a temporary file.
// 2. Appending the new tail header to the temporary file.
// 3. Renaming the original file to a backup.
// 4. Replacing the original file with the temporary file.
// 5. Removing the temporary file.
// If any step fails, it attempts to revert the changes and logs the errors.
pub fn write_wrl_tail_header(
	file_path: &Path,
	tail_header: &WRLTailHeader,
) -> Result<(), ()> {
	// Get current time as temp file suffix.
	let current_time = chrono::Local::now();
	let temp_file_suffix = format!("{}", current_time.format("%Y%m%d%H%M%S"));
	let temp_file_path = file_path.with_extension(format!("WRL.~temp~.{}", temp_file_suffix));
	let original_backup_path = file_path.with_extension(format!("WRL.~original~.{}", temp_file_suffix));

	// Copy the original file without the tail header to temp file.
	copy_wrl_file_without_tail(file_path, &temp_file_path).map_err(|_| {
		log::error!("Failed to copy WRL file without tail: {}. Aborting.", file_path.display());
		()
	})?;

	// Write the new tail header to the temp file.
	// If this fails, temp file will be removed.
	let json = match serde_json::to_string(tail_header) {
		Ok(j) => j,
		Err(e) => {
			log::error!("Failed to serialize tail header to JSON. Reverting changes.");
			log::error!("{}", e);
			if let Err(e2) = std::fs::remove_file(&temp_file_path) {
				log::error!("Revert failed: could not remove temp file: {}. Aborting.", temp_file_path.display());
				log::error!("{}", e2);
				log::info!("Temp file: {} was not removed. Please remove it manually if not needed.", temp_file_path.display());
			}
			return Err(());
		}
	};

	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.append(true)
		.open(&temp_file_path)
		.map_err(|_| {
			log::error!("Failed to open temp file for writing: {}. Aborting.", temp_file_path.display());
			()
		})?;

	if let Err(e) = write!(&mut file, "{}", json) {
		log::error!("Failed to write WRL tail header: {}. Reverting changes.", temp_file_path.display());
		log::error!("{}", e);
		if let Err(e) = std::fs::remove_file(&temp_file_path) {
			log::error!("Revert failed: could not remove temp file: {}. Aborting.", temp_file_path.display());
			log::error!("{}", e);
			log::info!("Temp file: {} was not removed. Please remove it manually if not needed.", temp_file_path.display());
		}
		()
	};

	// Rename the original file to a backup and then replace it with the temp file.
	// If this fails, temp file will be removed and original file name stays unchanged.
	if let Err(e) = std::fs::rename(&file_path, &original_backup_path) {
		log::error!("Failed to rename original file: {}. Reverting changes.", file_path.display());
		log::error!("{}", e);
		if let Err(e) = std::fs::remove_file(&temp_file_path) {
			log::error!("Revert failed: could not remove temp file: {}. Aborting.", temp_file_path.display());
			log::error!("{}", e);
			log::info!("Temp file: {} was not removed. Please remove it manually if not needed.", temp_file_path.display());
		}
		return Err(());
	}

	// Replace the original file with the temp file.
	// If this fails, original file is restored and temp file is removed.
	if let Err(e) = std::fs::rename(&temp_file_path, &file_path) {
		log::error!("Failed to replace original file with temp file: {}. Reverting changes.", temp_file_path.display());
		log::error!("{}", e);
		if let Err(e2) = std::fs::rename(&original_backup_path, &file_path) {
			log::error!("Revert failed: could not restore original file: {}. Aborting.", original_backup_path.display());
			log::error!("{}", e2);
			log::info!("Your original file is now named: {}, please remove the \"~original~\" extension manually.", file_path.display());
		}
		if let Err(e3) = std::fs::remove_file(&temp_file_path) {
			log::error!("Revert failed: could not remove temp file: {}. Aborting.", temp_file_path.display());
			log::error!("{}", e3);
			log::info!("Temp file: {} was not removed. Please remove it manually if not needed.", temp_file_path.display());
		}
		return Err(());
	}

	// Clean up: remove the backup of the original file.
	// If this fails, the backup remains.
	if let Err(e) = std::fs::remove_file(&original_backup_path) {
		log::error!("Failed to remove original backup file: {}. It will remain.", original_backup_path.display());
		log::error!("{}", e);
		log::info!("Backup file: {} was not removed. Please remove it manually if not needed.", original_backup_path.display());
	}

	Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	use crate::run_test;

	#[test]
	fn test_write_wrl_tail_header() {
		run_test!({
			// Arrange
			std::fs::create_dir_all("test_files/temp").expect("Failed to create temp directory");
			let source_file_path = PathBuf::from("test_files/reference/REF.WRL");
			let ref_file_path = PathBuf::from("test_files/reference/REF.WRL.TAIL");
			let file_path = PathBuf::from("test_files/temp/REF.WRL.TAIL.TEST");
			if let Err(e) = std::fs::copy(&source_file_path, &file_path) {
				log::error!("TEST INTERRUPTED! Failed to copy {} to {}: {}", source_file_path.display(), file_path.display(), e);
				return;
			}
			let tail_header = WRLTailHeader {
				_v: 1,
				hash_id: "1234567890abcdef".to_string(),
				name: "custom map name".to_string(),
				version: "12.0".to_string(),
				date: "2456-12-02 14:23:12".to_string(),
				author: "who made this world?".to_string(),
				description: "custom map description".to_string(),
				comment: "user comment".to_string(),
			};

			// Act
			let result = write_wrl_tail_header(&file_path, &tail_header);

			// Assert
			assert!(result.is_ok());

			// Verify the file match the reference
			let ref_content = std::fs::read(&ref_file_path).expect("TEST INTERRUPTED! Failed to read reference file");
			let test_content = std::fs::read(&file_path).expect("TEST INTERRUPTED! Failed to read test output file");
			assert_eq!(ref_content, test_content, "File content does not match reference");

			// Clean up
			if let Err(e) = std::fs::remove_file(&file_path) {
				log::error!("Failed to remove test file after test: {}", file_path.display());
				log::error!("{}", e);
				log::info!("Please remove the test file manually: {}", file_path.display());
			}

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_write_wrl_tail_header_with_error() {
		run_test!({
			// Arrange
			let file_path = PathBuf::from("test_files/temp/REF.WRL.TAIL.NON_EXISTING");
			let tail_header = WRLTailHeader {
				_v: 1,
				hash_id: "1234567890abcdef".to_string(),
				name: "custom map name".to_string(),
				version: "12.0".to_string(),
				date: "2456-12-02 14:23:12".to_string(),
				author: "who made this world?".to_string(),
				description: "custom map description".to_string(),
				comment: "user comment".to_string(),
			};

			// Act
			let result = write_wrl_tail_header(&file_path, &tail_header);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 2);
			assert_eq!(logs[0], "[ERROR] Source file does not exist: test_files/temp/REF.WRL.TAIL.NON_EXISTING");
			assert_eq!(logs[1], "[ERROR] Failed to copy WRL file without tail: test_files/temp/REF.WRL.TAIL.NON_EXISTING. Aborting.");
		});
	}
}

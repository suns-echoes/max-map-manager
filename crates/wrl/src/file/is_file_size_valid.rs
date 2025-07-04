use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::file::calc_expected_file_size::calc_expected_file_size;

/// Checks if file size is at least valid WRL data size.
pub fn is_file_size_valid(file_path: &Path) -> Result<bool, ()> {
	let mut file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => {
			log::error!("Failed to open file: {}", file_path.display());
			return Err(());
		}
	};

	match file.seek(SeekFrom::Start(5)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip header in file: {}", e);
			return Err(());
		}
	}

	let width = match file.read_u16::<LittleEndian>() {
		Ok(w) => w,
		Err(e) => {
			log::error!("Failed to read width from file: {}", e);
			return Err(());
		}
	};

	let height = match file.read_u16::<LittleEndian>() {
		Ok(h) => h,
		Err(e) => {
			log::error!("Failed to read height from file: {}", e);
			return Err(());
		}
	};

	match file.seek(SeekFrom::Current((width * height) as i64)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip minimap in file: {}", e);
			return Err(());
		}
	};

	match file.seek(SeekFrom::Current((width * height * 2) as i64)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip bigmap in file: {}", e);
			return Err(());
		}
	};

	let tile_count = match file.read_u16::<LittleEndian>() {
		Ok(tc) => tc,
		Err(e) => {
			log::error!("Failed to read tile count from file: {}", e);
			return Err(());
		}
	};

	let expected_length = calc_expected_file_size(width, height, tile_count);

	match file.metadata() {
		Ok(meta) => {
			if meta.len() >= expected_length as u64 {
				Ok(true)
			} else {
				log::error!("File length does not match expected min length: {} < {}", meta.len(), expected_length);
				Ok(false)
			}
		}
		Err(e) => {
			log::error!("Failed to get file metadata: {}", e);
			Err(())
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_is_file_size_valid() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/REF.WRL");

			// Act
			let result = is_file_size_valid(test_file_path);

			// Assert
			assert!(result.is_ok());
			assert_eq!(result.unwrap(), true);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_is_file_size_nonexistent_file() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/NON_EXISTENT_FILE.WRL");

			// Act
			let result = is_file_size_valid(test_file_path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], "[ERROR] Failed to open file: test_files/reference/NON_EXISTENT_FILE.WRL");
		});
	}

	#[test]
	fn test_is_file_size_not_valid() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/REF.WRL.TRIMMED");

			// Act
			let result = is_file_size_valid(test_file_path);

			// Assert
			assert!(result.is_ok());
			assert_eq!(result.unwrap(), false);

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], "[ERROR] File length does not match expected min length: 1050378 < 1050379");
		});
	}
}

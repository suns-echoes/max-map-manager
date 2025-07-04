use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::file::get_file_size::get_file_size;
use crate::file::calc_expected_file_size::calc_expected_file_size;
use crate::file::read_wrl_header::read_wrl_header;
use crate::hash_wrl_file_without_tail::hash_wrl_file_without_tail;
use crate::structs::WRLTailHeader;


pub fn read_wrl_tail_header(
	file_path: &Path,
) -> Result<Option<WRLTailHeader>, String> {
	let wrl_header = match read_wrl_header(file_path) {
		Ok(header) => header,
		Err(_) => {
			log::error!("Failed to read WRL header from file: {}", file_path.display());
			return Err("Failed to read WRL tail-header".into());
		}
	};

	let wrl_data_length = calc_expected_file_size(wrl_header.width, wrl_header.height, wrl_header.tile_count);

	let file_size = match get_file_size(file_path) {
		Ok(size) => size,
		Err(_) => {
			log::error!("Failed to get file size for: {}", file_path.display());
			return Err("Failed to read WRL tail-header".into());
		}
	};

	let wrl_tail_header_length: i64 = file_size as i64 - wrl_data_length as i64;

	if wrl_tail_header_length == 0 {
		let hash_id = hash_wrl_file_without_tail(file_path)
			.map_err(|_| "Failed to hash WRL file without tail".to_string())?;
		return Ok(Some(WRLTailHeader {
			_v: 1,
			hash_id,
			name: "".into(),
			version: "".into(),
			date: "".into(),
			author: "".into(),
			description: "".into(),
			comment: "".into(),
		}));
	}

	if wrl_tail_header_length < 0 {
		log::error!("Invalid WRL for file: {}", file_path.display());
		return Err("Invalid WRL file".into());
	}

	let mut file = match std::fs::File::open(file_path) {
		Ok(file) => file,
		Err(_) => {
			log::error!("Failed to open file: {}", file_path.display());
			return Err("Failed to read WRL tail-header".into());
		}
	};

	if let Err(_) = file.seek(SeekFrom::Start(wrl_data_length as u64)) {
		log::error!("Failed to seek to tail header position in file: {}", file_path.display());
		return Err("Failed to read WRL tail-header".into());
	}

	println!(
		"file_size: {}, expected_data_size: {}, tail_header_size: {}",
		file_size, wrl_data_length, wrl_tail_header_length
	);

	let mut buffer = String::new();
	if let Err(_) = file.read_to_string(&mut buffer) {
		log::error!("Failed to read tail header from file: {}", file_path.display());
		return Err("Failed to read WRL tail-header".into());
	}

	let tail_header = match serde_json::from_str(&buffer) {
		Ok(header) => header,
		Err(e) => {
			log::error!("Failed to deserialize WRL tail header: {}", file_path.display());
			log::error!("{}", e);
			return Err("Failed to read WRL tail-header".into());
		}
	};

	Ok(Some(tail_header))
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_read_wrl_tail_header() {
		run_test!({
		// Arrange
			let file_path = Path::new("test_files/reference/REF.WRL.TAIL");

			// Act
			let tail_header = read_wrl_tail_header(file_path).unwrap().unwrap();

			// Assert
			assert_eq!(tail_header._v, 1);
			assert_eq!(tail_header.hash_id, "1234567890abcdef");
			assert_eq!(tail_header.name, "custom map name");
			assert_eq!(tail_header.version, "12.0");
			assert_eq!(tail_header.date, "2456-12-02 14:23:12");
			assert_eq!(tail_header.author, "who made this world?");
			assert_eq!(tail_header.description, "custom map description");
			assert_eq!(tail_header.comment, "user comment");

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_read_wrl_tail_header_no_tail() {
		run_test!({
			// Arrange
			let file_path = Path::new("test_files/reference/REF.WRL");

			// Act
			let tail_header = read_wrl_tail_header(file_path).unwrap().unwrap();

			// Assert
			assert_eq!(tail_header._v, 1);
			assert_eq!(tail_header.hash_id, "93bcb82dc19b0d7a9a6eb9faf54869c9d3e353e0fdf4a847096ceff8720614aa");
			assert_eq!(tail_header.name, "");
			assert_eq!(tail_header.version, "");
			assert_eq!(tail_header.date, "");
			assert_eq!(tail_header.author, "");
			assert_eq!(tail_header.description, "");
			assert_eq!(tail_header.comment, "");

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_read_wrl_tail_header_invalid_tail() {
		run_test!({
			// Arrange
			let file_path = Path::new("test_files/reference/REF.WRL.TAIL.BROKEN");

			// Act
			let tail_header = read_wrl_tail_header(file_path);

			// Assert
			assert!(tail_header.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 2);
			assert_eq!(logs[0], "[ERROR] Failed to deserialize WRL tail header: test_files/reference/REF.WRL.TAIL.BROKEN");
			assert_eq!(logs[1], "[ERROR] EOF while parsing a string at line 7 column 12");
		});
	}

	#[test]
	fn test_read_wrl_tail_header_invalid_file() {
		run_test!({
			// Arrange
			let file_path = Path::new("test_files/reference/REF.WRL.TRIMMED");

			// Act
			let tail_header = read_wrl_tail_header(file_path);

			// Assert
			assert!(tail_header.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], "[ERROR] Invalid WRL for file: test_files/reference/REF.WRL.TRIMMED");
		});
	}
}

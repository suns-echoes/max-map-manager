use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use crate::consts::TILE_DATA_SIZE;
use crate::structs::{WRLHeader};

/// Reads the WRL header from the file and returns a WRLHeader struct.
pub fn read_wrl_header(file_path: &Path) -> Result<WRLHeader, String> {
	let mut file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => {
			log::error!("Failed to open file: {}", file_path.display());
			return Err("Failed to read WRL header".into());
		}
	};

	match file.seek(SeekFrom::Start(5)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip header in file: {}", e);
			return Err("Failed to read WRL header".into());
		}
	}

	let width = match file.read_u16::<LittleEndian>() {
		Ok(w) => w,
		Err(_) => {
			log::error!("Failed to read width from file: {}", file_path.display());
			return Err("Failed to read WRL header".into());
		}
	};

	let height = match file.read_u16::<LittleEndian>() {
		Ok(h) => h,
		Err(_) => {
			log::error!("Failed to read height from file: {}", file_path.display());
			return Err("Failed to read WRL header".into());
		}
	};

	let minimap_size = (width as usize) * (height as usize);
    let mut minimap = vec![0; minimap_size];
    match file.read_exact(&mut minimap) {
		Ok(_) => {},
		Err(_) => {
			log::error!("Failed to read minimap from file: {}", file_path.display());
			return Err("Failed to read WRL header".into());
		}
	};

	match file.seek(SeekFrom::Current((width as i64) * (height as i64) * 2)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip bigmap in file: {}", e);
			return Err("Failed to read WRL header".into());
		}
	};

	let tile_count = match file.read_u16::<LittleEndian>() {
		Ok(tc) => tc,
		Err(_) => {
			log::error!("Failed to read tile count from file: {}", file_path.display());
			return Err("Failed to read WRL header".into());
		}
	};

	match file.seek(SeekFrom::Current((tile_count as usize * TILE_DATA_SIZE) as i64)) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to skip tiles in file: {}", e);
			return Err("Failed to read WRL header".into());
		}
	};

	let palette_size = 256 * 3;
	let mut palette = vec![0; palette_size];
	match file.read_exact(&mut palette) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read palette from file: {}", e);
			return Err("Failed to read WRL header".into());
		}
	};

	Ok(WRLHeader {
		width,
		height,
		tile_count,
		minimap,
		palette,
	})
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	use crate::run_test;

	#[test]
	fn test_read_wrl_header() {
		run_test!({
			// Arrange
			let test_file_path = PathBuf::from("test_files/reference/REF.WRL");

			// Act
			let result = read_wrl_header(&test_file_path);

			// Assert
			assert!(result.is_ok());
			let header = result.unwrap();
			assert_eq!(header.width, 16);
			assert_eq!(header.height, 16);
			assert_eq!(header.tile_count, 64);
			assert_eq!(header.minimap.len(), 16 * 16);
			assert_eq!(header.palette.len(), 256 * 3);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_read_wrl_header_invalid_file() {
		run_test!({
			// Arrange
			let test_file_path = PathBuf::from("test_files/reference/empty_file");

			// Act
			let result = read_wrl_header(&test_file_path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert!(logs[0].contains("Failed to read width from file: test_files/reference/empty_file"));
		});
	}

	#[test]
	fn test_read_wrl_header_nonexistent_file() {
		run_test!({
			// Arrange
			let test_file_path = PathBuf::from("test_files/reference/NON_EXISTENT_FILE.WRL");

			// Act
			let result = read_wrl_header(&test_file_path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], "[ERROR] Failed to open file: test_files/reference/NON_EXISTENT_FILE.WRL");
		});
	}
}

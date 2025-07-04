use byteorder::{LittleEndian, ReadBytesExt};
use std::fs::{File};
use std::io::{Read};
use std::path::Path;

use crate::consts::TILE_DATA_SIZE;
use crate::structs::{WRLFile};


/// Reads the WRL file and returns a WRLFile struct.
pub fn read_wrl_file(file_path: &Path) -> Result<WRLFile, ()> {
	let mut file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => {
			log::error!("Failed to open file: {}", file_path.display());
			return Err(());
		}
	};

	let header_size = 5;
	let mut header = vec![0; header_size];
	match file.read_exact(&mut header) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read WRL header: {}", e);
			return Err(());
		}
	}

	let width = match file.read_u16::<LittleEndian>() {
		Ok(w) => w,
		Err(_) => {
			log::error!("Failed to read width from file: {}", file_path.display());
			return Err(());
		}
	};

	let height = match file.read_u16::<LittleEndian>() {
		Ok(h) => h,
		Err(_) => {
			log::error!("Failed to read height from file: {}", file_path.display());
			return Err(());
		}
	};

	let minimap_size = (width as usize) * (height as usize);
	let mut minimap = vec![0; minimap_size];
	match file.read_exact(&mut minimap) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read minimap from file: {}", e);
			return Err(());
		}
	};

	let bigmap_size = (width as usize) * (height as usize);
	let mut bigmap: Vec<u16> = vec![0; bigmap_size];
	match file.read_u16_into::<LittleEndian>(&mut bigmap) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read bigmap from file: {}", e);
			return Err(());
		}
	};

	let tile_count = match file.read_u16::<LittleEndian>() {
		Ok(tc) => tc,
		Err(_) => {
			log::error!("Failed to read tile count from file: {}", file_path.display());
			return Err(());
		}
	};

	let tiles_size = tile_count as usize * TILE_DATA_SIZE;
	let mut tiles: Vec<u8> = vec![0; tiles_size];
	match file.read_exact(&mut tiles) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read tile data from file: {}", e);
			return Err(());
		}
	};

	let palette_size = 256 * 3;
	let mut palette = vec![0; palette_size];
	match file.read_exact(&mut palette) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read palette from file: {}", e);
			return Err(());
		}
	}

	let pass_table_size = tile_count as usize;
	let mut pass_table = vec![0; pass_table_size];
	match file.read_exact(&mut pass_table) {
		Ok(_) => {},
		Err(e) => {
			log::error!("Failed to read pass table from file: {}", e);
			return Err(());
		}
	}

	Ok(WRLFile {
		header,
		width,
		height,
		minimap,
		bigmap,
		tile_count,
		tiles,
		palette,
		pass_table,
	})
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	use crate::run_test;

	#[test]
	fn test_read_wrl_file() {
		run_test!({
			// Arrange
			let path = PathBuf::from("test_files/reference/REF.WRL");

			// Act
			let wrl_file = read_wrl_file(&path).unwrap();

			// Assert
			assert_eq!(wrl_file.width, 16);
			assert_eq!(wrl_file.height, 16);
			assert_eq!(wrl_file.tile_count, 64);
			assert_eq!(wrl_file.minimap.len(), 256);
			assert_eq!(wrl_file.bigmap.len(), 256);
			assert_eq!(wrl_file.tiles.len(), 64 * TILE_DATA_SIZE);
			assert_eq!(wrl_file.palette.len(), 256 * 3);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_read_wrl_file_invalid_file() {
		run_test!({
			// Arrange
			let test_file_path = PathBuf::from("test_files/reference/invalid_file.wrl");

			// Act
			let result = read_wrl_file(&test_file_path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert!(logs[0].contains("Failed to open file: test_files/reference/invalid_file.wrl"));
		});
	}

	#[test]
	fn test_read_wrl_file_nonexistent_file() {
		run_test!({
			// Arrange
			let test_file_path = PathBuf::from("test_files/reference/NON_EXISTENT_FILE.WRL");

			// Act
			let result = read_wrl_file(&test_file_path);

			// Assert
			assert!(result.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert!(logs[0].contains("Failed to open file: test_files/reference/NON_EXISTENT_FILE.WRL"));
		});
	}
}

use blake3::{Hasher};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

use crate::calc_expected_file_size::calc_expected_file_size;
use crate::get_file_size::get_file_size;
use crate::read_wrl_header::read_wrl_header;


pub fn hash_wrl_file_without_tail(file_path: &Path) -> Result<String, ()> {
	let wrl_header = match read_wrl_header(file_path) {
		Ok(header) => header,
		Err(_) => {
			log::error!("Failed to read WRL header from file: {}", file_path.display());
			return Err(());
		}
	};

    let expected_file_size = calc_expected_file_size(
		wrl_header.width,
		wrl_header.height,
		wrl_header.tile_count,
	);

	let file_size = match get_file_size(file_path) {
		Ok(size) => size,
		Err(_) => {
			log::error!("Failed to get file size for: {}", file_path.display());
			return Err(());
		}
	};

	if expected_file_size > file_size {
		log::error!("File size is smaller than expected WRL data size: {}", file_path.display());
		return Err(());
	}

	if expected_file_size == file_size {
		return hash_full_file(file_path);
	}

	hash_part_of_file(file_path, expected_file_size)
}


fn hash_full_file(path: &Path) -> Result<String, ()> {
	let mut hasher = blake3::Hasher::new();

	if let Err(_) = hasher.update_mmap_rayon(path) {
		log::error!("Failed to hash file: {}", path.display());
		return Err(());
	}

	let hash = hasher.finalize();

	Ok(hash.to_string())
}

fn hash_part_of_file(file_path: &Path, bytes_to_hash: u64) -> Result<String, ()> {
	let file = match File::open(file_path) {
		Ok(file) => file,
		Err(e) => {
			log::error!("Failed to open file: {}", file_path.display());
			log::error!("{}", e);
			return Err(());
		}
	};

	let mmap = unsafe {
		match Mmap::map(&file) {
			Ok(mmap) => mmap,
			Err(_) => {
				log::error!("Failed to map file: {}", file_path.display());
				return Err(());
			}
		}
	};

	let data_to_hash = &mmap[..(bytes_to_hash as usize)];
    let mut hasher = Hasher::new();

    hasher.update_rayon(data_to_hash);

	let hash = hasher.finalize();

	Ok(hash.to_string())
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_hash_wrl_file_without_tail() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/REF.WRL");
			let expected_hash = "93bcb82dc19b0d7a9a6eb9faf54869c9d3e353e0fdf4a847096ceff8720614aa";

			// Act
			let hash = hash_wrl_file_without_tail(&test_file_path).unwrap();

			// Assert
			assert_eq!(hash, expected_hash.to_string());

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_hash_wrl_file_skip_tail() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/REF.WRL.TAIL");
			let expected_hash = "93bcb82dc19b0d7a9a6eb9faf54869c9d3e353e0fdf4a847096ceff8720614aa";

			// Act
			let hash = hash_wrl_file_without_tail(&test_file_path).unwrap();

			// Assert
			assert_eq!(hash, expected_hash.to_string());

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_hash_wrl_file_broken() {
		run_test!({
			// Arrange
			let test_file_path = Path::new("test_files/reference/REF.WRL.TRIMMED");

			// Act
			let hash = hash_wrl_file_without_tail(&test_file_path);

			// Assert
			assert!(hash.is_err());

			let logs = get_captured_logs();
			assert_eq!(logs.len(), 1);
			assert_eq!(logs[0], "[ERROR] File size is smaller than expected WRL data size: test_files/reference/REF.WRL.TRIMMED");
		});
	}
}

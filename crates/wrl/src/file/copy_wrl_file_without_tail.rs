use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use crate::file::calc_expected_file_size::calc_expected_file_size;
use crate::file::get_file_size::get_file_size;
use crate::file::read_wrl_header::read_wrl_header;

pub fn copy_wrl_file_without_tail(
	src: &Path,
	dest: &Path,
) -> Result<(), ()> {
	if !src.exists() {
		log::error!("Source file does not exist: {}", src.display());
		return Err(());
	}

	if dest.exists() {
		log::warn!("Destination file already exists: {}", dest.display());
		return Err(());
	}

	let wrl_header = match read_wrl_header(src) {
		Ok(header) => header,
		Err(_) => {
			log::error!("Failed to read WRL header from source file: {}", src.display());
			return Err(());
		}
	};

	let wrl_data_length = calc_expected_file_size(
		wrl_header.width,
		wrl_header.height,
		wrl_header.tile_count,
	);

	let file_size = match get_file_size(src) {
		Ok(size) => size,
		Err(_) => {
			log::error!("Failed to get file size for source file: {}", src.display());
			return Err(());
		}
	};

	if file_size < wrl_data_length {
		log::error!("Source file size is smaller than expected WRL data size: {}", src.display());
		return Err(());
	}

	if let Err(_) = copy_file_up_to_length(src, dest, wrl_data_length as u64) {
		log::error!("Failed to copy WRL file from {} to {}. Reverting changes.", src.display(), dest.display());

		if let Err(e) = std::fs::remove_file(dest) {
			log::error!("Revert failed: Failed to remove destination file after copy failure: {}. Aborting.", dest.display());
			log::error!("{}", e);
			log::info!("Destination file: {} was not removed. Please remove it manually.", dest.display());
		}

		return Err(());
	}

	Ok(())
}

/// Copies content from a source file to a destination file up to a specified length.
fn copy_file_up_to_length(
    src_path: &Path,
    dest_path: &Path,
    length: u64,
) -> Result<(), ()> {
    let mut src_file = match File::open(src_path) {
		Ok(file) => file,
		Err(e) => {
			log::error!("Failed to open source file: {}", e);
			return Err(());
		}
	};

    let mut dest_file = match File::create(dest_path) {
		Ok(file) => file,
		Err(e) => {
			log::error!("Failed to create destination file: {}", e);
			return Err(());
		}
	};

    let mut buffer = [0; 8 * 1024];
    let mut bytes_copied: u64 = 0;

    loop {
        let bytes_to_read_in_chunk = (length - bytes_copied).min(buffer.len() as u64) as usize;

        if bytes_to_read_in_chunk == 0 {
            break;
        }

        let bytes_read = match src_file.read(&mut buffer[..bytes_to_read_in_chunk]) {
            Ok(bytes) => bytes,
            Err(e) => {
                log::error!("Failed to read from source file: {}", e);
                return Err(());
            }
        };

        if bytes_read == 0 {
            break;
        }

        if let Err(e) = dest_file.write_all(&buffer[..bytes_read]) {
            log::error!("Failed to write to destination file: {}", e);
            return Err(());
        }

        bytes_copied += bytes_read as u64;

        if bytes_copied >= length {
            break;
        }
    }

    if let Err(e) = dest_file.flush() {
        log::error!("Failed to flush destination file: {}", e);
        return Err(());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_copy_wrl_file_no_tail() {
		run_test!({
			// Arrange
			std::fs::create_dir_all("test_files/temp").expect("Failed to create temp directory");
			let ref_path = Path::new("test_files/reference/REF.WRL");
			let src_path = Path::new("test_files/reference/REF.WRL.TAIL");
			let dest_path = Path::new("test_files/temp/REF.WRL.COPY");

			// Act
			let result = copy_wrl_file_without_tail(&src_path, &dest_path);

			// Assert
			assert!(result.is_ok());
			assert!(dest_path.exists());

			// Verify the copied file matches the reference
			let ref_content = std::fs::read(ref_path).expect("Failed to read reference file");
			let dest_content = std::fs::read(dest_path).expect("Failed to read destination file");
			assert_eq!(ref_content, dest_content);

			// Clean up
			if let Err(e) = std::fs::remove_file(dest_path) {
				log::error!("Failed to remove test file after test: {}", e);
			}

			let log = get_captured_logs();
			assert!(log.is_empty());
		});
	}
}

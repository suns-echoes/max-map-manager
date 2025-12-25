use std::path::Path;

use crate::{
    verify_file_content, verify_tail_header
};

pub fn verify_file(file_path: &Path) -> Result<bool, String> {
    let file_size = match file_path.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_) => {
			let error_message = format!("Failed to get file metadata: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
        }
    };

    let wrl_header = match crate::read_wrl_header(file_path) {
        Ok(header) => header,
        Err(_) => {
			let error_message = format!("Failed to read WRL header from file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
        }
    };

    let expected_file_size =
        crate::calc_expected_file_size(wrl_header.width, wrl_header.height, wrl_header.tile_count);

    if file_size >= expected_file_size {
		if let Err(e) = verify_file_content(file_path) {
			let error_message = format!("Invalid WRL file structure: {}: {}", file_path.display(), e);
			log::error!("{}", error_message);
			return Err(error_message);
		}
    }

	if file_size > expected_file_size {
        let is_tail_valid = verify_tail_header(file_path);
        if !is_tail_valid {
			let error_message = format!("Invalid tail header in file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
        }
    }

    Ok(true)
}

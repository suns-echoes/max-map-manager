use std::path::Path;

use crate::read_wrl_tail_header;

pub fn verify_tail_header(file_path: &Path) -> bool {
	let _ = match read_wrl_tail_header(file_path) {
		Ok(header) => header,
		Err(_) => {
			log::error!("Failed to read tail header of file: {}", file_path.display());
			return false;
		}
	};

	true
}

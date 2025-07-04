use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::v70;


pub struct SaveFileInfo {
	pub file_path: PathBuf,
	pub header: v70::Header,
}


pub fn find_save_files(saves_dir_path: &Path) -> Result<Vec<SaveFileInfo>, String> {
	let save_files_extensions = ["DTA", "BAK", "TRA", "CAM", "HOT", "MLT", "DMO", "DBG", "TXT", "SCE", "MPS"];
	let mut save_files = Vec::new();

	for entry in fs::read_dir(saves_dir_path).map_err(|e| format!("Failed to read saves directory: {}", e))? {
		let entry = entry.map_err(|e| format!("Failed to read saves directory entry: {}", e))?;
		let save_file_path = entry.path();

		if save_file_path.is_file() {
			if let Some(ext) = save_file_path.extension() {
				if save_files_extensions.contains(&ext.to_ascii_uppercase().to_str().unwrap_or("")) {
					let save_data = v70::load_save_file_header_v70(&save_file_path);
					if let Some(save) = save_data.ok() {
						save_files.push(SaveFileInfo {
							file_path: save_file_path,
							header: save,
						});
					}
				}
			}
		}
	}

	Ok(save_files)
}

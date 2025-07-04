use std::path::{Path, PathBuf};

use crate::{file_path_to_planet_index, SaveFileInfo};


pub fn get_related_save_files(save_files: &Vec<SaveFileInfo>, wrl_file_path: &Path) -> Vec<PathBuf> {
	let planet_index = file_path_to_planet_index(&wrl_file_path);
	if let Some(planet_index) = planet_index {
		save_files.iter()
			.filter(|info| info.header.planet == planet_index)
			.map(|info| info.file_path.clone()).collect()
	} else {
		Vec::new()
	}
}

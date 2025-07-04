use std::fs;
use std::path::{Path, PathBuf};

use wrl::read_wrl_header::read_wrl_header;

use crate::common::str_array_contains;
use crate::{file_path_to_planet_index, v70};

pub fn find_related_save_files(
    map_file_path: &Path,
    saves_dir_path: &Path,
) -> Result<Vec<PathBuf>, String> {
    let save_files_extensions = [
        "DTA", "BAK", "TRA", "CAM", "HOT", "MLT", "DMO", "DBG", "TXT", "SCE", "MPS",
    ];
    let planet_index = file_path_to_planet_index(&map_file_path);
    let wrl_data = read_wrl_header(map_file_path).map_err(|_| {
        format!(
            "Failed to read WRL header for map file: {}.",
            map_file_path.display()
        )
    })?;
    let width = wrl_data.width;
    let height = wrl_data.height;
    let mut related_saves = Vec::new();

    if let Some(planet_index) = planet_index {
        for entry in fs::read_dir(saves_dir_path)
            .map_err(|e| format!("Failed to read saves directory: {}", e))?
        {
            let entry =
                entry.map_err(|e| format!("Failed to read saves directory entry: {}", e))?;
            let save_file_path = entry.path();

            if save_file_path.is_file() {
                let file_ext = &save_file_path
                    .extension()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_uppercase();
                if str_array_contains(&save_files_extensions, &file_ext) {
                    let save_data = v70::load_save_file_v70(&save_file_path, width, height);
                    if let Some(save) = save_data.ok() {
                        if save.header.planet == planet_index {
                            related_saves.push(save_file_path);
                        }
                    }
                }
            }
        }
    }

    Ok(related_saves)
}

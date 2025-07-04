use std::ffi::OsStr;
use std::{fs, fs::File};
use std::path::{Path, PathBuf};

use crate::registry::{Registry, RegistryLoadError, RegistrySaveError};


pub fn get_file_name_from_path(path: &Path) -> Result<String, String> {
    let os_str_file_name: Option<&OsStr> = path.file_name();

    match os_str_file_name {
        Some(os_str) => {
            match os_str.to_str() {
                Some(str_file_name) => Ok(str_file_name.to_string()),
                None => Err(format!(
                    "Failed to convert file name '{:?}' to UTF-8. Path: {:?}",
                    os_str, path
                )),
            }
        }
        None => Err(format!(
            "The provided path does not have a file name. Path: {:?}",
            path
        )),
    }
}


pub fn check_map_path(
    map_file_path: &Path,
) -> Result<(), String> {
    is_path_valid_utf8(&map_file_path)?;
    if !map_file_path.exists() {
        log::error!("Map file does not exist: {}", map_file_path.display());
        return Err(format!(
            "Could not find map file: {}.\nIf this issue persists, please restart application.",
            map_file_path.file_name().unwrap().to_string_lossy()
        ));
    }

    Ok(())
}

pub fn check_saves_paths(
    saves_files_paths: &Vec<PathBuf>,
) -> Result<(), String> {
    for save_file_path in saves_files_paths {
        is_path_valid_utf8(&save_file_path)?;
        if !save_file_path.exists() {
            log::error!("Save file does not exist: {}", save_file_path.display());
            return Err(format!(
                "Could not find save file: {}.\nIf this issue persists, please restart application.",
                save_file_path.file_name().unwrap().to_string_lossy()
            ));
        }
    }

    Ok(())
}

pub fn check_archive_dir_path(
    archive_dir_path: &Path,
) -> Result<(), String> {
    is_path_valid_utf8(&archive_dir_path)?;
    if !archive_dir_path.exists() {
        log::error!(
            "Archive directory does not exist: {}",
            archive_dir_path.display()
        );
        return Err(
            "Could not find archive directory. Please check setting and retry.".to_string(),
        );
    }

    Ok(())
}

pub fn check_game_dir_path(
    game_dir_path: &Path,
) -> Result<(), String> {
    is_path_valid_utf8(&game_dir_path)?;
    if !game_dir_path.exists() {
        log::error!("Game directory does not exist: {}", game_dir_path.display());
        return Err("Could not find game directory. Please check setting and retry.".to_string());
    }

    Ok(())
}

pub fn check_saves_dir_path(
    saves_dir_path: &Path,
) -> Result<(), String> {
    is_path_valid_utf8(&saves_dir_path)?;
    if !saves_dir_path.exists() {
        log::error!(
            "Saves directory does not exist: {}",
            saves_dir_path.display()
        );
        return Err("Could not find saves directory. Please check setting and retry.".to_string());
    }

    Ok(())
}

fn is_path_valid_utf8(path: &Path) -> Result<(), String> {
    if path.to_str().is_none() {
        return Err(format!("Path is not valid UTF-8: {}", path.display()));
    }
    Ok(())
}


pub fn verify_file_integrity(src_path: &Path, dest_path: &Path) -> Result<(), String> {
    let mut src_file = File::open(src_path).map_err(|e| {
        log::error!(
            "Failed to open source file for integrity check: {}",
            src_path.display()
        );
        log::error!("{}", e);
        format!(
            "Unexpected error during file integrity check: {}",
            src_path.display()
        )
    })?;

    let mut dest_file = File::open(dest_path).map_err(|e| {
        log::error!(
            "Failed to open destination file for integrity check: {}",
            dest_path.display()
        );
        log::error!("{}", e);
        format!(
            "Unexpected error during file integrity check: {}",
            dest_path.display()
        )
    })?;

    let mut src_hasher = blake3::Hasher::new();
    let mut dest_hasher = blake3::Hasher::new();

    if let Err(_) = src_hasher.update_reader(&mut src_file) {
        log::error!("Failed to hash file: {}", src_path.display());
        return Err(format!(
            "Unexpected error during file integrity check: {}",
            src_path.display()
        ));
    }

    if let Err(_) = dest_hasher.update_reader(&mut dest_file) {
        log::error!("Failed to hash file: {}", dest_path.display());
        return Err(format!(
            "Unexpected error during file integrity check: {}",
            dest_path.display()
        ));
    }

    let src_hash = src_hasher.finalize();
    let dest_hash = dest_hasher.finalize();

    if src_hash != dest_hash {
        log::error!(
            "File integrity check failed: {} != {}",
            src_path.display(),
            dest_path.display()
        );
        return Err(format!(
            "File integrity check failed for {}. The copied file does not match the original.",
            src_path.display()
        ));
    }

    Ok(())
}

pub fn remove_map_archive_map_directory(path_to_archived_map: &Path) -> Result<(), String> {
	log::info!("Removing archive map directory: {}", path_to_archived_map.display());
	if let Err(e) = std::fs::remove_dir_all(path_to_archived_map) {
		log::error!("Failed to remove archive map directory: {}.", path_to_archived_map.display());
		log::error!("{}", e);
		return Err(format!("Failed to remove archive map directory: {}", path_to_archived_map.display()));
	}

	Ok(())
}

pub fn remove_map_archive_directory(path: &Path) -> Result<(), String> {
	log::info!("Removing archive directory: {}", path.display());
	if let Err(e) = std::fs::remove_dir_all(path) {
		log::error!("Failed to remove archive directory: {}.", path.display());
		log::error!("{}", e);
		return Err(format!("Failed to remove archive directory: {}", path.display()));
	}

	Ok(())
}


pub fn remove_files(files: &[String]) -> Result<(), String> {
	for file in files {
		let path = Path::new(file);
		if path.exists() {
			if let Err(e) = fs::remove_file(path) {
				log::error!("Failed to remove file: {}. Error: {}", path.display(), e);
				return Err(format!("Failed to remove file: {}", path.display()));
			}
		} else {
			log::warn!("File does not exist, skipping removal: {}", path.display());
		}
	}
	Ok(())
}


pub fn load_registry(registry_path: &Path) -> Result<Registry, String> {
    let registry = Registry::from_file(&registry_path).map_err(|e| match e {
        RegistryLoadError::FileNotFound => {
            "Registry file not found. Please check your settings.".to_string()
        }
        RegistryLoadError::FailedToReadFile => {
            "Failed to read registry file. Please check if the file is accessible and valid."
                .to_string()
        }
        RegistryLoadError::FailedToParseJson => {
            "Registry file seems to be corrupted. Please check if the file is valid JSON."
                .to_string()
        }
    })?;

    Ok(registry)
}


pub fn save_registry(registry: &Registry) -> Result<(), String> {
    registry.save().map_err(|e| match e {
        RegistrySaveError::FailedToSerialize => {
            "Unexpected error occurred during registry serialization".to_string()
        }
        RegistrySaveError::FailedToWriteFile => {
            "Failed to write registry to file. Please check your permissions and try again."
                .to_string()
        }
        _ => "Unexpected error occurred while saving the registry.".to_string(),
    })?;

    Ok(())
}

/// The `registry.json` file structure:
///
/// ```json
/// {
///  "version": "1.0",
/// 	"archive": {
/// 		"map_1_id": {
/// 			"path": "path/to/map_1.wrl",
/// 			"saves": ["path/to/save1_1.dta", "path/to/save1_2.dta"]
/// 		},
/// 		"map_2_id": {
/// 			"path": "path/to/map_2.wrl",
/// 			"saves": ["path/to/save2_1.dta", "path/to/save2_2.dta"]
/// 		}
/// 	}
/// }
/// ``````
///
/// This file is used to store the state of the archive of maps and their associated saves.

use serde::{Deserialize, Serialize};
use chrono::Local;
use std::collections::BTreeMap;
use std::fs;
use std::{io, io::Write};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RegistryMapEntry {
    pub map: String,
    pub saves: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RegistryArchive {
    #[serde(flatten)]
    pub maps: BTreeMap<String, RegistryMapEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Registry {
    #[serde(skip_serializing, default)]
    pub file_path: PathBuf,
    pub version: String,
    pub archive: RegistryArchive,
}

#[derive(Debug, PartialEq)]
pub enum RegistryLoadError {
	FileNotFound,
    FailedToReadFile,
	FailedToParseJson,
}

#[derive(Debug, PartialEq)]
pub enum RegistrySaveError {
	FailedToSerialize,
	FailedToCreateFile,
	FailedToWriteDirectory,
	FailedToWriteFile,
	FailedToMoveOldTemp,
	FailedToMoveJSON,
	FailedToMoveTemp,
	FailedToRemoveOldBak
}

pub enum RegistryInitError {
    FailedToReadFile,
	FailedToParseJson,
	FailedToInitializeFile,
}

impl Registry {
	pub fn new() -> Self {
		Registry {
			file_path: PathBuf::new(),
			version: "1.0".to_string(),
			archive: RegistryArchive {
				maps: BTreeMap::new(),
			},
		}
	}

    pub fn from_file(file_path: &Path) -> Result<Registry, RegistryLoadError> {
		if !file_path.exists() {
			log::error!("Registry file does not exist: {}", file_path.display());
			return Err(RegistryLoadError::FileNotFound);
		}

        let file_content = fs::read_to_string(file_path).map_err(|e| {
			log::error!("Failed to read file {}", file_path.display());
			log::error!("{}", e);
			RegistryLoadError::FailedToReadFile
		})?;

        let mut registry: Registry = serde_json::from_str(&file_content).map_err(|e| {
			log::error!("Failed to parse JSON from file {}", file_path.display());
			log::error!("{}", e);
			RegistryLoadError::FailedToParseJson
		})?;

		registry.file_path = file_path.to_path_buf();

        Ok(registry)
    }

	pub fn init_from_file(&mut self, file_path: &Path) -> Result<(), RegistryInitError> {
		if !file_path.exists() {
			self.file_path = file_path.to_path_buf();
			match self.save() {
				Ok(_) => return Ok(()),
				Err(e) => {
					match e {
						RegistrySaveError::FailedToSerialize |
						RegistrySaveError::FailedToCreateFile |
						RegistrySaveError::FailedToWriteDirectory |
						RegistrySaveError::FailedToWriteFile => {
							log::error!("Failed to initialize new registry file: {}", file_path.display());
							return Err(RegistryInitError::FailedToInitializeFile);
						}
						_ => {}
					}
				}
			}
		}

		if let Err(e) = self.load(file_path) {
			match e {
				RegistryLoadError::FileNotFound => {
					log::error!("Registry file does not exist (unexpected): {}", file_path.display());
					return Err(RegistryInitError::FailedToReadFile);
				}
				RegistryLoadError::FailedToReadFile => {
					log::error!("Failed to load registry file due to I/O error: {}", file_path.display());
					return Err(RegistryInitError::FailedToReadFile);
				}
				RegistryLoadError::FailedToParseJson => {
					log::error!("Failed to parse registry file (invalid JSON): {}", file_path.display());
					return Err(RegistryInitError::FailedToParseJson);
				}
			}
		}

		Ok(())
	}

    pub fn load(&mut self, file_path: &Path) -> Result<(), RegistryLoadError> {
		if !file_path.exists() {
			log::error!("Registry file does not exist: {}", file_path.display());
			return Err(RegistryLoadError::FileNotFound);
		}

        let file_content = fs::read_to_string(file_path).map_err(|e| {
			log::error!("Failed to read file {}", file_path.display());
			log::error!("{}", e);
			RegistryLoadError::FailedToReadFile
		})?;

		let registry: Registry = serde_json::from_str(&file_content).map_err(|e| {
			log::error!("Failed to parse JSON from file {}", file_path.display());
			log::error!("{}", e);
			RegistryLoadError::FailedToParseJson
		})?;

		self.file_path = file_path.to_path_buf();
		*self = registry;

		Ok(())
    }

    pub fn save_as(&self, file_path: &Path) -> Result<(), RegistrySaveError> {
		let timestamp = Local::now().format("%Y_%m_%d.%H_%M_%S").to_string();

		let temp_file_path = file_path.with_extension(format!("json.temp.{}", timestamp));
		let backup_file_path = file_path.with_extension("json.bak");
		let moved_backup_file_path = file_path.with_extension(format!("json.bak.moved.{}", timestamp));

		let json_string = serialize(self)?;

		write_registry_to_file(&temp_file_path, &json_string)?;

		if file_path.exists() {
			if backup_file_path.exists() {
				rename_old_bak_file(&backup_file_path, &moved_backup_file_path)?;
			}

			rename_json_file(&file_path, &backup_file_path)?;
		}

		rename_temp_file(&temp_file_path, file_path)?;

		remove_old_bak_file(&moved_backup_file_path)?;

        Ok(())
    }

	pub fn save(&self) -> Result<(), RegistrySaveError> {
		self.save_as(&self.file_path)
	}

    pub fn set_map_entry(&mut self, map_id: &str, map_entry: RegistryMapEntry) {
        self.archive.maps.insert(map_id.to_string(), map_entry);
    }

    pub fn remove_map_entry(&mut self, map_id: &str) -> Option<RegistryMapEntry> {
        self.archive.maps.remove(map_id)
    }

	pub fn get_map_entry(&self, map_id: &str) -> Option<RegistryMapEntry> {
		if self.archive.maps.contains_key(map_id) {
			Some(self.archive.maps.get(map_id).unwrap().clone())
		} else {
			None
		}
	}

	pub fn get_map_entry_mut(&mut self, map_id: &str) -> Option<&mut RegistryMapEntry> {
		self.archive.maps.get_mut(map_id)
	}

	pub fn has_map_entry(&self, map_id: &str) -> bool {
		self.archive.maps.contains_key(map_id)
	}

	pub fn to_string(&self, pretty: bool) -> Result<String, RegistrySaveError> {
		if pretty {
			serde_json::to_string_pretty(self).map_err(|e| {
				log::error!("Failed to serialize Registry.");
				log::error!("{}", e);
				RegistrySaveError::FailedToSerialize
			})
		} else {
			serde_json::to_string(self).map_err(|e| {
				log::error!("Failed to serialize Registry.");
				log::error!("{}", e);
				RegistrySaveError::FailedToSerialize
			})
		}
	}
}


fn serialize(registry: &Registry) -> Result<String, RegistrySaveError> {
	let json_string = serde_json::to_string_pretty(registry).map_err(|e| {
		log::error!("Failed to serialize Registry.");
		log::error!("{}", e);
		RegistrySaveError::FailedToSerialize
	})?;
	Ok(json_string)
}

fn write_registry_to_file(file_path: &Path, json_string: &str) -> Result<(), RegistrySaveError> {
	if !file_path.exists() {
		create_directories_from_filepath(file_path.to_str().unwrap())
			.map_err(|e| {
				log::error!("Failed to create directories for new path: {}", file_path.display());
				log::error!("{}", e);
				RegistrySaveError::FailedToWriteDirectory
			})?;
	}

	let mut file = fs::File::create(file_path).map_err(|e| {
		log::error!("Failed to create file {}", file_path.display());
		log::error!("{}", e);
		RegistrySaveError::FailedToCreateFile
	})?;

	file.write_all(json_string.as_bytes()).map_err(|e| {
		log::error!("Failed to write to file {}", file_path.display());
		log::error!("{}", e);
		RegistrySaveError::FailedToWriteFile
	})?;

	if let Err(e) = file.sync_all() {
		log::warn!("Failed to sync file {}", file_path.display());
		log::warn!("{}", e);
	}

	Ok(())
}

fn rename_old_bak_file(old_path: &Path, new_path: &Path) -> Result<(), RegistrySaveError> {
	if old_path.exists() {
		if !new_path.exists() {
			create_directories_from_filepath(new_path.to_str().unwrap())
				.map_err(|e| {
					log::error!("Failed to create directories for new path: {}", new_path.display());
					log::error!("{}", e);
					RegistrySaveError::FailedToWriteDirectory
				})?;
		}
		fs::rename(old_path, new_path).map_err(|e| {
			log::error!("Failed to rename old backup file from {} to {}", old_path.display(), new_path.display());
			log::error!("{}", e);
			RegistrySaveError::FailedToMoveOldTemp
		})?;
	}

	Ok(())
}

fn rename_json_file(old_path: &Path, new_path: &Path) -> Result<(), RegistrySaveError> {
	if old_path.exists() {
		if !new_path.exists() {
			create_directories_from_filepath(new_path.to_str().unwrap())
				.map_err(|e| {
					log::error!("Failed to create directories for new path: {}", new_path.display());
					log::error!("{}", e);
					RegistrySaveError::FailedToWriteDirectory
				})?;
		}
		fs::rename(old_path, new_path).map_err(|e| {
			log::error!("Failed to rename JSON file from {} to {}", old_path.display(), new_path.display());
			log::error!("{}", e);
			RegistrySaveError::FailedToMoveJSON
		})?;
	}

	Ok(())
}

fn rename_temp_file(temp_path: &Path, final_path: &Path) -> Result<(), RegistrySaveError> {
	if temp_path.exists() {
		fs::rename(temp_path, final_path).map_err(|e| {
			log::error!("Failed to rename temporary file from {} to {}", temp_path.display(), final_path.display());
			log::error!("{}", e);
			RegistrySaveError::FailedToMoveTemp
		})?;
	}

	Ok(())
}

fn remove_old_bak_file(path: &Path) -> Result<(), RegistrySaveError> {
	if path.exists() {
		fs::remove_file(path).map_err(|e| {
			log::error!("Failed to remove old backup file: {}", path.display());
			log::error!("{}", e);
			RegistrySaveError::FailedToRemoveOldBak
		})?;
	}

	Ok(())
}

fn create_directories_from_filepath(filepath: &str) -> io::Result<()> {
    let path = Path::new(filepath);

    if let Some(directory) = path.parent() {
        if directory.as_os_str().is_empty() {
            println!("Path does not have a directory component: {}", filepath);
            return Ok(());
        }

        match std::fs::create_dir_all(directory) {
            Ok(_) => {
				log::info!("Directories created: {}", directory.display());
				log::info!("  for file: {:?}", &filepath);
                Ok(())
            }
            Err(e) => {
                eprintln!("Error creating directories for {:?}: {}", directory, e);
                Err(e)
            }
        }
    } else {
        println!("Path does not have a directory component: {}", filepath);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

	use test_utils::*;
	use crate::local_test_utils::*;

	#[test]
	fn test_registry_has_map_entry_api() {
		run_test!({
			// Arrange
			let registry = create_test_registry(1, &PathBuf::from(""));
			let map_id = "map_id_1".to_string();

			// Assert
			assert!(registry.has_map_entry(&map_id));
		});
	}

	#[test]
	fn test_registry_get_map_entry_api() {
		run_test!({
			// Arrange
			let registry = create_test_registry(1, &PathBuf::from(""));
			let map_id = "map_id_1".to_string();
			let map_entry = RegistryMapEntry {
				map: "path/to/map1.wrl".to_string(),
				saves: vec!["path/to/save1_1.dta".to_string(), "path/to/save1_2.dta".to_string()],
			};

			// Assert
			assert_eq!(registry.get_map_entry(&map_id), Some(map_entry));
		});
	}

	#[test]
	fn test_registry_set_map_entry_api() {
		run_test!({
			// Arrange
			let mut registry = Registry::new();
			let map_id = "map_id_1".to_string();
			let map_entry = RegistryMapEntry {
				map: "path/to/map1.wrl".to_string(),
				saves: vec!["path/to/save1_1.dta".to_string(), "path/to/save1_2.dta".to_string()],
			};

			// Act
			registry.set_map_entry(&map_id, map_entry.clone());

			// Assert
			assert!(registry.has_map_entry("map_id_1"));
			assert_eq!(registry.get_map_entry("map_id_1"), Some(map_entry));
		});
	}

	#[test]
	fn test_registry_remove_map_entry_api() {
		run_test!({
			// Arrange
			let mut registry = create_test_registry(1, &PathBuf::from(""));
			let map_id = "map_id_1".to_string();

			// Act
			registry.remove_map_entry(&map_id);

			// Assert
			assert!(!registry.has_map_entry("map_id_1"));
		});
	}

	#[test]
	fn test_registry_to_string() {
		run_test!({
			// Arrange
			let reference_json = create_reference_registry_json();
			let registry = create_test_registry(2, &PathBuf::from(""));

			// Act
			let serialized = registry.to_string(false).unwrap();

			// Assert
			assert_eq!(&serialized, &reference_json);
		});
	}

	#[test]
	fn test_registry_from_file() {
		run_test!({
			// Arrange
			let file_path = get_reference_registry_path();
			let reference_registry = create_test_registry(2, &file_path);

			// Act
			let registry = Registry::from_file(&file_path).expect("Failed to load reference registry");

			// Assert
			assert_eq!(&reference_registry, &registry);
		});
	}

	#[test]
	fn test_registry_to_file() {
		run_test!({
			// Arrange
            let test_fs = TestFileSystem::new();
			let reference_file_content = load_reference_registry_json();
			let registry_path = test_fs.create_test_file("registry.json", "registry.json content");
			let registry = create_test_registry(2, &registry_path);

			// Act
			registry.save_as(&registry_path).expect("Failed to save registry to file");

			// Assert
			assert!(test_fs.file_has_content(&registry_path, &reference_file_content));
		});
	}

	#[test]
	fn test_registry_file_transaction() {
		run_test!({
			// Arrange
            let test_fs = TestFileSystem::new();
			let reference_file_content = load_reference_registry_json();
			let registry_path = test_fs.create_test_file("registry.json", "registry.json content");
			let registry_bak_path = test_fs.create_test_file("registry.json.bak", "registry.json.bak content");
			let registry = create_test_registry(2, &registry_path);

			// Act
			registry.save_as(&registry_path).expect("Failed to save registry to file");

			// Assert
			assert!(test_fs.file_has_content(&registry_path, &reference_file_content));
			assert!(test_fs.file_has_content(&registry_bak_path, "registry.json content"));
		});
	}
}

use std::fs;
use std::path::{Path, PathBuf};

use crate::registry::*;
use crate::common::*;


pub fn archive_files(
    map_hash_id: &str,
    map_file_path: &Path,
    saves_files_paths: &Vec<PathBuf>,
    archive_dir_path: &Path,
    game_dir_path: &Path,
    saves_dir_path: &Path,
) -> Result<(), String> {
    let map_file_name = match map_file_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            log::error!(
                "Map file path does not have a valid file name: {}",
                map_file_path.display()
            );
            return Err("Invalid map file path".to_string());
        }
    };

    log::info!("Starting archivization process: {}", map_hash_id);
    log::info!("  map file: {}", map_file_name);
    log::info!("  save files:");
    saves_files_paths.iter().for_each(|save_path| {
        log::info!("    - {}", save_path.display());
    });

    check_map_path(&map_file_path)?;
    check_saves_paths(&saves_files_paths)?;
    check_archive_dir_path(&archive_dir_path)?;
    check_game_dir_path(&game_dir_path)?;
    check_saves_dir_path(&saves_dir_path)?;

    let registry_path = archive_dir_path.join("registry.json");
    let mut registry = load_registry(&registry_path)?;

    let map_archive_path = create_map_archive_directory(&archive_dir_path, &map_hash_id)?;

    moves_files_to_archive(
        &map_hash_id,
        &map_file_path,
        &saves_files_paths,
        &map_archive_path,
        &mut registry,
    )?;

    save_registry(&registry)?;

	let archived_files: Vec<String> = vec![map_file_path.to_string_lossy().to_string()]
		.into_iter()
		.chain(saves_files_paths.iter().map(|p| p.to_string_lossy().to_string()))
		.collect();

	remove_files(&archived_files)?;

	log::info!("Archivization process completed successfully.");

    Ok(())
}


fn create_map_archive_directory(
    archive_dir_path: &Path,
    map_hash_id: &str,
) -> Result<PathBuf, String> {
    let map_archive_path = archive_dir_path.join(map_hash_id).to_path_buf();

    if map_archive_path.exists() {
        if !map_archive_path.is_dir() {
            log::error!(
                "Map archive path exists but is not a directory: {}",
                map_archive_path.display()
            );
            return Err(format!(
                "Map archive path exists but is not a directory: {}.\nPlease check this path and remove obstruction.",
                map_archive_path.display()
            ));
        }

        let dir_entries = match fs::read_dir(&map_archive_path) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read map archive directory: {}", e);
                return Err(format!(
                    "Failed to read map archive directory: {}.\nPlease check your permissions and try again.",
                    map_archive_path.display()
                ));
            }
        };

        if dir_entries.count() > 0 {
            log::error!(
                "Map archive directory is not empty: {}",
                map_archive_path.display()
            );
            return Err(format!(
                "Map archive directory is not empty: {}.\nPlease check this path and remove obstruction.",
                map_archive_path.display()
            ));
        }
    } else {
        if let Err(e) = fs::create_dir_all(&map_archive_path) {
            log::error!("Failed to create map archive directory: {}", e);
            return Err(format!(
                "Failed to create map archive directory: {}.\nPlease check your permissions and try again.",
                map_archive_path.display()
            ));
        }

        log::info!(
            "Created map archive directory: {}",
            map_archive_path.display()
        );
    }

    Ok(map_archive_path)
}

fn moves_files_to_archive(
    map_hash_id: &str,
    map_file_path: &Path,
    saves_files_paths: &Vec<PathBuf>,
    map_archive_path: &Path,
    registry: &mut Registry,
) -> Result<(), String> {
    if registry.has_map_entry(&map_hash_id) {
        panic!("Entry already exists. Is registry corrupted?");
		// TODO: Offer archive auto-resync as a solution?
    }

	{
		let map_file_name = get_file_name_from_path(&map_file_path)?;
		let dest_path = map_archive_path.join(&map_file_name);
		copy_file_to_archive(&map_file_path, &dest_path, &map_archive_path)?;

		registry.set_map_entry(
			map_hash_id,
			RegistryMapEntry {
				map: map_file_name,
				saves: vec![],
			},
		);
	}

    let registry_saves = &mut registry.get_map_entry_mut(&map_hash_id)
        .expect("Failed to get map entry from registry")
        .saves;

    for src_path in saves_files_paths {
        let save_file_name = get_file_name_from_path(src_path)?;
        let dest_path = map_archive_path.join(&save_file_name);
		copy_file_to_archive(src_path, &dest_path, &map_archive_path)?;

		registry_saves.push(save_file_name);
	}

    Ok(())
}

fn copy_file_to_archive(
	src_path: &Path,
	dest_path: &Path,
	map_archive_path: &Path,
) -> Result<(), String> {
	if let Err(e) = fs::copy(src_path, &dest_path) {
		if let Err(e) = fs::File::open(&dest_path).and_then(|f| f.sync_all()) {
			log::error!(
				"Failed to sync destination file {}: {}",
				dest_path.display(),
				e
			);
		}

		log::error!(
			"Failed to copy file {} to {}",
			src_path.display(),
			dest_path.display()
		);
		log::error!("{}", e);

		moves_files_to_archive_revert(map_archive_path)?;

		return Err(format!(
			"Failed to move file to archive: {}",
			src_path.display()
		));
	}

	if let Err(_) = verify_file_integrity(&src_path, &dest_path) {
		moves_files_to_archive_revert(&map_archive_path)?;

		return Err(format!(
			"Failed to move file to archive: {}",
			src_path.display()
		));
	}

	Ok(())
}

fn moves_files_to_archive_revert(path: &Path) -> Result<(), String> {
    log::warn!("Reverting archivization, target: {}", path.display());

    if let Err(_) = remove_map_archive_directory(path) {
        return Err(format!(
            "Archivization and revert failed. Please remove the directory manually: {}.",
            path.display()
        ));
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    use test_utils::*;
	use crate::local_test_utils::*;

    #[test]
    fn test_archive_files() {
        run_test!({
            // Arrange
            let test_fs = TestFileSystem::new();

            let archive_dir_path = test_fs.create_test_dir("archive");
            let game_dir_path = test_fs.create_test_dir("game");
            let saves_dir_path = test_fs.create_test_dir("saves");

            let map_hash_id = "00000000-TEST-MAP-HASH-000000000000";
            let map_file_path = test_fs.create_test_file("game/test_map.wrl", "Map content");
            let save_1_file_path = test_fs.create_test_file("saves/save_1.dta", "SAVE 1 content");
            let save_2_file_path = test_fs.create_test_file("saves/save_2.dta", "SAVE 2 content");
            let saves_files_paths_buf = vec![&save_1_file_path, &save_2_file_path];

            let saves_files_paths: Vec<PathBuf> =
                saves_files_paths_buf.iter().map(|p| {
					let mut path_buf = PathBuf::new();
					path_buf.push(p);
					path_buf
				}).collect();

            let registry_file_path = archive_dir_path.join("registry.json");
            let registry = create_test_registry(0, &registry_file_path);
            registry.save().expect("Failed to save registry for test");

            // Act
            let result = archive_files(
                map_hash_id,
                &map_file_path,
                &saves_files_paths,
                &archive_dir_path,
                &game_dir_path,
                &saves_dir_path,
            );

            // Assert
            assert!(result.is_ok());

			let expected_map_archive_path = archive_dir_path.join(map_hash_id);
			assert!(test_fs.is_path_directory(&expected_map_archive_path));

			let expected_map_file_path = expected_map_archive_path.join("test_map.wrl");

			assert!(test_fs.is_path_file(&expected_map_file_path));
			assert!(test_fs.file_has_content(&expected_map_file_path, "Map content"));
			assert!(!test_fs.file_exist(&map_file_path));

			assert!(test_fs.is_path_file(&expected_map_archive_path.join("save_1.dta")));
			assert!(test_fs.file_has_content(&expected_map_archive_path.join("save_1.dta"), "SAVE 1 content"));
			assert!(!test_fs.file_exist(&save_1_file_path));

			assert!(test_fs.is_path_file(&expected_map_archive_path.join("save_2.dta")));
			assert!(test_fs.file_has_content(&expected_map_archive_path.join("save_2.dta"), "SAVE 2 content"));
			assert!(!test_fs.file_exist(&save_2_file_path));

			let registry = Registry::from_file(&registry_file_path).expect("Failed to load registry");
			assert!(registry.has_map_entry(map_hash_id));

			let map_entry = registry.get_map_entry(map_hash_id).expect("Map entry not found");
			assert_eq!(map_entry.map, expected_map_archive_path.join("test_map.wrl").to_string_lossy().to_string());
			assert_eq!(map_entry.saves.len(), 2);
			assert!(map_entry.saves.contains(&expected_map_archive_path.join("save_1.dta").to_string_lossy().to_string()));
			assert!(map_entry.saves.contains(&expected_map_archive_path.join("save_2.dta").to_string_lossy().to_string()));
        });
    }
}

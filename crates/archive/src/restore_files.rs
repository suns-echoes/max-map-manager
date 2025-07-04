use std::fs;
use std::path::{Path, PathBuf};

use saves::v70::overwrite_planet_type_v70;

use crate::common::*;
use crate::registry::Registry;

pub fn restore_files(
    map_hash_id: &str,
    archive_dir_path: &Path,
    game_dir_path: &Path,
    saves_dir_path: &Path,
    target_map_file_name: String,
) -> Result<Vec<String>, String> {
    let registry_path = archive_dir_path.join("registry.json");
    let mut registry: Registry = load_registry(&registry_path).unwrap();

    let Some(map_entry) = registry.get_map_entry(map_hash_id) else {
        log::error!("No archive entry found for map hash ID: {}", map_hash_id);
        log::error!("Registry JSON is corrupted?");
        return Err(format!(
            "No archive entry found for map hash ID: {}. Is archive registry file corrupted?",
            map_hash_id
        ));
        // TODO: Offer archive auto-resync as a solution?
    };

    let mut map_dir_path = PathBuf::from(&archive_dir_path);
    map_dir_path.push(map_hash_id);
    let mut map_file_path = PathBuf::from(&map_dir_path);
    map_file_path.push(&map_entry.map);
    let saves_files_paths: Vec<PathBuf> = map_entry
        .saves
        .iter()
        .map(|s| {
            let mut save_path = PathBuf::from(&map_dir_path);
            save_path.push(s);
            save_path
        })
        .collect();

    log::info!("Starting restoration process: {}", map_hash_id);
    log::info!("  map file: {}", map_file_path.display());
    log::info!("  save files:");
    saves_files_paths.iter().for_each(|save_path| {
        log::info!("    - {}", save_path.display());
    });

    check_map_path(&map_file_path)?;
    check_saves_paths(&saves_files_paths)?;
    check_archive_dir_path(&archive_dir_path)?;
    check_game_dir_path(&game_dir_path)?;
    check_saves_dir_path(&saves_dir_path)?;

    move_map_file_to_game_dir(&map_file_path, &game_dir_path, &target_map_file_name)?;

    move_saves_files_to_saves_dir(&saves_files_paths, &saves_dir_path)?;

    for save_file_path in &saves_files_paths {
        let save_file_name = save_file_path.file_name().unwrap();
        let save_file_path = saves_dir_path.join(save_file_name);
        let map_slot_name = target_map_file_name
            .strip_suffix(".WRL")
            .unwrap_or(&target_map_file_name)
            .to_ascii_uppercase();
        log::info!(
            "Overwriting planet type in save file: {}",
            save_file_path.display()
        );
        overwrite_planet_type_v70(&save_file_path, &map_slot_name)?;
    }

    remove_map_archive_map_directory(&map_dir_path)?;

    registry.remove_map_entry(map_hash_id);

    save_registry(&registry)?;

    let mut restored_files = saves_files_paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<String>>();
    restored_files.push(map_file_path.to_string_lossy().to_string());

    log::info!("Restoration process completed successfully.");

    Ok(restored_files)
}

fn move_map_file_to_game_dir(
    map_file_path: &Path,
    game_dir_path: &Path,
    target_map_file_name: &str,
) -> Result<(), String> {
    let destination = game_dir_path.join(target_map_file_name);
    fs::copy(&map_file_path, &destination)
        .map_err(|e| format!("Failed to copy map file: {}", e))?;
    log::info!("Map file: {}", map_file_path.display());
    log::info!("  moved to game directory: {}", destination.display());
    Ok(())
}

fn move_saves_files_to_saves_dir(
    saves_files_paths: &Vec<PathBuf>,
    saves_dir_path: &Path,
) -> Result<(), String> {
    for save_file_path in saves_files_paths {
        let mut destination = saves_dir_path.join(save_file_path.file_name().unwrap());
        if destination.exists() {
            destination = find_next_available_save_file_path(
				&destination,
				saves_dir_path,
			);
        }
        fs::copy(&save_file_path, &destination)
            .map_err(|e| format!("Failed to copy save file: {}", e))?;
        log::info!(
            "Save file moved to saves directory: {}",
            destination.display()
        );
    }
    Ok(())
}

fn find_next_available_save_file_path(
	original_path: &Path,
	saves_dir_path: &Path,
) -> PathBuf {
	let original_dir_path = original_path.parent().unwrap();
	const FILE_NAME: &str = "SAVE";

	let mut used = std::collections::HashSet::<u8>::new();

	if let Ok(entries) = fs::read_dir(saves_dir_path) {
		for entry in entries.filter_map(Result::ok) {
			let fname = entry.file_name().to_string_lossy().to_ascii_lowercase();
			if !fname.starts_with(FILE_NAME) {
				continue;
			}
			// collect first contiguous run of digits after the "save" prefix (skip any non-digits until digits start)
			let mut digits = String::new();
			for ch in fname[FILE_NAME.len()..].chars() {
				if ch.is_ascii_digit() {
					digits.push(ch);
				} else if digits.is_empty() {
					// still haven't found digits, skip
					continue;
				} else {
					// we've collected digits and then hit a non-digit -> stop
					break;
				}
			}
			if !digits.is_empty() {
				if let Ok(n) = digits.parse::<u8>() {
					used.insert(n);
				}
			}
		}
	}

	// pick the smallest unused slot starting from 1
	let mut candidate: u16 = 1;
	loop {
		let c_u8 = if candidate > u8::MAX as u16 { u8::MAX } else { candidate as u8 };
		let new_name = match original_path.extension().and_then(|s| s.to_str()) {
			Some(ext) if !ext.is_empty() => format!("{}{}.{}", FILE_NAME, c_u8, ext),
			_ => format!("{}{}", FILE_NAME, c_u8),
		};
		let candidate_path = saves_dir_path.join(&new_name);
		if !candidate_path.exists() && !used.contains(&c_u8) {
			return candidate_path;
		}
		candidate = candidate.saturating_add(1);
		// fallback: if we've exhausted u8 range, return original with a timestamp
		if candidate == 0 {
			let ts = chrono::Utc::now().timestamp_millis();
			let new_name = match original_path.extension().and_then(|s| s.to_str()) {
				Some(ext) if !ext.is_empty() => format!("{}{}.{}", FILE_NAME, ts, ext),
				_ => format!("{}{}", FILE_NAME, ts),
			};
			return original_dir_path.join(new_name);
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::RegistryMapEntry;
    use test_utils::*;

    #[test]
    fn test_restore_files() {
        run_test!({
            // Arrange
            let test_fs = TestFileSystem::new();

            let game_dir_path = test_fs.create_test_dir("game");
            let saves_dir_path = test_fs.create_test_dir("saves");

            let archive_dir_path = test_fs.create_test_dir("archive");
            let map_file_name = "test_map.wrl";
            let map_hash_id = "00000000-TEST-MAP-HASH-000000000000";
            let archive_map_file_path = test_fs.create_test_file(
                &format!("archive/{}/{}", &map_hash_id, map_file_name),
                "Map content",
            );
            let archive_save_1_file_path = test_fs.create_test_file(
                &format!("archive/{}/save_1.dta", &map_hash_id),
                "SAVE 1 content",
            );
            let archive_save_2_file_path = test_fs.create_test_file(
                &format!("archive/{}/save_2.dta", &map_hash_id),
                "SAVE 2 content",
            );

            let archive_saves_files_paths: Vec<&Path> =
                vec![&archive_save_1_file_path, &archive_save_2_file_path];

            let mut registry = Registry::new();
            registry.set_map_entry(
                map_hash_id,
                RegistryMapEntry {
                    map: map_file_name.to_string(),
                    saves: archive_saves_files_paths
                        .iter()
                        .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
                        .collect(),
                },
            );

            let registry_path = test_fs.get_test_file_path("archive/registry.json");
            registry.save_as(&registry_path).unwrap();

            // Act
            let restored_files = restore_files(
                map_hash_id,
                &archive_dir_path,
                &game_dir_path,
                &saves_dir_path,
                "test_map.wrl".to_string(),
            );

            // Assert
            assert!(restored_files.is_ok());
            let restored_files = restored_files.unwrap();
            assert!(test_fs.file_exist(&game_dir_path.join("test_map.wrl")));
            assert!(test_fs.file_exist(&saves_dir_path.join("save_1.dta")));
            assert!(test_fs.file_exist(&saves_dir_path.join("save_2.dta")));
            assert!(!test_fs.file_exist(&archive_map_file_path));
            assert!(!test_fs.file_exist(&archive_save_1_file_path));
            assert!(!test_fs.file_exist(&archive_save_2_file_path));
            assert_eq!(restored_files.len(), 3);
            assert!(test_fs.file_has_content(&game_dir_path.join("test_map.wrl"), "Map content"));
            assert!(test_fs.file_has_content(&saves_dir_path.join("save_1.dta"), "SAVE 1 content"));
            assert!(test_fs.file_has_content(&saves_dir_path.join("save_2.dta"), "SAVE 2 content"));
            let updated_registry = Registry::from_file(&registry_path).unwrap();
            assert!(!updated_registry.has_map_entry(map_hash_id));
        });
    }
}

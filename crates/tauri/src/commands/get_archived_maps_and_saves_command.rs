use std::path::{Path, PathBuf};

use saves::{find_save_files, get_related_save_files};

use crate::{
    GLOBAL_APP_STATE,
    app_state::{MapAndSaves, MapMetadata},
};

/// Scans the game directory for WRL files and their related save files,
/// and updates the global application state with the found maps and saves.
/// Sends the updated application state to the front-end.
#[tauri::command]
pub async fn get_archived_maps_and_saves_command() -> Result<Vec<MapAndSaves>, String> {
    let app_state = GLOBAL_APP_STATE.clone();
    let archive_dir_path = app_state.archive_dir_path();
    let saves_dir_path = app_state.saves_dir_path();

    app_state.clear_archived_maps_and_saves();

    let wrl_files = find_archived_wrl_files();

    let save_files = find_save_files(&saves_dir_path).map_err(|e| {
        log::error!("Failed to find save files: {}", e);
        format!("Failed to find save files: {}", e)
    })?;

    for wrl_file in wrl_files {
        let save_files = get_related_save_files(&save_files, &wrl_file);

        app_state.add_archived_map_and_saves(wrl_file, save_files);
    }

    let result = app_state
        .get_archived_maps_and_saves()
        .maps
        .into_iter()
        .map(|(hash_id, metadata)| MapAndSaves {
            map: metadata.map.clone(),
            map_hash_id: hash_id.clone(),
            saves: {
                metadata
                    .saves
                    .into_iter()
                    .map(|save_file_name| {
                        let save_path = Path::new(&archive_dir_path)
                            .join(&hash_id)
                            .join(&save_file_name);
                        save_path.to_string_lossy().to_string()
                    })
                    .collect()
            },
        })
        .collect();

    Ok(result)
}

fn find_archived_wrl_files() -> Vec<PathBuf> {
    let app_state = GLOBAL_APP_STATE.clone();
    app_state.reload_archive_registry();

    let mut wrl_files = Vec::new();
    let archived_maps_and_saves = app_state.get_archived_maps_and_saves();

    for map_and_saves in archived_maps_and_saves.maps {
        let map_path = Path::new(&app_state.archive_dir_path())
            .join(&map_and_saves.0)
            .join(&map_and_saves.1.map);
        if map_path.exists() {
            let map_hash_id = wrl::hash_wrl_file_without_tail(&map_path).unwrap();
            app_state.set_map_metadata(
                &map_hash_id.clone(),
                MapMetadata {
                    map_hash_id: map_hash_id.to_string(),
                    file_name: map_path.file_name().unwrap().to_string_lossy().into(),
                    file_path: map_path.to_string_lossy().into(),
                    name: String::new(),
                    description: String::new(),
                    version: String::new(),
                    author: String::new(),
                    date: String::new(),
                },
            );
            wrl_files.push(map_path);
        }
    }

    wrl_files
}

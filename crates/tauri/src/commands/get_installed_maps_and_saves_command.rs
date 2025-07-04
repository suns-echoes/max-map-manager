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
pub async fn get_installed_maps_and_saves_command() -> Result<Vec<MapAndSaves>, String> {
    let app_state = GLOBAL_APP_STATE.clone();

    app_state.clear_installed_maps_and_saves();

    let game_dir_path = app_state.game_dir_path();
    let saves_dir_path = app_state.saves_dir_path();

    let wrl_files = find_wrl_files(&game_dir_path);

    let save_files = find_save_files(&saves_dir_path).map_err(|e| {
        log::error!("Failed to find save files: {}", e);
        format!("Failed to find save files: {}", e)
    })?;

    for wrl_file in wrl_files {
        let save_files = get_related_save_files(&save_files, &wrl_file);

        app_state.add_installed_map_and_saves(wrl_file, save_files);
    }

    Ok(app_state.get_installed_maps_and_saves())
}

fn find_wrl_files(game_dir_path: &Path) -> Vec<PathBuf> {
    let app_state = GLOBAL_APP_STATE.clone();

    // Scan for WRL files in the game directory
    let mut wrl_files = Vec::new();
    if let Ok(entries) = std::fs::read_dir(game_dir_path) {
        for entry in entries.flatten() {
            if !entry.path().is_file() {
                continue;
            }
            if let Some(ext) = entry.path().extension() {
                if ext.to_ascii_uppercase() == "WRL" {
                    wrl_files.push(entry.path());
                    let map_hash_id = wrl::hash_wrl_file_without_tail(&entry.path()).unwrap();
                    app_state.set_map_metadata(
                        &map_hash_id.clone(),
                        MapMetadata {
                            map_hash_id: map_hash_id.to_string(),
                            file_name: entry.path().file_name().unwrap().to_string_lossy().into(),
                            file_path: entry.path().to_string_lossy().into(),
                            name: String::new(),
                            description: String::new(),
                            version: String::new(),
                            author: String::new(),
                            date: String::new(),
                        },
                    );
                }
            }
        }
    }

    wrl_files
}

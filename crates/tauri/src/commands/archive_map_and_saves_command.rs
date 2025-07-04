use std::path::PathBuf;

use archive::archive_files;
use saves::find_related_save_files;

use crate::GLOBAL_APP_STATE;

/// Archives a map and its related save files.
/// The map is identified by its file name.
/// The map and saves are moved to the archive directory.
/// Sends the map hash ID to the front-end.
#[tauri::command]
pub async fn archive_map_and_saves_command(map_hash_id: String) -> Result<String, String> {
    let app_state = GLOBAL_APP_STATE.clone();

    let game_dir_path = app_state.game_dir_path();
    let saves_dir_path = app_state.saves_dir_path();
    let archive_dir_path = app_state.archive_dir_path();

    let map_file_path = app_state
        .get_installed_maps_and_saves()
        .iter()
        .find(|map_and_saves| map_and_saves.map_hash_id == map_hash_id)
        .map(|map_and_saves| PathBuf::from(map_and_saves.map.clone()))
        .ok_or_else(|| format!("Map with hash ID {} not found", map_hash_id))?;

    let saves_files_paths_buf =
        find_related_save_files(&map_file_path, &saves_dir_path).map_err(|e| {
            log::error!(
                "Failed to find related save files for map: {}",
                map_file_path.display()
            );
            log::error!("{}", e);
            format!("Failed to find related save files: {}", e)
            // TODO: Refine this message
        })?;

    let saves_files_paths: Vec<PathBuf> = saves_files_paths_buf
        .iter()
        .map(|path_buf| {
            let mut new_path_buf = PathBuf::new();
            new_path_buf.push(path_buf);
            new_path_buf
        })
        .collect();

    match archive_files(
        &map_hash_id,
        &map_file_path,
        &saves_files_paths,
        &archive_dir_path,
        &game_dir_path,
        &saves_dir_path,
    ) {
        Ok(_) => {
            let metadata = app_state.get_map_metadata(&map_hash_id);

            if let Some(mut map_metadata) = metadata {
                let archive_map_path = archive_dir_path
                    .join(&map_hash_id)
                    .join(&map_metadata.file_name);
                map_metadata.file_path = archive_map_path.to_string_lossy().into();
            }

            Ok(map_hash_id)
        }
        Err(e) => {
            log::error!("Archivization failed: {}", e);
            Err("Archivization failed. Check logs for more details.".into())
        }
    }
}

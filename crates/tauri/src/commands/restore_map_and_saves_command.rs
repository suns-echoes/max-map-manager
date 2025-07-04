use archive::restore_files;

use crate::GLOBAL_APP_STATE;

/// Restores a map and its related save files from the archive.
/// The map is identified by its hash ID.
/// The map and saves are restored to their original directories.
#[tauri::command]
pub async fn restore_map_and_saves_command(
    map_hash_id: String,
    target_map_file_name: String,
) -> Result<Vec<String>, String> {
    let app_state = GLOBAL_APP_STATE.clone();

    let archive_dir_path = app_state.archive_dir_path();
    let game_dir_path = app_state.game_dir_path();
    let saves_dir_path = app_state.saves_dir_path();

    match restore_files(
        &map_hash_id,
        &archive_dir_path,
        &game_dir_path,
        &saves_dir_path,
        target_map_file_name,
    ) {
        Ok(restored_paths) => Ok(restored_paths),
        Err(e) => {
            log::error!("Restoration failed: {}", e);
            Err(format!("Restoration failed: {}", e))
        }
    }
}

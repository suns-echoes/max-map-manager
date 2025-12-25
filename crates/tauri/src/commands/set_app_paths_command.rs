use std::path::PathBuf;

use crate::{
    GLOBAL_APP_STATE,
    app_settings::{self, AppSettings},
};

#[tauri::command]
pub fn set_app_paths_command(
    max_path: String,
    saves_path: String,
    archive_path: String,
) -> Result<(), String> {
    let app_state = GLOBAL_APP_STATE.clone();

    let max_path_buf = PathBuf::from(max_path);
    if !max_path_buf.exists() || !max_path_buf.is_dir() {
        let msg = format!(
            "Provided game path does not exist or is not a directory: {:?}",
            max_path_buf
        );
        log::error!("{}", msg);
        return Err(msg);
    }

    let saves_path_buf = PathBuf::from(saves_path);
    if !saves_path_buf.exists() || !saves_path_buf.is_dir() {
        let msg = format!(
            "Provided saves path does not exist or is not a directory: {:?}",
            saves_path_buf
        );
        log::error!("{}", msg);
        return Err(msg);
    }

    let archive_path_buf = PathBuf::from(archive_path);
    if !archive_path_buf.exists() || !archive_path_buf.is_dir() {
        let msg = format!(
            "Provided archive path does not exist or is not a directory: {:?}",
            archive_path_buf
        );
        log::error!("{}", msg);
        return Err(msg);
    }

    app_state.set_game_dir_path(&max_path_buf);
    app_state.set_saves_dir_path(&saves_path_buf);
    app_state.set_archive_dir_path(&archive_path_buf);

    app_settings::save_app_settings(&AppSettings {
        game_dir: max_path_buf.to_string_lossy().to_string(),
        saves_dir: saves_path_buf.to_string_lossy().to_string(),
        archive_dir: archive_path_buf.to_string_lossy().to_string(),
    })?;

    app_state.set_needs_setup(false);
    app_state.init_max_res_reader();

    Ok(())
}

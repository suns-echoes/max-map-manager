use std::path::PathBuf;

#[tauri::command]
pub fn verify_dir_path_command(path: String) -> Result<bool, String> {
    let path_buf = PathBuf::from(path);

    if !path_buf.exists() {
        let msg = format!("Provided path does not exist: {:?}", path_buf);
        log::error!("{}", msg);
        return Err(msg);
    }

    if !path_buf.is_dir() {
        let msg = format!("Provided path is not a directory: {:?}", path_buf);
        log::error!("{}", msg);
        return Err(msg);
    }

    Ok(true)
}

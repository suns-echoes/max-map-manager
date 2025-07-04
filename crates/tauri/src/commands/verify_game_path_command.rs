use std::path::PathBuf;

#[tauri::command]
pub fn verify_game_path_command(path: String) -> Result<bool, String> {
	let path_buf = PathBuf::from(path);

	if !path_buf.exists() {
		let msg = format!("Provided game path does not exist: {:?}", path_buf);
		log::error!("{}", msg);
		return Err(msg);
	}

	if !path_buf.is_dir() {
		let msg = format!("Provided game path is not a directory: {:?}", path_buf);
		log::error!("{}", msg);
		return Err(msg);
	}

	let max_ref_file_path = path_buf.join("MAX.RES");
	if !max_ref_file_path.exists() {
		let msg = format!("MAX.RES file does not exist in the provided game path: {:?}", max_ref_file_path);
		log::error!("{}", msg);
		return Err(msg);
	}

	if !max_ref_file_path.is_file() {
		let msg = format!("MAX.RES path is not a file: {:?}", max_ref_file_path);
		log::error!("{}", msg);
		return Err(msg);
	}

	Ok(true)
}

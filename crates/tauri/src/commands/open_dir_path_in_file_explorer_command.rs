use std::path::PathBuf;

use tauri::{State, Wry};
use tauri_plugin_opener::Opener;

#[tauri::command]
pub async fn open_dir_path_in_file_explorer_command(
	opener: State<'_, Opener<Wry>>,
	path: String,
) -> Result<(), String> {
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

	opener
		.open_path(path_buf.to_string_lossy(), None::<&str>)
		.map_err(|e| {
		let msg = format!(
			"Failed to open directory in file explorer: {:?}, error: {}",
			path_buf, e
		);
		log::error!("{}", msg);
		msg
	})
}

use std::collections::HashSet;

use wrl;

use crate::GLOBAL_APP_STATE;

#[tauri::command]
pub async fn install_imported_map_command(import_path: String, slot_name: String) -> Result<bool, String> {
    let app_state = GLOBAL_APP_STATE.clone();
    let max_path = app_state.game_dir_path();
	let file_path = std::path::Path::new(&import_path);

	match wrl::verify_file(&file_path) {
		Ok(is_valid) => {
			if !is_valid {
				let error_message = format!("WRL file verification failed: {}", import_path);
				log::error!("{}", error_message);
				return Err(error_message);
			}
		}
		Err(e) => {
			let error_message = format!("Error verifying WRL file: {}", import_path);
			log::error!("{}", error_message);
			log::error!("{}", e);
			return Err(error_message);
		}
	}

	let is_file_valid = wrl::verify_file(&file_path).unwrap_or(false);
	if !is_file_valid {
		let error_message = format!("Invalid WRL file: {}", import_path);
		log::error!("{}", error_message);
		return Err(error_message);
	}

	let file_hash = match wrl::file::hash_wrl_file_without_tail(&file_path) {
		Ok(hash) => hash,
		Err(_) => {
			let error_message = format!("Failed to calculate WRL file hash: {}", import_path);
			log::error!("{}", error_message);
			return Err(error_message);
		}
	};

	let archived_maps_and_saves = app_state.get_archived_maps_and_saves();
	let installed_maps_and_saves = app_state.get_installed_maps_and_saves();

	let mut known_hash_ids: HashSet<String> = archived_maps_and_saves
		.maps
		.keys()
		.cloned()
		.collect();

	known_hash_ids.extend(
		installed_maps_and_saves
			.iter()
			.map(|map_and_saves| map_and_saves.map_hash_id.clone()),
	);

	if known_hash_ids.contains(&file_hash) {
		let error_message = format!("This map is already known, and cannot be imported: {}", import_path);
		log::error!("{}", error_message);
		return Err(error_message);
	}

	match std::fs::copy(&import_path, max_path.join(format!("{}.WRL", slot_name))) {
		Ok(_) => {
			Ok(true)
		}
		Err(e) => {
			let error_message = format!("Failed to copy WRL file to game directory: {}", import_path);
			log::error!("{}", error_message);
			log::error!("{}", e);
			Err(error_message)
		}
	}
}

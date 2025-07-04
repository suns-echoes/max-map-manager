use std::path::PathBuf;

use saves::{read_save_file_metadata, structs::SaveFileMetadata};

#[tauri::command]
pub async fn read_save_files_metadata_command(
    save_file_paths: Vec<String>,
    map_width: u16,
    map_height: u16,
) -> Result<Vec<SaveFileMetadata>, String> {
    let mut files_metadata: Vec<SaveFileMetadata> = Vec::new();
    for path in save_file_paths {
        println!("Reading save file metadata for {}", path);
        match read_save_file_metadata(&PathBuf::from(&path), map_width, map_height) {
            Ok(metadata) => files_metadata.push(metadata),
            Err(e) => {
                log::error!("Failed to read save file metadata for {}: {}", path, e);
                return Err(format!(
                    "Failed to read save file metadata for {}: {}",
                    path, e
                ));
            }
        }
    }
    Ok(files_metadata)
}

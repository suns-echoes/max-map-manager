use std::path::Path;

use wrl;

use crate::GLOBAL_APP_STATE;

#[tauri::command]
pub async fn generate_bigmap_preview_command(
    map_hash_id: &str,
    scanline: bool,
) -> Result<Vec<u8>, String> {
    let app_state = GLOBAL_APP_STATE.clone();

    let map_metadata = app_state.get_map_metadata(map_hash_id).ok_or_else(|| {
        log::error!("Map metadata not found for hash ID: {}", map_hash_id);
        String::new()
    })?;

    let wrl_file_path = Path::new(&map_metadata.file_path);
    let wrl_file = wrl::read_wrl_file(&wrl_file_path).unwrap();
    let preview = wrl::generate_bigmap_preview(
        &wrl_file.bigmap,
        &wrl_file.tiles,
        &wrl_file.palette,
        wrl_file.width,
        wrl_file.height,
        256,
        256,
        scanline,
    );
    Ok(preview)
}

use std::path::Path;

use wrl;

use crate::GLOBAL_APP_STATE;

#[tauri::command]
pub fn read_archived_maps_metadata_command() -> Result<Vec<wrl::MapMetadata>, String> {
    let app_state = GLOBAL_APP_STATE.clone();
    let archive_dir_path = app_state.archive_dir_path();

    app_state.reload_archive_registry();

    let mut maps_metadata = Vec::new();
    let mut map_file_paths = Vec::new();
    let maps_and_saves = app_state.get_archived_maps_and_saves();

    for map_and_saves in maps_and_saves.maps {
        let map_path = Path::new(&archive_dir_path)
            .join(&map_and_saves.0)
            .join(&map_and_saves.1.map);
        map_file_paths.push(map_path);
    }

    for map_file_path in map_file_paths {
        let map_header = wrl::read_wrl_header(&map_file_path).map_err(|e| {
            format!(
                "Failed to read WRL header for file {}: {}",
                map_file_path.display(),
                e
            )
        })?;

        let mut map_tail = match wrl::read_wrl_tail_header(&map_file_path) {
            Ok(Some(header)) => header,
            Ok(None) => wrl::structs::WRLTailHeader::new("".to_string()),
            Err(e) => {
                log::error!(
                    "Failed to read WRL tail header for file {}: {}",
                    map_file_path.display(),
                    e
                );
                continue;
            }
        };

        if let Some(known_map) = app_state.get_known_map_info(&map_tail.hash_id) {
            map_tail.name = known_map.name;
            map_tail.description = known_map.description;
            map_tail.version = known_map.version;
            map_tail.author = known_map.author;
            map_tail.date = known_map.date;
        }

        let map_metadata = wrl::MapMetadata {
            file_path: map_file_path.to_string_lossy().to_string(),
            width: map_header.width,
            height: map_header.height,
            minimap: wrl::preview::generate_minimap_preview(
                &map_header.minimap,
                &map_header.palette,
                map_header.width,
                map_header.height,
            ),
            tail: map_tail,
        };
        maps_metadata.push(map_metadata);
    }

    log::info!("Read {} archived maps metadata", maps_metadata.len());

    Ok(maps_metadata)
}

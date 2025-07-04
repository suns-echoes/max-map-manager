use std::path::PathBuf;

use tauri::http;

use crate::{GLOBAL_APP_STATE, internal_server::create_error_response::create_error_response};

pub fn get_wrl_minimap_request(url_parts: Vec<&str>) -> http::Response<Vec<u8>> {
    let map_hash_id = url_parts[2];

    let map_metadata = GLOBAL_APP_STATE.get_map_metadata(map_hash_id);

    if map_metadata.is_none() {
        return create_error_response(404, "Map metadata not found");
    };

    let map_metadata = map_metadata.unwrap();
    let map_file_path = PathBuf::from(&map_metadata.file_path);
    let map_header = wrl::read_wrl_header(&map_file_path).map_err(|e| {
        format!(
            "Failed to read WRL header for file {}: {}",
            map_file_path.display(),
            e
        )
    });

    if map_header.is_err() {
        return create_error_response(500, &map_header.err().unwrap());
    }

    let map_header = map_header.unwrap();
    let width = map_header.width;
    let height = map_header.height;

    let minimap_pixels =
        wrl::generate_minimap_preview(&map_header.minimap, &map_header.palette, width, height);

    if minimap_pixels.len() == 0 {
        return create_error_response(500, "Minimap pixel buffer is empty");
    }

    let image_data = bmp::rgba_to_bmp32(&minimap_pixels, width as u32, height as u32)
        .expect("Failed to generate BMP image");

    return http::Response::builder()
        .header("Content-Type", "image/bmp")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        )
        .body(image_data)
        .expect("Failed to build response");
}

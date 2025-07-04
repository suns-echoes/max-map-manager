use tauri::http;

use crate::internal_server::{
    create_error_response::create_error_response, generate_bigmap_preview::generate_bigmap_preview,
};

pub fn get_wrl_bigmap_request(url_parts: Vec<&str>, _path: &str) -> http::Response<Vec<u8>> {
    let map_hash_id = url_parts[2];
    let mut size: usize = 256;

    if url_parts.len() >= 4 {
        match url_parts[3].parse::<usize>() {
            Ok(s) => {
                size = s;
            }
            _ => {
                return create_error_response(400, "Invalid size parameter");
            }
        }
    }

    let minimap_pixels = generate_bigmap_preview(map_hash_id, size, false).unwrap_or_else(|e| {
        log::error!(
            "Failed to generate bigmap preview for map {}: {}",
            map_hash_id,
            e
        );
        Vec::new()
    });

    if minimap_pixels.len() == 0 {
        return create_error_response(500, "Minimap pixel buffer is empty");
    }

    let image_data = bmp::rgba_to_bmp32(&minimap_pixels, size as u32, size as u32)
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

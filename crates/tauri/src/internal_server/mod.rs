use tauri::http;

use crate::{
    GLOBAL_APP_STATE,
    internal_server::{
        create_error_response::create_error_response, request_handlers::{get_file_request, get_res_image_request, get_wrl_bigmap_request, get_wrl_minimap_request},
    },
};

mod create_error_response;
mod generate_bigmap_preview;
mod lre;
mod request_handlers;

pub fn handle_request(request: http::Request<Vec<u8>>) -> http::Response<Vec<u8>> {
    let app_state = GLOBAL_APP_STATE.clone();

    // example: /get-file/FILE_TAG?mime/type
    let url = request.uri();
    let path = url.to_string();
    let url_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if url_parts.len() < 2 {
        return http::Response::builder()
            .status(400)
            .header("Content-Type", "text/plain")
            .header("Access-Control-Allow-Origin", "*")
            .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
            .header(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            )
            .body(Vec::new())
            .expect("Failed to build 400 response");
    }

    let asset_type = url_parts[1];

    if asset_type == "get-file" {
        return get_file_request(url_parts, &path);
    }

    if app_state.needs_setup() {
        return create_error_response(503, "Application setup is required");
    }

	if asset_type == "get-res-image" {
		return get_res_image_request(url_parts);
    } else if asset_type == "get-wrl-minimap" {
		return get_wrl_minimap_request(url_parts);
    } else if asset_type == "get-wrl-bigmap" {
		return get_wrl_bigmap_request(url_parts, &path);
	} else {
		return create_error_response(500, "Unknown asset type");
	}
}

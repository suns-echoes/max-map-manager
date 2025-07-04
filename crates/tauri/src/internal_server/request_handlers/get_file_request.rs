use std::path::Path;

use tauri::http;

use crate::{GLOBAL_APP_STATE, internal_server::create_error_response::create_error_response};

pub fn get_file_request(url_parts: Vec<&str>, path: &str) -> http::Response<Vec<u8>> {
	let mmm_res_reader = GLOBAL_APP_STATE.get_mmm_res_reader().unwrap();
	let file_tag_and_format = url_parts[2].split('?').collect::<Vec<&str>>();
	let file_tag = file_tag_and_format[0];
	let mime_type = path.split('?').nth(1).unwrap_or("plain/text");

	let file_path_in_resource_dir = format!("resources/{}", file_tag);
	if Path::new(&file_path_in_resource_dir).exists() {
		if let Ok(file_data) = std::fs::read(&file_path_in_resource_dir) {
			return http::Response::builder()
				.header("Content-Type", mime_type)
				.header("Access-Control-Allow-Origin", "*")
				.header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
				.header(
					"Access-Control-Allow-Headers",
					"Content-Type, Authorization",
				)
				.body(file_data)
				.expect("Failed to build response");
		}
	}

	let file_data = match mmm_res_reader.read_file(file_tag) {
		Some(data) => data,
		None => {
			return create_error_response(
				404,
				&format!("File \"{}\" not found in MMM.RES", file_tag),
			);
		}
	};

	return http::Response::builder()
		.header("Content-Type", mime_type)
		.header("Access-Control-Allow-Origin", "*")
		.header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
		.header(
			"Access-Control-Allow-Headers",
			"Content-Type, Authorization",
		)
		.body(file_data)
		.expect("Failed to build response");
}

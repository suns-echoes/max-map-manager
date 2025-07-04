use tauri::http;

pub fn create_error_response(status: u16, debug_info: &str) -> http::Response<Vec<u8>> {
    let error_message = format!("Failed to build {} response: {}", status, debug_info);
    http::Response::builder()
        .header("Content-Type", "text/plain")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        )
        .body(error_message.clone().into_bytes())
        .expect(&error_message.as_str())
}

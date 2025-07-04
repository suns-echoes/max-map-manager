use tauri::http;

use res_reader::ImageData;

use crate::{
    GLOBAL_APP_STATE,
    internal_server::{create_error_response::create_error_response, lre},
};

pub fn get_res_image_request(url_parts: Vec<&str>) -> http::Response<Vec<u8>> {
    let max_res_reader = GLOBAL_APP_STATE.get_max_res_reader().unwrap();
    let tag = url_parts[2];
    let mut mask_tag = None;

    if url_parts.len() == 4 {
        mask_tag = Some(url_parts[3]);
    }

    let image_data: Result<ImageData, String> =
        if let Some(mut image_data) = max_res_reader.read_image(tag) {
            if let Some(mask_tag) = mask_tag {
                let mmm_res_reader = GLOBAL_APP_STATE.get_mmm_res_reader().unwrap();
                if let Some(mask_image_data) = mmm_res_reader.read_file(mask_tag) {
                    let mask_image_data = lre::decompress_rgba_pixels(&mask_image_data)
                        .expect("Failed to decode LRE mask image data");
                    image_data.data = bmp::xor_rgba_pixel_data(&image_data.data, &mask_image_data)
                        .expect("Failed to apply mask to image");
                    Ok(image_data)
                } else {
                    Err(format!("Mask image \"{}\" not found in mmm.res", mask_tag))
                }
            } else {
                Ok(image_data)
            }
        } else {
            Err(format!("Image \"{}\" not found in MAX.RES", tag))
        };

    if image_data.is_err() {
        return create_error_response(404, &image_data.err().unwrap());
    }

    let image_data = image_data.unwrap();

    let bmp_image_buffer = bmp::rgba_to_bmp32(
        &image_data.data,
        image_data.width as u32,
        image_data.height as u32,
    )
    .expect("Failed to generate BMP image");

    return http::Response::builder()
        .header("Content-Type", "image/bmp")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        .header(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        )
        .body(bmp_image_buffer)
        .expect("Failed to build response");
}

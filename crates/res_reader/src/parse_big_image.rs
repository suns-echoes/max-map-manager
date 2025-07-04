use crate::{ImageData, MAX_IMAGE_HEIGHT, MAX_IMAGE_WIDTH, color_utils::rgb_to_bgra};

pub fn parse_big_image(data: &[u8]) -> Option<ImageData> {
    if data.len() < 8 {
        return None;
    }

    let width = i16::from_le_bytes(data[4..6].try_into().ok()?);
    let height = i16::from_le_bytes(data[6..8].try_into().ok()?);

    let palette_size = 256 * 3;

    if width <= 0 || height <= 0 || width > MAX_IMAGE_WIDTH || height > MAX_IMAGE_HEIGHT {
        return None;
    }

    let palette = rgb_to_bgra(&mut data[8..8 + palette_size].to_owned());

    let indexed_image_data = image_rle_decode(&data[8 + palette_size..]).map_or_else(
        |e| {
            eprintln!("Failed to decode RLE data: {}", e);
            None
        },
        |data| Some(data),
    )?;

    let mut image_data_index = 0;
    let mut image_data = vec![0; (width as i32 * height as i32 * 4) as usize];
    for &palette_color_index in indexed_image_data.iter() {
        let palette_slice =
            &palette[palette_color_index as usize * 4..palette_color_index as usize * 4 + 4];
        let image_data_slice = &mut image_data[image_data_index..image_data_index + 4];
        image_data_slice.copy_from_slice(&palette_slice);
        image_data_index += 4;
    }

    Some(ImageData {
        width: width as u32,
        height: height as u32,
        data: image_data,
    })
}

pub fn image_rle_decode(data: &[u8]) -> Result<Vec<u8>, String> {
    if data.is_empty() {
        return Ok(Vec::new());
    }

    let mut decoded_data = Vec::new();
    let mut i = 0;

    while i < data.len() {
        let option: i16 =
            i16::from_le_bytes(data[i..i + 2].try_into().map_err(|_| "Invalid RLE data")?);
        i += 2;

        if option > 0 {
            let count = option as usize;
            decoded_data.extend_from_slice(&data[i..i + count as usize]);
            i += count as usize;
        } else {
            let count = (-option) as usize;
            let value = data[i];
            i += 1;

            decoded_data.extend(std::iter::repeat(value).take(count));
        }
    }

    Ok(decoded_data)
}

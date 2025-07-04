use crate::{
    ImageData, MAX_IMAGE_HEIGHT, MAX_IMAGE_WIDTH, color_utils::indexed_to_color,
    palette::FRAMEPIC_PALETTE_BGRA,
};

pub fn parse_simple_image(data: &[u8]) -> Option<ImageData> {
    if data.len() < 8 {
        return None;
    }

    let width = i16::from_le_bytes(data[0..2].try_into().ok()?);
    let height = i16::from_le_bytes(data[2..4].try_into().ok()?);

    if width <= 0
        || height <= 0
        || width > MAX_IMAGE_WIDTH
        || height > MAX_IMAGE_HEIGHT
        || data.len() - 8 != (width as usize * height as usize)
    {
        return None;
    }

    let indexed_image_data = data[8..].to_vec();

    let image_data = indexed_to_color(&indexed_image_data, &FRAMEPIC_PALETTE_BGRA);

    Some(ImageData {
        width: width as u32,
        height: height as u32,
        data: image_data,
    })
}

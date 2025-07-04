use std::collections::HashMap;

// --- CONSTANTS ---
// New format: ONLY 2 (ColorCount u16) bytes in the header
const LRE_HEADER_SIZE: usize = 2;
const _MAX_COLORS: usize = 65536; // u16 limit

// --- STRUCT DEFINITIONS for internal data handling ---

/// Internal structure representing the compressed data before final serialization.
struct _LreCompressed {
    color_count: u16,
    palette: Vec<u8>,
    // Stored as [count_0, index_0, count_1, index_1, ...]
    pixels: Vec<u16>,
}

/// Internal structure representing the parsed binary data before decompression.
struct LreParsed {
    color_count: u16,
    palette: Vec<u8>,
    // Stored as [count_0, index_0, count_1, index_1, ...]
    pixels: Vec<u16>,
}

// --- LRE COMPRESSION / DECOMPRESSION CORE ---

/// Performs Palette generation, Indexing, and LRE compression on raw RGBA data.
///
/// Takes raw RGBA data (Vec<u8>) and returns the final LRE binary buffer (Vec<u8>).
pub fn _compress_rgba_pixels(rgba_data: Vec<u8>) -> Result<Vec<u8>, String> {
    if rgba_data.len() % 4 != 0 {
        return Err("Input data length must be a multiple of 4 (RGBA format).".to_string());
    }

    let mut palette_map: HashMap<Vec<u8>, u16> = HashMap::new();
    let mut palette_array: Vec<Vec<u8>> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    // 1. Generate Palette and Indices
    for chunk in rgba_data.chunks_exact(4) {
        let color_key = chunk.to_vec();

        let index = *palette_map.entry(color_key.clone()).or_insert_with(|| {
            let next_index = palette_array.len();
            if next_index >= _MAX_COLORS {
                panic!("Color count exceeded the u16 limit ({})", _MAX_COLORS);
            }
            palette_array.push(color_key);
            next_index as u16
        });

        indices.push(index);
    }

    if palette_array.len() > _MAX_COLORS {
        return Err(format!(
            "Color count exceeded the u16 limit ({} colors).",
            _MAX_COLORS
        ));
    }

    // 2. Perform RLE on Indices
    let mut compressed_pixels: Vec<u16> = Vec::new();

    if indices.is_empty() {
        return Ok(Vec::new());
    }

    let mut current_index = indices[0];
    let mut current_count: u8 = 1;

    for &next_index in indices.iter().skip(1) {
        if next_index == current_index && current_count < 255 {
            current_count += 1;
        } else {
            compressed_pixels.push(current_count as u16);
            compressed_pixels.push(current_index);

            current_index = next_index;
            current_count = 1;
        }
    }

    compressed_pixels.push(current_count as u16);
    compressed_pixels.push(current_index);

    // 3. Flatten Palette
    let flattened_palette = palette_array.into_iter().flatten().collect::<Vec<u8>>();

    let compressed_data = _LreCompressed {
        color_count: palette_map.len() as u16,
        palette: flattened_palette,
        pixels: compressed_pixels,
    };

    // 4. Serialize to Binary Buffer
    _serialize_lre(compressed_data)
}

/// Reconstructs the raw RGBA data from the LRE binary format.
///
/// Takes the LRE binary buffer (Vec<u8>) and returns the raw RGBA data (Vec<u8>).
pub fn decompress_rgba_pixels(data: &[u8]) -> Result<Vec<u8>, String> {
    // 1. Parse Binary Buffer to Object Structure
    let parsed_object = parse_lre_binary(data)?;

    if (parsed_object.color_count as usize) * 4 != parsed_object.palette.len() {
        return Err(format!(
            "Palette size validation failed. Expected {} bytes, got {}.",
            (parsed_object.color_count as usize) * 4,
            parsed_object.palette.len()
        ));
    }
    if parsed_object.pixels.len() % 2 != 0 {
        return Err("LRE pixel stream must contain pairs of (count, index).".to_string());
    }

    // 2. Decompress Pixels
    let mut decompressed_rgba: Vec<u8> = Vec::new();
    let color_count = parsed_object.color_count as u16;

    let mut pixel_iterator = parsed_object.pixels.into_iter();
    while let (Some(run_length_u16), Some(index_u16)) =
        (pixel_iterator.next(), pixel_iterator.next())
    {
        let run_length = run_length_u16 as usize;
        let index = index_u16 as usize;

        if index as u16 >= color_count {
            return Err(format!(
                "Index {} out of bounds for color count {}.",
                index, color_count
            ));
        }

        let palette_start = index * 4;
        let color_bytes = &parsed_object.palette[palette_start..palette_start + 4];

        for _ in 0..run_length {
            decompressed_rgba.extend_from_slice(color_bytes);
        }
    }

    Ok(decompressed_rgba)
}

// --- LRE BINARY SERIALIZATION / DESERIALIZATION ---

/// Serializes the compressed LRE object into a single binary buffer.
fn _serialize_lre(data: _LreCompressed) -> Result<Vec<u8>, String> {
    let palette_size = data.palette.len();
    let run_count = data.pixels.len() / 2;
    // Each run is 3 bytes: u8 count + u16 index (LITTLE ENDIAN)
    let actual_pixels_size = run_count * 3;

    let total_size = LRE_HEADER_SIZE + palette_size + actual_pixels_size;
    let mut buffer = Vec::with_capacity(total_size);

    // 1. Write Color Count (u16, Little Endian) - 2 BYTES
    buffer.extend_from_slice(&data.color_count.to_le_bytes());

    // 2. Write Palette Data
    buffer.extend_from_slice(&data.palette);

    // 3. Write Pixels Data (u8 count, u16 index, Little Endian for index)
    let mut pixel_iterator = data.pixels.into_iter();
    while let (Some(count_u16), Some(index_u16)) = (pixel_iterator.next(), pixel_iterator.next()) {
        let count = count_u16 as u8;
        let index = index_u16 as u16;

        buffer.push(count);
        buffer.extend_from_slice(&index.to_le_bytes());
    }

    Ok(buffer)
}

/// Parses a binary buffer back into the LRE compressed object structure.
fn parse_lre_binary(buffer: &[u8]) -> Result<LreParsed, String> {
    if buffer.len() < LRE_HEADER_SIZE {
        return Err(format!(
            "File is too small. Minimum header size is {} bytes.",
            LRE_HEADER_SIZE
        ));
    }

    let mut read_offset = 0;

    // 1. Read Color Count (u16, Little Endian)
    let color_count_bytes: [u8; 2] = buffer[read_offset..read_offset + 2]
        .try_into()
        .map_err(|_| "Failed to read color count u16.".to_string())?;
    let color_count = u16::from_le_bytes(color_count_bytes);
    read_offset += 2;

    // Calculate Palette Size implicitly: 4 bytes per color
    let palette_size = (color_count as usize) * 4;

    // Calculate Pixels Size: total file size minus header and palette
    let pixels_size = buffer.len() - LRE_HEADER_SIZE - palette_size;

    // Size validation
    if palette_size == 0 && color_count > 0 {
        return Err("Invalid color count. Calculated palette size is negative.".to_string());
    }
    if pixels_size >= buffer.len() {
        return Err(format!(
            "File size too small for declared color count. Expected minimum {} + 1 bytes, got {}.",
            LRE_HEADER_SIZE + palette_size,
            buffer.len()
        ));
    }
    if pixels_size % 3 != 0 {
        return Err(
            "LRE pixel stream size must be a multiple of 3 for (u8 count, u16 index) format."
                .to_string(),
        );
    }

    // 2. Read Palette Data (Raw bytes)
    let palette = buffer[read_offset..read_offset + palette_size].to_vec();
    read_offset += palette_size;

    // 3. Read Pixels Data (Convert from 3-byte runs to [count, index] pairs)
    let mut pixels: Vec<u16> = Vec::new();
    let pixel_data_end = read_offset + pixels_size;

    while read_offset < pixel_data_end {
        let count = buffer[read_offset];
        read_offset += 1;

        let index_bytes: [u8; 2] = buffer[read_offset..read_offset + 2]
            .try_into()
            .map_err(|_| "Failed to read index u16.".to_string())?;
        let index = u16::from_le_bytes(index_bytes);

        read_offset += 2;

        pixels.push(count as u16);
        pixels.push(index);
    }

    Ok(LreParsed {
        color_count,
        palette,
        pixels,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lre_compression_decompression() {
        let original_rgba: Vec<u8> = vec![
            255, 0, 0, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0,
            255, 255,
        ];
        let compressed = _compress_rgba_pixels(original_rgba.clone()).expect("Compression failed");
        let decompressed = decompress_rgba_pixels(&compressed).expect("Decompression failed");
        assert_eq!(original_rgba, decompressed);
    }
}

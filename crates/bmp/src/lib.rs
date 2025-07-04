/// Converts a contiguous RGBA8888 buffer into a complete, uncompressed 32-bit (32bpp)
/// bottom-up BMP file format buffer (BGRA).
///
/// The BMP format requires:
/// 1. Headers (File Header + BITMAPINFOHEADER).
/// 2. Pixel data stored in BGRA order (R and B swapped from RGBA).
/// 3. Pixel data rows stored in reverse order (bottom-up).
///
/// # Arguments
/// * `rgba_data` - A slice containing the raw pixel data in RGBA format (4 bytes per pixel).
/// * `width` - The width of the image in pixels.
/// * `height` - The height of the image in pixels.
///
/// # Returns
/// A `Result` containing the complete BMP file buffer as `Vec<u8>`, or an error string
/// if the input data size is incorrect.
pub fn rgba_to_bmp32(rgba_data: &[u8], width: u32, height: u32) -> Result<Vec<u8>, String> {
    let expected_size = (width as usize) * (height as usize) * 4;
    if rgba_data.len() != expected_size {
        return Err(format!("Input data size mismatch. Expected {} bytes for {}x{} RGBA, found {} bytes.",
            expected_size, width, height, rgba_data.len()));
    }

    const FILE_HEADER_SIZE: u32 = 14;
    const INFO_HEADER_SIZE: u32 = 40;
    const HEADER_SIZE: u32 = FILE_HEADER_SIZE + INFO_HEADER_SIZE; // 54
    const BITS_PER_PIXEL: u16 = 32;

    let pixel_data_size: u32 = width * height * 4;
    let file_size: u32 = HEADER_SIZE + pixel_data_size;

    let mut bmp_buffer = Vec::with_capacity(file_size as usize);

    // --- Write BMP File Header (14 bytes) ---

    // bfType: "BM" (0x424D)
    bmp_buffer.extend_from_slice(&[0x42, 0x4D]);

    // bfSize: Total file size (Little Endian)
    bmp_buffer.extend_from_slice(&file_size.to_le_bytes());

    // bfReserved1 & bfReserved2: 0 (2+2 bytes)
    bmp_buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    // bfOffBits: Offset to start of pixel data (54)
    bmp_buffer.extend_from_slice(&HEADER_SIZE.to_le_bytes());


    // --- Write DIB Header (BITMAPINFOHEADER - 40 bytes) ---

    // biSize: Size of the DIB header (40)
    bmp_buffer.extend_from_slice(&INFO_HEADER_SIZE.to_le_bytes());

    // biWidth: Image width
    bmp_buffer.extend_from_slice(&width.to_le_bytes());

    // biHeight: Image height (Positive for bottom-up BMP)
    bmp_buffer.extend_from_slice(&height.to_le_bytes());

    // biPlanes: 1
    bmp_buffer.extend_from_slice(&1u16.to_le_bytes());

    // biBitCount: 32 bpp
    bmp_buffer.extend_from_slice(&BITS_PER_PIXEL.to_le_bytes());

    // biCompression: 0 (BI_RGB - uncompressed)
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());

    // biSizeImage: 0 (can be 0 for BI_RGB)
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());

    // biXPelsPerMeter & biYPelsPerMeter: 0
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());

    // biClrUsed & biClrImportant: 0
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());
    bmp_buffer.extend_from_slice(&0u32.to_le_bytes());

    let row_size_bytes = width as usize * 4;

    for y in (0..height).rev() {
        let row_start_index = y as usize * row_size_bytes;
        let row_end_index = row_start_index + row_size_bytes;

        let row_slice = &rgba_data[row_start_index..row_end_index];

        for chunk in row_slice.chunks_exact(4) {
            let r = chunk[0];
            let g = chunk[1];
            let b = chunk[2];
            let a = chunk[3];

            bmp_buffer.push(b);
            bmp_buffer.push(g);
            bmp_buffer.push(r);
            bmp_buffer.push(a);
        }
    }

    Ok(bmp_buffer)
}

pub fn xor_rgba_pixel_data(target_rgba: &[u8], mask_rgba: &[u8]) -> Result<Vec<u8>, String> {
	if target_rgba.len() != mask_rgba.len() {
		return Err("Target and mask RGBA data must be of the same length.".to_string());
	}

	let mut result = Vec::with_capacity(target_rgba.len());
	for i in 0..target_rgba.len() {
		result.push(target_rgba[i] ^ mask_rgba[i]);
	}

	Ok(result)
}

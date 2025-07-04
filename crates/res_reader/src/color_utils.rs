pub fn rgb_to_bgra(rgb_pixels: &mut [u8]) -> Vec<u8> {
    let mut bgra_pixels = Vec::with_capacity(rgb_pixels.len());
    for chunk in rgb_pixels.chunks_exact(3) {
        bgra_pixels.push(chunk[0]);
        bgra_pixels.push(chunk[1]);
        bgra_pixels.push(chunk[2]);
        bgra_pixels.push(255);
    }
    bgra_pixels
}


pub fn indexed_to_color(
	indexed_pixels: &[u8],
	palette: &[u8],
) -> Vec<u8> {
	let mut color_pixels = Vec::with_capacity(indexed_pixels.len() * 4);
	for &index in indexed_pixels {
		let palette_slice = &palette[index as usize * 4..index as usize * 4 + 4];
		color_pixels.extend_from_slice(palette_slice);
	}
	color_pixels
}

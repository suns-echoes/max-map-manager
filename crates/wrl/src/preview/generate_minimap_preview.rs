/// Generate an RGBA pixel buffer for the minimap preview.
/// The width and height are in map cells, not pixels.
pub fn generate_minimap_preview(
	minimap: &[u8],
	palette: &[u8],
	width: u16,
	height: u16,
) -> Vec<u8> {
	let mut preview = Vec::with_capacity(width as usize * height as usize * 4);
	for &color_index in minimap {
		let base_index = (color_index as usize) * 3;
		preview.push(palette[base_index]);
		preview.push(palette[base_index + 1]);
		preview.push(palette[base_index + 2]);
		preview.push(255);
	}
	preview
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_generate_minimap_preview() {
		run_test!({
			// Arrange
			let minimap = vec![0, 1, 2, 3];
			let palette = vec![
				0, 0, 0,
				255, 0, 0,
				0, 255, 0,
				0, 0, 255,
			];
			let width = 2;
			let height = 2;

			// Act
			let preview = generate_minimap_preview(&minimap, &palette, width, height);

			// Assert
			assert_eq!(preview.len(), (width * height * 4) as usize);
			assert_eq!(preview[0..4], [0, 0, 0, 255]);
			assert_eq!(preview[4..8], [255, 0, 0, 255]);
			assert_eq!(preview[8..12], [0, 255, 0, 255]);
			assert_eq!(preview[12..16], [0, 0, 255, 255]);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}
}

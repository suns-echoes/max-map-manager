use std::path::Path;
use image::{ImageBuffer, Rgba};

use crate::read_wrl_header::read_wrl_header;
use crate::generate_minimap_preview::generate_minimap_preview;


/// Read WRL file and save the preview images as PNG file.
pub fn save_minimap_to_file(
	wrl_file: &Path,
	png_output_file: &Path,
) -> Result<(), ()> {
	let header = match read_wrl_header(wrl_file) {
		Ok(header) => header,
		Err(_) => {
			log::error!("failed to read WRL header for file {}", wrl_file.display());
			return Err(());
		}
	};

	let minimap = header.minimap;
	let palette = header.palette;
	let width = header.width;
	let height = header.height;

	let preview = generate_minimap_preview(&minimap, &palette, width, height);

	let img: ImageBuffer<Rgba<u8>, Vec<u8>> = match ImageBuffer::from_raw(width as u32, height as u32, preview) {
		Some(img) => img,
		None => {
			log::error!("failed to create image buffer");
			return Err(());
		}
	};

	match img.save_with_format(png_output_file, image::ImageFormat::Png) {
		Ok(_) => Ok(()),
		Err(e) => {
			log::error!("failed to save image for file {}: {}", wrl_file.display(), e);
			Err(())
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::File;
	use std::io::Read;
	use std::path::PathBuf;

	use crate::run_test;

	#[test]
	fn test_save_minimap_to_file() {
		run_test!({
			// Arrange
			let ref_file = PathBuf::from("test_files/reference/ref_minimap_preview.png");
			let wrl_file = PathBuf::from("test_files/reference/REF.WRL");
			let png_output_file = PathBuf::from("test_files/temp/minimap_preview.png");

			// Act
			let result = save_minimap_to_file(&wrl_file, &png_output_file);

			// Assert
			assert!(result.is_ok());

			// Verify the output file matches the reference file
			let mut ref_data = Vec::new();
			let mut output_data = Vec::new();
			File::open(&ref_file).unwrap().read_to_end(&mut ref_data).unwrap();
			File::open(&png_output_file).unwrap().read_to_end(&mut output_data).unwrap();
			assert_eq!(ref_data, output_data);

			let logs = get_captured_logs();
			assert!(logs.is_empty());

			// Clean up the output file
			if let Err(e) = std::fs::remove_file(&png_output_file) {
				log::error!("Failed to remove output file after test: {}", e);
			}
		});
	}
}

use crate::{TILE_DATA_SIZE, TILE_SIZE};


/// Generate an RGBA pixel buffer for the bigmap preview scaled down to given size.
/// The width and height are in map cells, not pixels.
pub fn generate_bigmap_preview(
	bigmap: &[u16],
	tiles: &[u8],
	palette: &[u8],
	width: u16,
	height: u16,
	target_pixel_width: usize,
	target_pixel_height: usize,
	scanline: bool,
) -> Vec<u8> {
	let mut preview = vec![0u8; target_pixel_width * target_pixel_height * 4];

	let mut step = 1;

	if target_pixel_width < (TILE_SIZE * width as usize) / 16 {
		step = 16;
	} else if target_pixel_width < (TILE_SIZE * width as usize) / 8 {
		step = 8;
	} else if target_pixel_width < (TILE_SIZE * width as usize) / 4 {
		step = 4;
	} else if target_pixel_width < (TILE_SIZE * width as usize) / 2 {
		step = 2;
	}

	let map_pixel_width = width as usize * TILE_SIZE;
	let map_pixel_height = height as usize * TILE_SIZE;

    for y in 0..target_pixel_height {
        for x in 0..target_pixel_width {
            let src_x_start = x * map_pixel_width / target_pixel_width;
            let src_x_end = ((x + 1) * map_pixel_width / target_pixel_width).min(map_pixel_width - 1);
            let src_y_start = y * map_pixel_height / target_pixel_height;
            let src_y_end = ((y + 1) * map_pixel_width / target_pixel_height).min(map_pixel_height - 1);

            let color = interpolate_pixel_color(
                bigmap,
                tiles,
                palette,
                src_x_start,
                src_x_end,
                src_y_start,
                src_y_end,
                width as usize,
                height as usize,
				step,
            );

            let index = (y * target_pixel_width + x) * 4;
			if scanline && (y % 2 == 1) {
				preview[index + 0] = (color[0] as f32 * 0.5) as u8;
				preview[index + 1] = (color[1] as f32 * 0.5) as u8;
				preview[index + 2] = (color[2] as f32 * 0.5) as u8;
			} else {
				preview[index + 0] = color[0];
				preview[index + 1] = color[1];
				preview[index + 2] = color[2];
			}
            preview[index + 3] = 255;
        }
    }

	preview
}

/// Interpolate pixel color for a given region in the bigmap.
/// The width and height are in map cells, not pixels.
fn interpolate_pixel_color(
    bigmap: &[u16],
    tiles: &[u8],
    palette: &[u8],
    start_x: usize,
    end_x: usize,
    start_y: usize,
    end_y: usize,
    width: usize,
    height: usize,
    step: usize,
) -> Vec<u8> {
    let mut color = [0u32; 3];
    let mut pixels_interpolated = 0u32;

    let palette_len_minus_3 = palette.len().saturating_sub(3);

    // Calculate bounds for the bigmap tile loop
    let bigmap_start_tile_x = start_x / TILE_SIZE;
    let bigmap_end_tile_x = end_x / TILE_SIZE;
    let bigmap_start_tile_y = start_y / TILE_SIZE;
    let bigmap_end_tile_y = end_y / TILE_SIZE;

    // Iterate over the *tiles* that cover the interpolation region
    for bigmap_tile_y in bigmap_start_tile_y..=(bigmap_end_tile_y.min(height.saturating_sub(1))) {
        // Pre-calculate common offsets for this tile row
        let bigmap_y_offset = bigmap_tile_y * width;

        // Calculate the actual pixel y-range within this specific tile
        let current_tile_pixel_y_start = start_y.max(bigmap_tile_y * TILE_SIZE);
        let current_tile_pixel_y_end = end_y.min((bigmap_tile_y + 1) * TILE_SIZE - 1);

        for bigmap_tile_x in bigmap_start_tile_x..=(bigmap_end_tile_x.min(width.saturating_sub(1))) {
            // Get tile_id once per tile
            let bigmap_index = bigmap_y_offset + bigmap_tile_x;
            if bigmap_index >= bigmap.len() {
                continue; // Should ideally not happen if outer loop bounds are correct
            }
            let tile_id = bigmap[bigmap_index] as usize;

            // Pre-calculate base offset for tile data
            let tile_base_data_offset = tile_id * TILE_DATA_SIZE;

            // Calculate the actual pixel x-range within this specific tile
            let current_tile_pixel_x_start = start_x.max(bigmap_tile_x * TILE_SIZE);
            let current_tile_pixel_x_end = end_x.min((bigmap_tile_x + 1) * TILE_SIZE - 1);

            // Now, iterate only over the relevant pixels *within this tile*
            for y_in_tile_pixel in (current_tile_pixel_y_start..=current_tile_pixel_y_end).step_by(step) {
                let tile_y_in_tile = y_in_tile_pixel % TILE_SIZE;
                let tile_row_offset = tile_base_data_offset + tile_y_in_tile * TILE_SIZE; // Pre-calculate row offset

                for x_in_tile_pixel in (current_tile_pixel_x_start..=current_tile_pixel_x_end).step_by(step) {
                    let tile_x_in_tile = x_in_tile_pixel % TILE_SIZE;

                    let tile_data_offset = tile_row_offset + tile_x_in_tile;

                    // Bounds check for tiles array
                    if tile_data_offset >= tiles.len() {
                        continue;
                    }

                    let tile_value = tiles[tile_data_offset];

                    let palette_start_index = (tile_value as usize) * 3;
                    if palette_start_index > palette_len_minus_3 {
                        continue;
                    }

                    color[0] += palette[palette_start_index] as u32;
                    color[1] += palette[palette_start_index + 1] as u32;
                    color[2] += palette[palette_start_index + 2] as u32;
                    pixels_interpolated += 1;
                }
            }
        }
    }

    let mut final_color = Vec::with_capacity(3);
    if pixels_interpolated > 0 {
        final_color.push((color[0] / pixels_interpolated) as u8);
        final_color.push((color[1] / pixels_interpolated) as u8);
        final_color.push((color[2] / pixels_interpolated) as u8);
    } else {
        final_color.extend_from_slice(&[0, 0, 0]);
    }
    final_color
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::File;
	use std::io::Read;
	use std::path::PathBuf;

	use crate::run_test;

	use crate::read_wrl_file;

	#[test]
	fn test_interpolate_pixel_color() {
		run_test!({
			// Arrange
			let mut preview_ref = Vec::new();
			let mut file = File::open("test_files/reference/ref_bigmap_preview.rgba").unwrap();
			file.read_to_end(&mut preview_ref).unwrap();

			let path = PathBuf::from("test_files/reference/REF.WRL");
			let wrl = read_wrl_file(&path).unwrap();

			// Act
			let preview = generate_bigmap_preview(
				&wrl.bigmap,
				&wrl.tiles,
				&wrl.palette,
				wrl.width,
				wrl.height,
				128,
				128,
				false,
			);

			// Assert
			assert_eq!(preview, preview_ref);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}
}

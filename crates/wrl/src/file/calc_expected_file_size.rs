use crate::consts::TILE_DATA_SIZE;

/// Calculate expected file size based on width, height, and tile count.
pub fn calc_expected_file_size(width: u16, height: u16, tile_count: u16) -> u64 {
	let header_size = 5u64;
	let width_size = 2u64;
	let height_size = 2u64;
	let minimap_size = width as u64 * height as u64;
	let bigmap_size = minimap_size * 2u64;
	let tile_count_size = 2u64;
	let tiles_size = tile_count as u64 * TILE_DATA_SIZE as u64;
	let palette_size = 256u64 * 3u64;
	let pass_table_size = tile_count as u64;

	let total_length = header_size + width_size + height_size +
		minimap_size + bigmap_size + tile_count_size + tiles_size +
		palette_size + pass_table_size;

	total_length
}


#[cfg(test)]
mod tests {
	use super::*;

	use crate::run_test;

	#[test]
	fn test_calc_expected_file_size() {
		run_test!({
			// Arrange
			let expected_size = (5 + 2 + 2 + 100 + 200 + 2 + 5 * TILE_DATA_SIZE + 256 * 3 + 5) as u64;
			let width = 10;
			let height = 10;
			let tile_count = 5;

			// Act
			let size = calc_expected_file_size(width, height, tile_count);

			// Assert
			assert_eq!(size, expected_size);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_calc_expected_file_size_standard_map() {
		run_test!({
			// Arrange
			let expected_size = (5 + 2 + 2 + 12544 + 25088 + 2 + 180 * TILE_DATA_SIZE + 768 + 180) as u64;
			let width = 112;
			let height = 112;
			let tile_count = 180;

			// Act
			let size = calc_expected_file_size(width, height, tile_count);

			// Assert
			assert_eq!(size, expected_size);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_calc_expected_file_size_mega_map() {
		run_test!({
			// Arrange
			let expected_size = (5 + 2 + 2 + 50176 + 100352 + 2 + 720 * TILE_DATA_SIZE + 768 + 720) as u64;
			let width = 224;
			let height = 224;
			let tile_count = 720;

			// Act
			let size = calc_expected_file_size(width, height, tile_count);

			// Assert
			assert_eq!(size, expected_size);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}

	#[test]
	fn test_calc_expected_file_size_all_max() {
		run_test!({
			// Arrange
			let expected_size = (5 + 2 + 2 + 65536 + 131072 + 2 + 65535 * TILE_DATA_SIZE + 768 + 65535) as u64;
			let width = 256;
			let height = 256;
			let tile_count = 65535;

			// Act
			let size = calc_expected_file_size(width, height, tile_count);

			// Assert
			assert_eq!(size, expected_size);

			let logs = get_captured_logs();
			assert!(logs.is_empty());
		});
	}
}

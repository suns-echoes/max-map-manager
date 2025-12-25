use std::{fs::File, io::{Read, Seek, SeekFrom}, path::Path};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::TILE_DATA_SIZE;

pub fn verify_file_content(file_path: &Path) -> Result<bool, String> {
	let mut file = match File::open(file_path) {
		Ok(file) => file,
		Err(_) => {
			let error_message = format!("Failed to open file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
		}
	};

	let header_size = 5;
	let mut header = vec![0; header_size];
	match file.read_exact(&mut header) {
		Ok(_) => {},
		Err(e) => {
			let error_message = format!("Failed to read WRL header: {}", e);
			log::error!("{}", error_message);
			return Err(error_message);
		}
	}

	let width = match file.read_u16::<LittleEndian>() {
		Ok(w) => w,
		Err(_) => {
			let error_message = format!("Failed to read width from file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
		}
	};

	let height = match file.read_u16::<LittleEndian>() {
		Ok(h) => h,
		Err(_) => {
			let error_message = format!("Failed to read height from file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
		}
	};

	{
		let minimap_size = (width as usize) * (height as usize);
		file.seek(SeekFrom::Current(minimap_size as i64)).map_err(|e| {
			let errror_message = format!("Failed to seek past minimap in file: {}", e);
			log::error!("{}", errror_message);
			errror_message
		})?;
	}

	{
		let bigmap_size = (width as usize) * (height as usize) * 2;
		file.seek(SeekFrom::Current(bigmap_size as i64)).map_err(|e| {
			let error_message = format!("Failed to seek past bigmap in file: {}", e);
			log::error!("{}", error_message);
			error_message
		})?;
	}

	let tile_count = match file.read_u16::<LittleEndian>() {
		Ok(tc) => tc,
		Err(_) => {
			let error_message = format!("Failed to read tile count from file: {}", file_path.display());
			log::error!("{}", error_message);
			return Err(error_message);
		}
	};

	let tiles_size = tile_count as usize * TILE_DATA_SIZE;
	file.seek(SeekFrom::Current(tiles_size as i64)).map_err(|e| {
		let error_message = format!("Failed to seek past tiles in file: {}", e);
		log::error!("{}", error_message);
		error_message
	})?;

	let palette_size = 256 * 3;
	file.seek(SeekFrom::Current(palette_size as i64)).map_err(|e| {
		let error_message = format!("Failed to seek past palette in file: {}", e);
		log::error!("{}", error_message);
		error_message
	})?;

	let pass_table_size = tile_count as usize;
	file.seek(SeekFrom::Current(pass_table_size as i64)).map_err(|e| {
		let error_message = format!("Failed to seek past passability table in file: {}", e);
		log::error!("{}", error_message);
		error_message
	})?;

	Ok(true)
}

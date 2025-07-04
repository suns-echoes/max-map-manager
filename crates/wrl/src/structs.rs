use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WRLFile {
	pub header: Vec<u8>,
	pub width: u16,
	pub height: u16,
	pub minimap: Vec<u8>,
	pub bigmap: Vec<u16>,
	pub tile_count: u16,
	pub tiles: Vec<u8>,
	pub palette: Vec<u8>,
	pub pass_table: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WRLHeader {
	pub width: u16,
	pub height: u16,
	pub tile_count: u16,
	pub minimap: Vec<u8>,
	pub palette: Vec<u8>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WRLTailHeader {
	pub _v: u16,
	pub hash_id: String,
	pub name: String,
	pub version: String,
	pub date: String,
	pub author: String,
	pub description: String,
	pub comment: String,
}

impl WRLTailHeader {
	pub fn new(hash_id: String) -> Self {
		WRLTailHeader {
			_v: 1,
			hash_id,
			name: String::from(""),
			version: String::from(""),
			date: String::from(""),
			author: String::from(""),
			description: String::from(""),
			comment: String::from(""),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MapMetadata {
	pub file_path: String,
	pub width: u16,
	pub height: u16,
	pub minimap: Vec<u8>,
	pub tail: WRLTailHeader,
}

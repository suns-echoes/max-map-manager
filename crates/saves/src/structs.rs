use serde::{Serialize};


#[derive(Serialize, Clone, Debug)]
pub struct SaveFileMetadata {
	pub version: i16,
	pub save_type: String,
	pub name: String,
	pub map_hash_id: String,
	pub mission_index: u32,
	pub current_turn: u32,
	pub difficulty: String,
	pub game_mode: String,
	pub victory_type: String,
	pub victory_limit: u32,
	pub player_color: String,
	pub player_name: String,
}

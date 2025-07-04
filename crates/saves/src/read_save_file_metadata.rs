use std::path::Path;

use crate::{structs::SaveFileMetadata, v70};


pub fn read_save_file_metadata(file_path: &Path, width: u16, height: u16) -> Result<SaveFileMetadata, String> {
    match v70::load_save_file_v70(file_path, width, height) {
		Ok(save_file) => Ok(create_save_metadata_from_v70(&save_file)),
		Err(e) => {
			Err(format!("Failed to load save file {}: {}", file_path.display(), e))
		},
	}
}


fn create_save_metadata_from_v70(save_file_v70: &v70::SaveFile) -> SaveFileMetadata {
    SaveFileMetadata {
        version: 70,
        save_type: match save_file_v70.header.save_file_type {
			v70::SaveFileType::Custom => "Custom".to_string(),
			v70::SaveFileType::Tutorial => "Tutorial".to_string(),
			v70::SaveFileType::Campaign => "Campaign".to_string(),
			v70::SaveFileType::HotSeat => "Hot Seat".to_string(),
			v70::SaveFileType::Multiplayer => "Multiplayer".to_string(),
			v70::SaveFileType::Demo => "Demo".to_string(),
			v70::SaveFileType::Debug => "Debug".to_string(),
			v70::SaveFileType::Text => "Text".to_string(),
			v70::SaveFileType::Scenario => "Scenario".to_string(),
			v70::SaveFileType::MultiScenario => "Multi-Scenario".to_string(),
		},
        name: save_file_v70.header.save_game_name.clone(),
        map_hash_id: "".to_string(),
        mission_index: save_file_v70.header.mission_index as u32,
        current_turn: save_file_v70.turn_counter as u32,
        difficulty: match save_file_v70.opponent {
			v70::OpponentType::Clueless => "Clueless".to_string(),
			v70::OpponentType::Apprentice => "Apprentice".to_string(),
			v70::OpponentType::Average => "Average".to_string(),
			v70::OpponentType::Expert => "Expert".to_string(),
			v70::OpponentType::Master => "Master".to_string(),
			v70::OpponentType::God => "God".to_string(),
		},
        game_mode: match save_file_v70.play_mode {
			v70::PlayMode::TurnBased => "Turn Based".to_string(),
			v70::PlayMode::SimultaneousMoves => "Simultaneous Moves".to_string(),
		},
        victory_type: match save_file_v70.options.victory_type {
			0 => "Duration".to_string(),
			1 => "Score".to_string(),
			2_u32..=u32::MAX => "?".to_string(),
		},
		victory_limit: save_file_v70.options.victory_limit as u32,
        player_color: match save_file_v70.player_team {
			v70::TeamIndex::Red => "Red".to_string(),
			v70::TeamIndex::Green => "Green".to_string(),
			v70::TeamIndex::Blue => "Blue".to_string(),
			v70::TeamIndex::Gray => "Gray".to_string(),
		},
        player_name: get_player_name(&save_file_v70),
    }
}


fn get_player_name(save_file_v70: &v70::SaveFile) -> String {
    save_file_v70.team_name[save_file_v70.player_team as usize].clone()
}

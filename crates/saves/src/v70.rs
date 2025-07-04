use std::convert::TryFrom;
use std::fs::File;
use std::io::{Read, Seek, Write};
use std::mem;
use std::path::Path;

use num_enum::TryFromPrimitive;

use crate::common::fixed_str_to_string;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum SaveFileType {
    Custom,
    Tutorial,
    Campaign,
    HotSeat,
    Multiplayer,
    Demo,
    Debug,
    Text,
    Scenario,
    MultiScenario,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum PlanetType {
    // Snow
    Snowcrab,
    Frigia,
    IceBerg,
    TheCooler,
    UltimaThule,
    LongFloes,
    // Crater
    IronCross,
    Splatterscape,
    Peakaboo,
    ValentinesPlanet,
    ThreeRings,
    GreatDivide,
    // Green
    NewLuzon,
    MiddleSea,
    HighImpact,
    Sanctuary,
    Islandia,
    Hammerhead,
    // Desert
    Freckles,
    Sandspit,
    GreatCircle,
    LongPassage,
    FlashPoint,
    Bottleneck,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum TeamType {
    None,
    Human,
    Computer,
    Remote,
    Eliminated,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum TeamClan {
    None,
    TheChosen,
    CrimsonPath,
    VonGriffin,
    AyersHand,
    Musashi,
    SacredEights,
    SevenKnights,
    AxisInc,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum OpponentType {
    Clueless,
    Apprentice,
    Average,
    Expert,
    Master,
    God,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum VictoryType {
    Duration,
    Score,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum PlayMode {
    TurnBased,
    SimultaneousMoves,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum TeamIndex {
    Red,
    Green,
    Blue,
    Gray,
}

// Static size: 36 bytes
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Header {
    pub version: i16,                 // 2 bytes
    pub save_file_type: SaveFileType, // 1 byte
    pub save_game_name: String,       // 30 bytes
    pub planet: PlanetType,           // 1 byte
    pub mission_index: u16,           // 2 bytes
}

// 12 x 4 bytes = 48 bytes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InitOptions {
    pub world: u32,
    pub turn_timer: u32,
    pub end_turn: u32,
    pub start_gold: u32,
    pub play_mode: u32,
    pub victory_type: u32,
    pub victory_limit: u32,
    pub opponent: u32,
    pub raw_resource: u32,
    pub fuel_resource: u32,
    pub gold_resource: u32,
    pub alien_derelicts: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResearchTopicInfo {
    pub research_level: u32,
    pub turns_to_complete: u32,
    pub allocation: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenLocation {
    pub x: i8,
    pub y: i8,
}

#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TeamInfo {
    pub markers: [Point; 10],
    pub team_type: TeamType,
    pub field_41: i8,
    pub team_clan: TeamClan,
    pub research_topics: [ResearchTopicInfo; 8],
    pub victory_points: u32,
    pub next_unit_id: u16,
    pub unit_counters: [u8; 93],
    pub screen_location: [ScreenLocation; 6],
    pub score_graph: [u16; 50],
    pub selected_unit: u16,
    pub zoom_level: u16,
    pub screen_position: Point,
    pub gui_button_state_range: bool,
    pub gui_button_state_scan: bool,
    pub gui_button_state_status: bool,
    pub gui_button_state_colors: bool,
    pub gui_button_state_hits: bool,
    pub gui_button_state_ammo: bool,
    pub gui_button_state_names: bool,
    pub gui_button_state_minimap_2x: bool,
    pub gui_button_state_minimap_tnt: bool,
    pub gui_button_state_grid: bool,
    pub gui_button_state_survey: bool,
    pub stats_factories_built: u16,
    pub stats_mines_built: u16,
    pub stats_buildings_built: u16,
    pub stats_units_built: u16,
    pub casualties: [u16; 93],
    pub stats_gold_spent_on_upgrades: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaveFile {
    // Static size: 224 bytes
    pub header: Header,           // 36 bytes
    pub team_name: [String; 4],   // 4 * 30 bytes = 120 bytes
    pub team_type: [TeamType; 5], // 5 * 1 byte = 5 bytes
    pub team_clan: [TeamClan; 5], // 5 * 1 byte = 5 bytes
    pub rng_seed: u32,            // 4 bytes
    pub opponent: OpponentType,   // 1 byte
    pub turn_timer: u16,          // 2 bytes
    pub end_turn: u16,            // 2 bytes
    pub play_mode: PlayMode,      // 1 byte
    pub options: InitOptions,     // 12 * 4 bytes = 48 bytes
    // Dynamic size:
    pub surface_map: Vec<u8>,       // map.width * map.height
    pub grid_resource_map: Vec<u8>, // map.width * map.height * 2
    // Static size:
    pub team_info: [TeamInfo; 4],    // 4 * 565 bytes = 2260 bytes
    pub active_turn_team: TeamIndex, // 1 byte
    pub player_team: TeamIndex,      // 1 byte
    pub turn_counter: i32,           // 4 bytes
    pub game_state: i16,             // 2 bytes
    pub turn_timer_time: u16,        // 2 bytes
                                     // Preferences and other stuff we don't care about
}

fn parse_enum<T>(value: u8, field_name: &str) -> Result<T, String>
where
    T: TryFrom<u8>,
    <T as TryFrom<u8>>::Error: std::fmt::Display,
{
    T::try_from(value)
        .map_err(|err| format!("Failed to parse {}: {}", field_name, err))
}

pub fn load_save_file_header_v70(file_path: &Path) -> Result<Header, String> {
    let expected_file_size = 36;

    let file_metadata = std::fs::metadata(file_path).map_err(|e| e.to_string())?;
    let file_size = file_metadata.len() as usize;

    if file_size < expected_file_size {
        return Err("File too short to be a valid SaveFileHeaderV70".to_string());
    }

    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut file_data = [0u8; 36];
    file.read_exact(&mut file_data).map_err(|e| e.to_string())?;

    let header = Header {
        version: i16::from_le_bytes(file_data[0x00..0x02].try_into().unwrap()),
        save_file_type: parse_enum::<SaveFileType>(
            file_data[0x02],
            "header.save_file_type",
        )?,
        save_game_name: fixed_str_to_string(&file_data[0x03..0x21]),
        planet: parse_enum::<PlanetType>(file_data[0x21], "header.planet")?,
        mission_index: u16::from_le_bytes(file_data[0x22..0x24].try_into().unwrap()),
    };

    if header.version != 70 {
        return Err(format!("Unsupported save file version: {}", header.version));
    }

    Ok(header)
}

pub fn load_save_file_v70(file_path: &Path, width: u16, height: u16) -> Result<SaveFile, String> {
    const A: usize = 0xE0;
    let b: usize = A + (width as usize * height as usize) * 3;
    let c: usize = b + mem::size_of::<TeamInfo>() * 4;
    let expected_file_size = c + 10; // TODO: + rest ...

    let mut team_types: Vec<TeamType> = vec![
        TeamType::None,
        TeamType::None,
        TeamType::None,
        TeamType::None,
        TeamType::None,
    ];

    let file_data = std::fs::read(file_path).map_err(|e| e.to_string())?;

    let version = u16::from_le_bytes(file_data[0x00..0x02].try_into().unwrap());

    if version != 70 {
        return Err(format!("Unsupported save file version: {}", version));
    }

    if file_data.len() < expected_file_size {
        return Err("File too short to be a valid SaveFileV70".to_string());
    }

    let save_data = SaveFile {
        header: Header {
            version: i16::from_le_bytes(file_data[0x00..0x02].try_into().unwrap()),
            save_file_type: parse_enum::<SaveFileType>(file_data[0x02], "save_file_type")?,
            save_game_name: fixed_str_to_string(&file_data[0x03..0x21]),
            planet: parse_enum::<PlanetType>(file_data[0x21], "planet")?,
            mission_index: u16::from_le_bytes(file_data[0x22..0x24].try_into().unwrap()),
        },
        team_name: [
            fixed_str_to_string(&file_data[0x24..0x42]),
            fixed_str_to_string(&file_data[0x42..0x60]),
            fixed_str_to_string(&file_data[0x60..0x7E]),
            fixed_str_to_string(&file_data[0x7E..0x9C]),
        ],
        team_type: [
            {
                let team_type = parse_enum::<TeamType>(file_data[0x9C], "team_type[0]")?;
                team_types[0] = team_type.clone();
                team_type
            },
            {
                let team_type = parse_enum::<TeamType>(file_data[0x9D], "team_type[1]")?;
                team_types[1] = team_type.clone();
                team_type
            },
            {
                let team_type = parse_enum::<TeamType>(file_data[0x9E], "team_type[2]")?;
                team_types[2] = team_type.clone();
                team_type
            },
            {
                let team_type = parse_enum::<TeamType>(file_data[0x9F], "team_type[3]")?;
                team_types[3] = team_type.clone();
                team_type
            },
            {
                let team_type = parse_enum::<TeamType>(file_data[0xA0], "team_type[4]")?;
                team_types[4] = team_type.clone();
                team_type
            },
        ],
        team_clan: [
            parse_enum::<TeamClan>(file_data[0xA1], "team_clan[0]")?,
            parse_enum::<TeamClan>(file_data[0xA2], "team_clan[1]")?,
            parse_enum::<TeamClan>(file_data[0xA3], "team_clan[2]")?,
            parse_enum::<TeamClan>(file_data[0xA4], "team_clan[3]")?,
            parse_enum::<TeamClan>(file_data[0xA5], "team_clan[4]")?,
        ],
        rng_seed: u32::from_le_bytes(file_data[0xA6..0xAA].try_into().unwrap()),
        opponent: parse_enum::<OpponentType>(file_data[0xAA], "opponent")?,
        turn_timer: u16::from_le_bytes(file_data[0xAB..0xAD].try_into().unwrap()),
        end_turn: u16::from_le_bytes(file_data[0xAD..0xAF].try_into().unwrap()),
        play_mode: parse_enum::<PlayMode>(file_data[0xAF], "play_mode")?,
        options: InitOptions {
            world: u32::from_le_bytes(file_data[0xB0..0xB4].try_into().unwrap()),
            turn_timer: u32::from_le_bytes(file_data[0xB4..0xB8].try_into().unwrap()),
            end_turn: u32::from_le_bytes(file_data[0xB8..0xBC].try_into().unwrap()),
            start_gold: u32::from_le_bytes(file_data[0xBC..0xC0].try_into().unwrap()),
            play_mode: u32::from_le_bytes(file_data[0xC0..0xC4].try_into().unwrap()),
            victory_type: u32::from_le_bytes(file_data[0xC4..0xC8].try_into().unwrap()),
            victory_limit: u32::from_le_bytes(file_data[0xC8..0xCC].try_into().unwrap()),
            opponent: u32::from_le_bytes(file_data[0xCC..0xD0].try_into().unwrap()),
            raw_resource: u32::from_le_bytes(file_data[0xD0..0xD4].try_into().unwrap()),
            fuel_resource: u32::from_le_bytes(file_data[0xD4..0xD8].try_into().unwrap()),
            gold_resource: u32::from_le_bytes(file_data[0xD8..0xDC].try_into().unwrap()),
            alien_derelicts: u32::from_le_bytes(file_data[0xDC..0xE0].try_into().unwrap()),
        },

        surface_map: file_data[A..(A + (width * height) as usize)].to_vec(),
        grid_resource_map: file_data[(A + (width * height) as usize)..]
            [(A + width as usize * height as usize)..(A + width as usize * height as usize * 2)]
            .to_vec(),

        team_info: [
            unsafe {
                mem::transmute::<[u8; 565], TeamInfo>(file_data[b..b + 565].try_into().unwrap())
            },
            unsafe {
                mem::transmute::<[u8; 565], TeamInfo>(
                    file_data[b + 565..b + 565 * 2].try_into().unwrap(),
                )
            },
            unsafe {
                mem::transmute::<[u8; 565], TeamInfo>(
                    file_data[b + 565 * 2..b + 565 * 3].try_into().unwrap(),
                )
            },
            unsafe {
                mem::transmute::<[u8; 565], TeamInfo>(
                    file_data[b + 565 * 3..b + 565 * 4].try_into().unwrap(),
                )
            },
        ],
        active_turn_team: parse_enum::<TeamIndex>(file_data[c], "active_turn_team")?,
        player_team: parse_enum::<TeamIndex>(file_data[c + 1], "player_team")?,
        turn_counter: i32::from_le_bytes(file_data[c + 2..c + 6].try_into().unwrap()),
        game_state: i16::from_le_bytes(file_data[c + 6..c + 8].try_into().unwrap()),
        turn_timer_time: u16::from_le_bytes(file_data[c + 8..c + 10].try_into().unwrap()),
    };

    Ok(save_data)
}

pub fn is_valid_save_file_v70(file_path: &str) -> Result<bool, String> {
    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut buf = [0u8; 2];
    file.read_exact(&mut buf).map_err(|e| e.to_string())?;
    let version = u16::from_le_bytes(buf);
    Ok(version == 70)
}

fn planet_slot_name_to_u8(map_slot_name: &str) -> Option<u8> {
	match map_slot_name {
		"SNOW_1" => Some(0),
		"SNOW_2" => Some(1),
		"SNOW_3" => Some(2),
		"SNOW_4" => Some(3),
		"SNOW_5" => Some(4),
		"SNOW_6" => Some(5),
		"CRATER_1" => Some(6),
		"CRATER_2" => Some(7),
		"CRATER_3" => Some(8),
		"CRATER_4" => Some(9),
		"CRATER_5" => Some(10),
		"CRATER_6" => Some(11),
		"GREEN_1" => Some(12),
		"GREEN_2" => Some(13),
		"GREEN_3" => Some(14),
		"GREEN_4" => Some(15),
		"GREEN_5" => Some(16),
		"GREEN_6" => Some(17),
		"DESERT_1" => Some(18),
		"DESERT_2" => Some(19),
		"DESERT_3" => Some(20),
		"DESERT_4" => Some(21),
		"DESERT_5" => Some(22),
		"DESERT_6" => Some(23),
		_ => None,
	}
}

pub fn overwrite_planet_type_v70(
	file_path: &Path,
	map_slot_name: &str,
) -> Result<(), String> {
	if !is_valid_save_file_v70(file_path.to_str().unwrap())? {
		return Err("Not a valid v70 save file".to_string());
	}

	let mut file = std::fs::OpenOptions::new()
		.write(true)
		.open(file_path)
		.map_err(|e| e.to_string())?;

	file.seek(std::io::SeekFrom::Start(0x21))
		.map_err(|e| e.to_string())?;
	if let Some(planet_byte) = planet_slot_name_to_u8(map_slot_name) {
		file.write_all(&[planet_byte]).map_err(|e| e.to_string())?;
	} else {
		return Err(format!("Unknown map slot name: {}", map_slot_name));
	}

	Ok(())
}

pub fn planet_type_to_u16(planet: PlanetType) -> u16 {
	match planet {
		PlanetType::Snowcrab => 0,
		PlanetType::Frigia => 1,
		PlanetType::IceBerg => 2,
		PlanetType::TheCooler => 3,
		PlanetType::UltimaThule => 4,
		PlanetType::LongFloes => 5,
		PlanetType::IronCross => 6,
		PlanetType::Splatterscape => 7,
		PlanetType::Peakaboo => 8,
		PlanetType::ValentinesPlanet => 9,
		PlanetType::ThreeRings => 10,
		PlanetType::GreatDivide => 11,
		PlanetType::NewLuzon => 12,
		PlanetType::MiddleSea => 13,
		PlanetType::HighImpact => 14,
		PlanetType::Sanctuary => 15,
		PlanetType::Islandia => 16,
		PlanetType::Hammerhead => 17,
		PlanetType::Freckles => 18,
		PlanetType::Sandspit => 19,
		PlanetType::GreatCircle => 20,
		PlanetType::LongPassage => 21,
		PlanetType::FlashPoint => 22,
		PlanetType::Bottleneck => 23,
	}
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    use test_utils::*;

    #[test]
    fn test_load_save_file_v70() {
        run_test!({
            let file_path = PathBuf::from("test_files/v70/SAVE31.DTA");
            let width = 16;
            let height = 16;
            let save_data = load_save_file_v70(&file_path, width, height).unwrap();
            assert_eq!(save_data.header.version, 70);
            assert_eq!(save_data.header.save_file_type, SaveFileType::Custom);
            assert_eq!(save_data.header.save_game_name, "TEST SAVE 1");
            assert_eq!(save_data.header.planet, PlanetType::IronCross);
            assert_eq!(save_data.header.mission_index, 0);

            assert_eq!(save_data.team_name[0], "Human Player");
            assert_eq!(save_data.team_type[0], TeamType::Human);
            assert_eq!(save_data.team_clan[0], TeamClan::CrimsonPath);
            assert_eq!(save_data.team_name[1], "Computer Player");
            assert_eq!(save_data.team_type[1], TeamType::Computer);
            assert_eq!(save_data.team_clan[1], TeamClan::AyersHand);
            assert_eq!(save_data.team_name[2], "");
            assert_eq!(save_data.team_type[2], TeamType::None);
            assert_eq!(save_data.team_clan[2], TeamClan::SacredEights);
            assert_eq!(save_data.team_name[3], "");
            assert_eq!(save_data.team_type[3], TeamType::None);
            assert_eq!(save_data.team_clan[3], TeamClan::SevenKnights);
            assert_eq!(save_data.team_clan[4], TeamClan::None);

            assert_eq!(save_data.rng_seed, 1754509078);
            assert_eq!(save_data.opponent, OpponentType::God);
            assert_eq!(save_data.turn_timer, 240);
            assert_eq!(save_data.end_turn, 45);
            assert_eq!(save_data.play_mode, PlayMode::TurnBased);

            assert_eq!(save_data.options.world, 6);
            assert_eq!(save_data.options.turn_timer, 240);
            assert_eq!(save_data.options.end_turn, 45);
            assert_eq!(save_data.options.start_gold, 150);
            assert_eq!(save_data.options.play_mode, 0);
            assert_eq!(save_data.options.victory_type, 0);
            assert_eq!(save_data.options.victory_limit, 9999);
            assert_eq!(save_data.options.opponent, 5);
            assert_eq!(save_data.options.raw_resource, 2);
            assert_eq!(save_data.options.fuel_resource, 2);
            assert_eq!(save_data.options.gold_resource, 2);
            assert_eq!(save_data.options.alien_derelicts, 1);
        });
    }

    #[test]
    fn test_is_valid_save_file_v70() {
        run_test!({
            let file_path = "test_files/v70/SAVE31.DTA";
            assert!(is_valid_save_file_v70(file_path).is_ok());
        });
    }
}

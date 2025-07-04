pub fn str_array_contains(arr: &[&str], value: &str) -> bool {
    arr.iter().any(|&s| s == value)
}

pub fn fixed_str_to_string(data: &[u8]) -> String {
    String::from_utf8_lossy(data)
        .trim_end_matches('\0')
        .to_string()
}

//// pub fn map_wrl_name_to_index(name: &str) -> Option<u16> {
//// 	let name = name.trim_end_matches(".WRL").to_uppercase();
//// 	let mut map_name_chunks = name.split('_');
//// 	let planet_name = map_name_chunks.next()?;
//// 	let map_index = map_name_chunks.next()?.parse::<u16>().ok();
////
//// 	if let Some(map_index) = map_index {
//// 		match planet_name {
//// 			"SNOW" => {
//// 				return Some(map_index);
//// 			},
//// 			"CRATER" => {
//// 				return Some(map_index + 6);
//// 			},
//// 			"GREEN" => {
//// 				return Some(map_index + 12);
//// 			},
//// 			"DESERT" => {
//// 				return Some(map_index + 18);
//// 			},
//// 			_ => return None,
//// 		}
//// 	}
////
//// 	None
//// }

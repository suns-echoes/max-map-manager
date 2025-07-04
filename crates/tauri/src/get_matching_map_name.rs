use std::collections::HashMap;

use crate::WRLTailHeader;


pub fn get_matching_map_name(known_maps: &HashMap<String, KnownMap>, hash_id: &str) -> Option<WRLTailHeader> {
	match known_maps.get(hash_id) {
		Some(known_map) => {
			return Some(WRLTailHeader {
				_v: 1,
				hash_id: hash_id.to_string(),
				name: known_map.name.to_owned(),
				version: known_map.version.to_owned(),
				date: known_map.date.to_owned(),
				author: known_map.author.to_owned(),
				description: known_map.description.to_owned(),
				comment: "".to_string(),
			});
		},
		None => {
			return None;
		},
	}
}

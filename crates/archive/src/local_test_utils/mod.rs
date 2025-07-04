use std::path::{Path, PathBuf};

use crate::registry::*;


pub fn create_test_registry(maps_count: usize, file_path: &Path) -> Registry {
	let mut registry = Registry::new();
	for i in 1..=maps_count {
		registry.set_map_entry(
			&format!("map_id_{}", i),
			RegistryMapEntry {
				map: format!("path/to/map{}.wrl", i),
				saves: vec![
					format!("path/to/save{}_1.dta", i),
					format!("path/to/save{}_2.dta", i),
				],
			},
		);
	}
	registry.file_path = file_path.to_path_buf();
	registry
}

pub fn get_reference_registry_path() -> PathBuf {
	PathBuf::from("test_files/reference/registry.json")
}

pub fn load_reference_registry_json() -> String {
	let reference_path = get_reference_registry_path();
	std::fs::read_to_string(reference_path).expect("Failed to read reference registry file")
		.trim().to_string()
}

pub fn create_reference_registry_json() -> String {
	r#"{"version":"1.0","archive":{"map_id_1":{"map":"path/to/map1.wrl","saves":["path/to/save1_1.dta","path/to/save1_2.dta"]},"map_id_2":{"map":"path/to/map2.wrl","saves":["path/to/save2_1.dta","path/to/save2_2.dta"]}}}"#.to_string()
}

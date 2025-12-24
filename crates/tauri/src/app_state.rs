use std::collections::HashMap;
use std::env::home_dir;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

use res_reader::ResReader;
use serde::Serialize;

use archive::{Registry, RegistryArchive};

use crate::load_known_maps_info::{KnownMapInfo, KnownMaps};

#[derive(Debug, Clone, Serialize)]
pub struct MapAndSaves {
    pub map: String,
    pub map_hash_id: String,
    pub saves: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MapMetadata {
    pub map_hash_id: String,
    pub file_name: String,
    pub file_path: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
struct AppStateInternal {
    pub app_data_dir_path: PathBuf,
    pub resource_dir_path: PathBuf,
    pub game_dir_path: PathBuf,
    pub saves_dir_path: PathBuf,
    pub archive_dir_path: PathBuf,
    pub archived_maps_and_saves: Vec<MapAndSaves>,
    pub installed_maps_and_saves: Vec<MapAndSaves>,
    #[serde(skip_serializing)]
    pub known_maps: KnownMaps,
    //// #[serde(skip_serializing)]
    //// pub selected_map_hash_id: Option<String>,
    #[serde(skip_serializing)]
    pub maps_metadata: HashMap<String, MapMetadata>,
    #[serde(skip_serializing)]
    pub archive_registry: Registry,

    #[serde(skip_serializing)]
    pub max_res_reader: Option<ResReader>,
    #[serde(skip_serializing)]
    pub mmm_res_reader: Option<ResReader>,

    pub needs_setup: bool,
}

#[derive(Clone, Serialize)]
pub struct AppState {
    #[serde(flatten)]
    internal: Arc<RwLock<AppStateInternal>>,
}

impl AppState {
    pub fn new() -> Self {
        let internal_state = AppStateInternal {
            app_data_dir_path: PathBuf::new(),
            resource_dir_path: PathBuf::new(),
            game_dir_path: PathBuf::new(),
            saves_dir_path: PathBuf::new(),
            archive_dir_path: PathBuf::new(),
            archived_maps_and_saves: Vec::new(),
            installed_maps_and_saves: Vec::new(),
            known_maps: KnownMaps::new(),
            //// selected_map_hash_id: None,
            maps_metadata: HashMap::new(),
            archive_registry: Registry::new(),

            max_res_reader: None,
            mmm_res_reader: None,

            needs_setup: false,
        };
        Self {
            internal: Arc::new(RwLock::new(internal_state)),
        }
    }

    // --- Public Methods ---

    pub fn get_known_map_info(&self, map_hash_id: &str) -> Option<KnownMapInfo> {
        self.internal
            .read()
            .unwrap()
            .known_maps
            .known_maps
            .get(map_hash_id)
            .cloned()
    }

    // --- Public Getters ---

    pub fn get_map_metadata(&self, map_hash_id: &str) -> Option<MapMetadata> {
        self.internal
            .read()
            .unwrap()
            .maps_metadata
            .get(map_hash_id)
            .cloned()
    }

    pub fn get_map_metadata_mut(&self, map_hash_id: &str) -> Option<MapMetadata> {
        self.internal
            .write()
            .unwrap()
            .maps_metadata
            .get_mut(map_hash_id)
            .cloned()
    }

    pub fn get_installed_maps_and_saves(&self) -> Vec<MapAndSaves> {
        self.internal
            .read()
            .unwrap()
            .installed_maps_and_saves
            .clone()
    }

    pub fn get_archived_maps_and_saves(&self) -> RegistryArchive {
        self.internal
            .read()
            .unwrap()
            .archive_registry
            .archive
            .clone()
    }

    pub fn app_data_dir_path(&self) -> PathBuf {
        self.internal.read().unwrap().app_data_dir_path.clone()
    }

    pub fn resource_dir_path(&self) -> PathBuf {
        self.internal.read().unwrap().resource_dir_path.clone()
    }

    pub fn game_dir_path(&self) -> PathBuf {
        self.internal.read().unwrap().game_dir_path.clone()
    }

    pub fn saves_dir_path(&self) -> PathBuf {
        self.internal.read().unwrap().saves_dir_path.clone()
    }

    pub fn archive_dir_path(&self) -> PathBuf {
        self.internal.read().unwrap().archive_dir_path.clone()
    }

    // --- Public Setters ---

    pub fn set_map_metadata(&self, map_hash_id: &str, metadata: MapMetadata) {
        self.internal
            .write()
            .unwrap()
            .maps_metadata
            .insert(map_hash_id.to_string(), metadata);
    }

    //// pub fn set_selected_map_hash_id(&self, map_hash_id: Option<&str>) {
    //// 	match map_hash_id {
    //// 		Some(id) => self.internal.write().unwrap().selected_map_hash_id = Some(id.to_string()),
    //// 		None => self.internal.write().unwrap().selected_map_hash_id = None,
    //// 	}
    //// }

    pub fn set_app_data_dir_path(&self, path: &Path) {
        self.internal.write().unwrap().app_data_dir_path = path.to_path_buf();
    }

    pub fn set_resource_dir_path(&self, path: &Path) {
        self.internal.write().unwrap().resource_dir_path = path.to_path_buf();
    }

    fn parse_home_dir_path(&self, path: &Path) -> PathBuf {
        if path.starts_with("~") || path.starts_with("$HOME") {
            if let Some(home_dir) = home_dir() {
                let expanded_path = path
                    .strip_prefix("~")
                    .or_else(|_| path.strip_prefix("$HOME"))
                    .map(|p| home_dir.join(p))
                    .unwrap_or_else(|_| path.to_path_buf());
                return expanded_path;
            }
        }
        path.to_path_buf()
    }

    pub fn set_game_dir_path(&self, path: &Path) {
        self.internal.write().unwrap().game_dir_path = self.parse_home_dir_path(path);
    }

    pub fn set_saves_dir_path(&self, path: &Path) {
        self.internal.write().unwrap().saves_dir_path = self.parse_home_dir_path(path);
    }

    pub fn set_archive_dir_path(&self, path: &Path) {
        self.internal.write().unwrap().archive_dir_path = self.parse_home_dir_path(path);
    }

    pub fn reload_archive_registry(&self) {
        let archive_dir_path = self.archive_dir_path();
        let registry_file_path = &archive_dir_path.join("registry.json");
        self.internal
            .write()
            .unwrap()
            .archive_registry
            .load(registry_file_path)
            .ok();
    }

    pub fn set_known_maps(&self, known_maps: KnownMaps) {
        self.internal.write().unwrap().known_maps = known_maps;
    }

    pub fn remove_map_metadata(&self, map_hash_id: &str) {
        self.internal
            .write()
            .unwrap()
            .maps_metadata
            .remove(map_hash_id);
    }

    pub fn clear_installed_maps_and_saves(&self) {
        self.internal
            .write()
            .unwrap()
            .installed_maps_and_saves
            .clear();
    }

    pub fn add_installed_map_and_saves(&self, map_path: PathBuf, saves: Vec<PathBuf>) {
        self.internal
            .write()
            .unwrap()
            .installed_maps_and_saves
            .push(MapAndSaves {
                map: map_path.to_string_lossy().to_string(),
                map_hash_id: wrl::hash_wrl_file_without_tail(&map_path)
                    .unwrap()
                    .to_string(),
                saves: saves
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect(),
            });
    }

    pub fn clear_archived_maps_and_saves(&self) {
        self.internal
            .write()
            .unwrap()
            .archived_maps_and_saves
            .clear();
    }

    pub fn add_archived_map_and_saves(&self, map: PathBuf, saves: Vec<PathBuf>) {
        self.internal
            .write()
            .unwrap()
            .archived_maps_and_saves
            .push(MapAndSaves {
                map: map.to_string_lossy().to_string(),
                map_hash_id: wrl::hash_wrl_file_without_tail(&map).unwrap().to_string(),
                saves: saves
                    .iter()
                    .map(|p| p.to_string_lossy().to_string())
                    .collect(),
            });
    }

    pub fn init_max_res_reader(&self) {
		if self.get_max_res_reader().is_some() {
			return;
		}
        let game_dir_path = self.game_dir_path();
        let mut max_res_path = game_dir_path.join("MAX.RES");
        if !max_res_path.exists() {
			max_res_path = game_dir_path.join("max.res");
		}
        let res_reader = ResReader::new(&max_res_path);
        self.internal.write().unwrap().max_res_reader = Some(res_reader);
    }

    pub fn get_max_res_reader(&self) -> Option<ResReader> {
        self.internal.read().unwrap().max_res_reader.clone()
    }

    pub fn init_mmm_res_reader(&self) {
        let resource_dir_path = self.resource_dir_path();
        let mmm_res_path = resource_dir_path.join("resources/mmm.res");
        let res_reader = ResReader::new(&mmm_res_path);
        self.internal.write().unwrap().mmm_res_reader = Some(res_reader);
    }

    pub fn get_mmm_res_reader(&self) -> Option<ResReader> {
        self.internal.read().unwrap().mmm_res_reader.clone()
    }

    pub fn set_needs_setup(&self, needs_setup: bool) {
        self.internal.write().unwrap().needs_setup = needs_setup;
    }

    pub fn needs_setup(&self) -> bool {
        self.internal.read().unwrap().needs_setup
    }
}

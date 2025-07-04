use std::{collections::HashMap, io::BufReader};

use serde::Deserialize;
use serde_ini;

#[derive(Debug, Clone, Deserialize)]
pub struct KnownMapInfo {
    #[serde(rename = "FileName")]
    pub file_name: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "Author")]
    pub author: String,
    #[serde(rename = "Date")]
    pub date: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct KnownMaps {
    #[serde(flatten)]
    pub known_maps: HashMap<String, KnownMapInfo>,
}

impl KnownMaps {
    pub fn new() -> Self {
        KnownMaps {
            known_maps: HashMap::new(),
        }
    }
}

pub fn load_known_maps_info(file_data: &Vec<u8>) -> Result<KnownMaps, String> {
    let known_maps: KnownMaps = serde_ini::de::from_read(BufReader::new(file_data.as_slice()))
        .map_err(|e| {
            log::error!("Failed to parse known maps file: {}", e);
            format!("Failed to parse known maps file: {}", e)
        })?;

    Ok(known_maps)
}

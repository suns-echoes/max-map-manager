use std::fs::{self, File};
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use crate::GLOBAL_APP_STATE;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AppSettings {
    pub game_dir: String,
    pub saves_dir: String,
    pub archive_dir: String,
}

pub fn load_app_settings() -> Result<AppSettings, String> {
    let app_data_dir_path = GLOBAL_APP_STATE.app_data_dir_path();
    let settings_file_path = app_data_dir_path.join("settings.json");

    log::info!(
        "Loading app settings from: {}",
        settings_file_path.display()
    );

    match File::open(&settings_file_path) {
        Ok(file) => read_settings_from_file(file, &settings_file_path),
        Err(err) if err.kind() == ErrorKind::NotFound => {
            log::info!(
                "User settings not found at: {}. Loading bundled defaults...",
                settings_file_path.display()
            );
            let settings = load_default_settings()?;
            save_app_settings(&settings)?;
            Ok(settings)
        }
        Err(err) => {
            log::error!(
                "Failed to open settings file: {}",
                settings_file_path.display()
            );
            log::error!("{}", err);
            Err(format!("Failed to open settings file: {}", err))
        }
    }
}

pub fn save_app_settings(settings: &AppSettings) -> Result<(), String> {
    let app_data_dir_path = GLOBAL_APP_STATE.app_data_dir_path();
    let settings_file_path = app_data_dir_path.join("settings.json");

    if let Some(parent_dir) = settings_file_path.parent() {
        fs::create_dir_all(parent_dir).map_err(|e| {
            log::error!(
                "Failed to create settings directory: {}",
                parent_dir.display()
            );
            log::error!("{}", e);
            format!("Failed to create settings directory: {}", e)
        })?;
    }

    log::info!("Saving app settings to: {}", settings_file_path.display());

    let file = File::create(&settings_file_path).map_err(|e| {
        log::error!(
            "Failed to create settings file: {}",
            settings_file_path.display()
        );
        log::error!("{}", e);
        format!("Failed to create settings file: {}", e)
    })?;

    serde_json::to_writer(file, settings).map_err(|e| {
        log::error!(
            "Failed to write settings JSON: {}",
            settings_file_path.display()
        );
        log::error!("{}", e);
        format!("Failed to write settings JSON: {}", e)
    })?;

    Ok(())
}

fn load_default_settings() -> Result<AppSettings, String> {
    let resource_dir_path = GLOBAL_APP_STATE.resource_dir_path();
    let default_settings_path = resource_dir_path.join("resources/settings.json");

    log::info!(
        "Loading default app settings from: {}",
        default_settings_path.display()
    );

    let file = File::open(&default_settings_path).map_err(|e| {
        log::error!(
            "Failed to open bundled settings file: {}",
            default_settings_path.display()
        );
        log::error!("{}", e);
        format!("Failed to open bundled settings file: {}", e)
    })?;

    read_settings_from_file(file, &default_settings_path)
}

fn read_settings_from_file(file: File, file_path: &Path) -> Result<AppSettings, String> {
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|e| {
        log::error!("Failed to parse settings JSON: {}", file_path.display());
        log::error!("{}", e);
        format!("Failed to parse settings JSON: {}", e)
    })
}

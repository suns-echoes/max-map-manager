use crate::app_settings::{self, AppSettings};

#[tauri::command]
pub fn read_settings_command() -> Result<AppSettings, String> {
    let settings = app_settings::load_app_settings()
        .map_err(|e| format!("Failed to load app settings: {}", e))?;
    Ok(settings)
}

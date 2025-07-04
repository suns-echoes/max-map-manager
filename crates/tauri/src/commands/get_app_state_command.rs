use crate::{GLOBAL_APP_STATE, app_state::AppState};

/// Retrieves the current application state,
/// and sends it to the front-end.
#[tauri::command]
pub async fn get_app_state_command() -> Result<AppState, String> {
    let state = GLOBAL_APP_STATE.clone();
    Ok(state)
}

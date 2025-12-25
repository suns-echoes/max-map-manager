use crate::GLOBAL_APP_STATE;

#[tauri::command]
pub fn is_setup_required_command() -> bool {
    let app_state = GLOBAL_APP_STATE.clone();
    app_state.needs_setup()
}

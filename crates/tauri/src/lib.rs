use std::path::PathBuf;

use tauri::Manager;

mod app_settings;
mod app_state;
mod commands;
mod internal_server;
mod load_known_maps_info;
use load_known_maps_info::load_known_maps_info;
mod log_file;

use app_state::AppState;
use commands::*;

lazy_static::lazy_static! {
    pub static ref GLOBAL_APP_STATE: AppState = AppState::new();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = log_file::setup_logger();

    log::info!("Starting application...");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data directory");

            let app_state = GLOBAL_APP_STATE.clone();
            app_state.set_app_data_dir_path(&app_data_dir);
            app_state.set_resource_dir_path(&app.path().resource_dir().unwrap());

            let settings = app_settings::load_app_settings().map_err(|e| {
                log::error!("Failed to load app settings: {}", e);
                e
            })?;

            app_state.init_mmm_res_reader();

            let known_maps_file_data = app_state
                .get_mmm_res_reader()
                .unwrap()
                .read_file("KNOWNMAP")
                .unwrap();
            let known_maps = load_known_maps_info(&known_maps_file_data)
                .expect("Failed to load known maps info");

            app_state.set_known_maps(known_maps);

            {
                app_state.set_game_dir_path(&PathBuf::from(settings.game_dir));
                app_state.set_saves_dir_path(&PathBuf::from(settings.saves_dir));
                app_state.set_archive_dir_path(&PathBuf::from(settings.archive_dir));
            }

            // If MAX.RES is not found SETUP is required
            {
                let game_dir_path = app_state.game_dir_path();
                let mut max_res_path = game_dir_path.join("MAX.RES");
                let saves_dir_path = app_state.saves_dir_path();
                let archive_dir_path = app_state.archive_dir_path();

                if !max_res_path.exists() {
                    max_res_path = game_dir_path.join("max.res");
                }

                if !max_res_path.exists() || !saves_dir_path.exists() || !archive_dir_path.exists()
                {
                    app_state.set_needs_setup(true);
                    log::info!("MAX.RES not found, setup required");
                    return Ok(());
                }
            }

            app_state.init_max_res_reader();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_archived_maps_and_saves_command,
            read_archived_maps_metadata_command,
            archive_map_and_saves_command,
            generate_bigmap_preview_command,
            get_app_state_command,
            get_installed_maps_and_saves_command,
            is_setup_required_command,
            read_installed_maps_metadata_command,
            read_save_files_metadata_command,
            read_settings_command,
            restore_map_and_saves_command,
            set_app_paths_command,
            verify_dir_path_command,
            verify_game_path_command,
			open_dir_path_in_file_explorer_command,
            open_devtools_command,
        ])
        .register_uri_scheme_protocol("be", move |_app, request| {
            internal_server::handle_request(request)
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

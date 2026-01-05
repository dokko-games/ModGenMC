mod models;
mod commands;
mod storage;
mod core;
use crate::models::version;
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_available_mc_versions,
            commands::project::get_recent_projects,
            commands::project::create_project
        ])
        .manage(core::state::AppState {
            current_project: Mutex::new(None),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_available_mc_versions() -> Vec<version::Version> {
    vec![
        version::Version {
            major: 26,
            minor: 1,
            patch: 0,
        },
        version::Version {
            major: 26,
            minor: 1,
            patch: 1,
        },
        version::Version {
            major: 26,
            minor: 1,
            patch: 2,
        },
        version::Version {
            major: 26,
            minor: 2,
            patch: 0,
        },
    ]
}

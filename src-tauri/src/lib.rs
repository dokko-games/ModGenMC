pub mod models;
pub mod project_commands;
pub mod project;

use crate::models::Version;

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
            project_commands::create_project,
            project_commands::get_recent_projects,
            get_available_mc_versions
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_available_mc_versions() -> Vec<Version> {
    vec![
        models::Version {
            major: 26,
            minor: 1,
            patch: 0,
        },
        models::Version {
            major: 26,
            minor: 1,
            patch: 1,
        },
        models::Version {
            major: 26,
            minor: 1,
            patch: 2,
        },
        models::Version {
            major: 26,
            minor: 2,
            patch: 0,
        },
    ]
}

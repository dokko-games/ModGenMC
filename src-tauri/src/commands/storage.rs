use crate::core::state::AppState;
use std::fs;
use std::path::PathBuf;

#[tauri::command]
pub fn save_current_project(state: tauri::State<AppState>) -> Result<(), String> {
    let guard = state.current_project.lock().unwrap();

    let project = guard
        .as_ref()
        .ok_or("No project currently loaded")?;

    let mut project_json_path = PathBuf::from(&project.path);
    project_json_path.push(".craftide");
    project_json_path.push("project.json");

    if let Some(parent) = project_json_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create project directory: {e}"))?;
    }

    let json = crate::storage::project_files::project_to_json(project);

    let json_string = serde_json::to_string_pretty(&json)
        .map_err(|e| format!("Failed to serialize project json: {e}"))?;

    fs::write(&project_json_path, json_string)
        .map_err(|e| format!("Failed to write project.json: {e}"))?;

    Ok(())
}

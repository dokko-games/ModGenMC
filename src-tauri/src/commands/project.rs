use std::{path::PathBuf, collections::HashSet};

use tauri::{AppHandle, Manager};
use std::fs;

use crate::{core::state::AppState, models::{project, recent, version}, storage};
//TODO: validate name on svelte: no first numbers, no weird letters
#[tauri::command]
pub async fn create_project(
    app: AppHandle,
    state: tauri::State<'_, AppState>,
    name: String,
    fixed_name: String,
    path: String,
    selected_version: String,
) -> Result<(), String> {
    // Convert the string to a Version
    let version: version::Version = selected_version.parse().unwrap();
    // Create the metadata for the recent project list
    let new_recent_project = recent::RecentProject {
        name: name.clone(),
        path: PathBuf::from(path.clone()),
        target_version: version.clone(),
    };
    let author = whoami::username().unwrap_or("Me".into());
    let mut mod_id = author.clone();
    mod_id.push_str(".craftide.");
    mod_id.push_str(&fixed_name);
    // Create the actual project
    let new_project = project::Project {
        name,
        path: PathBuf::from(path),
        mod_id,
        target_version: version,
        author: whoami::username().unwrap_or("Me".into()),
        description: "My mod created with CraftIDE".into(),
        generators: Vec::new()
    };
    // Add the project to the recent list
    add_recent_project(app, new_recent_project).await;
    // Set the current project to the created project
    set_current_project(&state, new_project);
    println!("{:?}", state.current_project.lock().unwrap().clone());
    Ok(())
}
#[tauri::command]
pub async fn open_project(state: tauri::State<'_, AppState>, project_path: PathBuf) -> Result<(), String> {
    let mut path = project_path;
    path.push(".craftide");
    path.push("project.json");
    let contents = fs::read_to_string(&path)
        .map_err(|_| format!("Could not read project files"))?;
    let proj = storage::project_files::json_to_project(&contents);
    set_current_project(&state, proj);
    Ok(())

}
pub fn set_current_project(state: &tauri::State<AppState>, project: project::Project) {
    let mut current = state.current_project.lock().unwrap();
    *current = Some(project);
}
#[tauri::command]
pub async fn get_recent_projects(app: AppHandle) -> Result<Vec<recent::RecentProject>, String> {
  let app_data = app.path().app_data_dir().unwrap();
    // load existing
    let recents = crate::storage::recent_projects::read_recent_list(&app_data).await;

    Ok(recents)
}
pub async fn add_recent_project(app: AppHandle, new_project: recent::RecentProject) {
    let app_data = app.path().app_data_dir().unwrap();
    // load existing
    let mut recents = crate::storage::recent_projects::read_recent_list(&app_data).await;

    // insert newest at top
    recents.insert(0, new_project);

    update_recent_projects(app, recents).await;
}
pub async fn update_recent_projects(app: AppHandle, mut recents: Vec<recent::RecentProject>) {
    let app_data = app.path().app_data_dir().unwrap();

    // remove duplicates, keeping the first (most recent) occurrence
    let mut seen = HashSet::new();
    recents.retain(|project| seen.insert(project.clone()));

    //cap list size
    recents.truncate(10);

    crate::storage::recent_projects::write_recent_list(&app_data, &recents).await;
}
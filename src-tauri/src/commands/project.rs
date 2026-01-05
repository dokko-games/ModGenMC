use std::{path::PathBuf};

use tauri::{AppHandle, Manager};

use crate::models::{recent};
#[tauri::command]
pub async fn get_recent_projects(app: AppHandle) -> Result<Vec<recent::RecentProject>, String> {
  let app_data = app.path().app_data_dir().unwrap();
    // load existing
    let recents = crate::storage::recent_projects::read_recent_list(&app_data).await;

    Ok(recents)
}
#[tauri::command]
pub async fn create_project(
    app: AppHandle,
    name: String,
    path: String,
    selected_version: String,
) -> Result<(), String> {
    let version = selected_version.parse().unwrap();

    let new_project = recent::RecentProject {
        name,
        path: PathBuf::from(path),
        target_version: version,
    };
    let app_data = app.path().app_data_dir().unwrap();
    // load existing
    let mut recents = crate::storage::recent_projects::read_recent_list(&app_data).await;

    // remove duplicates (same path)
    recents.retain(|p| p.path != new_project.path);

    // insert newest at top
    recents.insert(0, new_project);

    // optional: cap list size
    recents.truncate(10);

    crate::storage::recent_projects::write_recent_list(&app_data, &recents).await;

    Ok(())
}

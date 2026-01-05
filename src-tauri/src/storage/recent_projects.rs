use std::path::PathBuf;
use crate::models::recent::RecentProject;
use serde_json;
use std::fs;

fn recent_projects_path(app_data: &PathBuf) -> PathBuf {
    let mut dir = app_data.to_path_buf();

    dir.push("recent_projects.json");
    dir
}

pub async fn read_recent_list(app_data: &PathBuf) -> Vec<RecentProject> {
    let path = recent_projects_path(app_data);

    // file does not exist → no recents yet
    if !path.exists() {
        return Vec::new();
    }

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return Vec::new(), // unreadable file → fail soft
    };

    match serde_json::from_str::<Vec<RecentProject>>(&contents) {
        Ok(list) => list,
        Err(_) => Vec::new(), // corrupted json → fail soft
    }
}
pub async fn write_recent_list(app_data: &PathBuf, list: &[RecentProject]) {
    let path = recent_projects_path(&app_data);

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let json = match serde_json::to_string_pretty(list) {
        Ok(j) => j,
        Err(_) => return, // serialization failed → nothing we can do
    };

    // simple write (good enough for now)
    let _ = fs::write(path, json);
}

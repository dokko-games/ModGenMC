use std::sync::Mutex;
use crate::models::project::Project;

pub struct AppState {
    pub current_project: Mutex<Option<Project>>,
}

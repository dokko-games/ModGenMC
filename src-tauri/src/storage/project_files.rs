use serde_json::json;

use crate::models::project;

pub fn project_to_json(proj: &project::Project) -> serde_json::Value {
    let obj =json!(proj);
    obj
}
pub fn json_to_project(json: &String) -> project::Project {
    let proj: project::Project = serde_json::from_str(json).expect("Project JSON was not well-formatted");
    proj
}
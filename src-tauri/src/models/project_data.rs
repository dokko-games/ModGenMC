use crate::models::Version;
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub name: String,
    pub path: String,
    pub target_version: Version,
}

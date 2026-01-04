use crate::models::Version;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub name: String,
    pub path: String,
    pub target_version: Version,
}

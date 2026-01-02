use crate::models::ProjectData;
use std::fmt;
use serde::Serialize;
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub data: ProjectData,
}
impl fmt::Display for Project {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Project: {}", self.data.name)?;
        writeln!(f, "  Path: {}", self.data.path)?;
        writeln!(f, "  Target Version: {}", self.data.target_version)?;
        Ok(())
    }
}
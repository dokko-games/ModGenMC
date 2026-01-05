use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::models::version::Version;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub author: String, //author.craftide.name, so for example dokko.craftide.firstpersonreplays - this will default to pc username
    pub path: PathBuf,
    pub target_version: Version,
    pub description: String, // this will default to something like "my cool awesome mod!"
}

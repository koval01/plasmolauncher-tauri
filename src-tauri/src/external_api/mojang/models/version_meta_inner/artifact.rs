use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArtifactWithPath {
    pub path: PathBuf,
    pub sha1: String,
    pub size: u64,
    pub url: Url,
}
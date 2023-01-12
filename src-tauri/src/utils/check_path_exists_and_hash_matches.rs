use std::path::{PathBuf};

use super::hash_from_path::hash_from_path;
use anyhow::Result;


pub async fn check_path_exists_and_hash_matches(
    path: impl Into<PathBuf>,
    hash: impl Into<String>,
) -> Result<bool> {

    let path: PathBuf = path.into();

    if !path.exists() { return Ok(false) };

    let local_hash = hash_from_path(path).await?;

    if hash.into() != local_hash { return Ok(false) };

    Ok(true)
}
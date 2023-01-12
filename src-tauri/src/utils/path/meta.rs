use std::{path::{PathBuf}, fmt::Display, borrow::Borrow};

use anyhow::Result;
use tauri::AppHandle;

use super::get_data_dir;

pub fn get_meta_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_data_dir(handle)?;
    path.push("meta");

    Ok(path)
}

pub fn get_minecraft_meta_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_data_dir(handle)?;

    path.push("meta");
    path.push("minecraft");

    Ok(path)
}

pub fn get_version_manifest_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_minecraft_meta_path(handle)?;

    path.push("version_manifest_v2.json");

    Ok(path) 
}

pub fn get_version_meta_path(
    handle: impl Borrow<AppHandle>,
    version: impl Display + Into<String>
) -> Result<PathBuf> {

    let mut path = get_minecraft_meta_path(handle)?;

    path.push(format!("{version}.json"));
    // path.set_extension("json");

    Ok(path)
}
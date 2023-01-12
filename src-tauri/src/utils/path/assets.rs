use std::{path::PathBuf, borrow::Borrow, fmt::Display};

use anyhow::Result;
use tauri::AppHandle;

use super::get_data_dir;

pub fn get_assets_base_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_data_dir(handle)?;
    path.push("assets");
    Ok(path)
}

pub fn get_assets_index_path(
    handle: impl Borrow<AppHandle>,
    assets_index: impl Display + Into<String>
) -> Result<PathBuf> {
    let mut path = get_assets_base_path(handle)?;
    path.push("indexes");
    path.push(format!("{assets_index}.json"));
    Ok(path)
}

pub fn assets_relative_path(hash: impl Into<String>) -> PathBuf {
    let hash: String = hash.into();
    let dir = hash.chars().take(2).collect::<String>();

    let mut path = PathBuf::new();
    path.push(dir);
    path.push(hash);

    path
}

pub fn get_assets_objects_base_path(
    handle: impl Borrow<AppHandle>,
) -> Result<PathBuf> {
    let mut path = get_assets_base_path(handle)?;
    path.push("objects");
    
    Ok(path)
}

pub fn get_assets_object_path(
    handle: impl Borrow<AppHandle>,
    hash: impl Into<String>
) -> Result<PathBuf> {
    let mut path = get_assets_objects_base_path(handle)?;
    path.push(assets_relative_path(hash));
    
    Ok(path)
}
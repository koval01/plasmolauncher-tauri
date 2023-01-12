use std::{path::PathBuf, borrow::Borrow};

use anyhow::Result;
use tauri::AppHandle;

use super::get_data_dir;

pub fn get_libraries_base_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_data_dir(handle)?;
    path.push("libraries");
    Ok(path)
}
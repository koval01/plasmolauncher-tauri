use std::{path::PathBuf, borrow::Borrow};

use tauri::AppHandle;

use crate::error::TauriError;

use anyhow::Result;

pub mod java;
pub mod meta;
pub mod create_dir_all_without_file_name;
pub mod libraries;
pub mod assets;
// pub mod game;
pub mod launcher;

pub fn get_data_dir(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let path = handle.borrow().path_resolver()
        .app_data_dir()
        .ok_or(TauriError::DataDir)?;

    Ok(path)
}
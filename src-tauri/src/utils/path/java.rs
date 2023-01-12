use std::{path::{PathBuf, Path}, io::ErrorKind, borrow::Borrow};

use tauri::{AppHandle};
use std::fs::read_dir;
use anyhow::Result;

use super::get_data_dir;

pub fn check_if_java_binary_exists(
    handle: impl Borrow<AppHandle>,
    java_version: impl AsRef<Path>
) -> Result<bool> {
    let binary_path = get_java_binary_path(handle, java_version)?;
    Ok(binary_path.exists())
}

pub fn get_java_base_path(
    handle: impl Borrow<AppHandle>,
    java_version: impl AsRef<Path>
) -> Result<PathBuf> {
    let mut base_path = get_data_dir(handle)?;
    base_path.push("java");
    base_path.push(java_version);

    Ok(base_path)
}

pub fn get_java_binary_path(
    handle: impl Borrow<AppHandle>,
    java_version: impl AsRef<Path>
) -> Result<PathBuf> {
    let mut base_path = get_java_base_path(handle, java_version)?;
    let relative_path = get_java_binary_relative_path();
    base_path.push(relative_path);
    
    Ok(base_path)
}

pub fn get_java_binary_relative_path() -> PathBuf {

    let mut path = PathBuf::new();

    if std::env::consts::OS == "macos" { path.push("Home") }

    path.push("bin");

    if std::env::consts::OS == "windows" {
        path.push("javaw.exe")
    } else {
        path.push("java")
    }

    path
}

pub async fn get_unpacked_contents_paths(unpacked_path: impl Into<PathBuf>) -> Result<Vec<PathBuf>> {

    let mut path: PathBuf = unpacked_path.into();

    let file_name = read_dir(&path)?.flatten()
        .find_map(|entry| {
            let file_name = entry.file_name().to_str()?.to_string();
            if file_name.starts_with("jdk") {
                return Some(file_name);
            } else {
                return None;
            }
        })
        .ok_or(std::io::Error::new(ErrorKind::Other, 
            "Didn't found a directory that stats with 'jdk'"
        ))?;

    path.push(file_name);

    if std::env::consts::OS == "macos" {
        path.push("Contents");
    }

    let paths = read_dir(&path)?
        .flatten()
        .map(|entry| {
            entry.path()
        })
        .collect::<Vec<PathBuf>>();

    Ok(paths)
}

pub async fn get_unpacked_legal_path(unpacked_path: impl Into<PathBuf>) -> Result<PathBuf> {

    let mut path: PathBuf = unpacked_path.into();

    let file_name = read_dir(&path)?.flatten()
        .find_map(|entry| {
            let file_name = entry.file_name().to_str()?.to_string();
            if file_name.starts_with("jdk") {
                return Some(file_name);
            } else {
                return None;
            }
        })
        .ok_or(std::io::Error::new(ErrorKind::Other, 
            "Didn't found a directory that stats with 'jdk'"
        ))?;

    path.push(file_name);

    path.push("legal");

    Ok(path)
}
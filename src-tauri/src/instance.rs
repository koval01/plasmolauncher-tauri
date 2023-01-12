use std::{path::{PathBuf, Path}, borrow::Borrow, fmt::Display};

use plasmolauncher_common::models::instance::Instance;

use tauri::AppHandle;

use crate::utils::path::{get_data_dir, libraries::get_libraries_base_path};
use anyhow::Result;

pub trait InstanceTrait {
    fn get_dir_path(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf>;
    fn get_client_jar_path(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf>;
}

impl InstanceTrait for Instance {
    fn get_dir_path(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
        let mut path = get_data_dir(handle)?;
        path.push("instances");

        let relative_path = match &self.path {
            Some(path) => path.clone(),
            None => PathBuf::from(&self.name),
        };

        path.extend(&relative_path);
        Ok(path)
    }

    fn get_client_jar_path(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
        let version = &self.game_version;
        Ok(get_version_client_jar_path(handle, version)?)
    }
}

pub fn get_version_client_jar_path(
    handle: impl Borrow<AppHandle>,
    version: impl AsRef<Path> + Display,
) -> Result<PathBuf> {
    let mut path = get_libraries_base_path(handle)?;
    path.push("com");
    path.push("mojang");
    path.push("minecraft");
    path.push(version.as_ref());
    path.push(format!("minecraft-{version}-client.jar"));
    Ok(path)
}
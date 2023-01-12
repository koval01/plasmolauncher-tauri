use std::{path::PathBuf, borrow::Borrow, fmt::Display};


use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use anyhow::Result;

use crate::{external_api::fabric::{cached_file::CachedGameLoaderVersionFile, get_path_and_url_from_maven_name::get_path_and_url_from_maven_name}, utils::path::meta::{get_meta_path}, task::downloader::DownloaderItem};

use super::launcher_meta::LauncherMeta;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FabricLoaderMeta {
    pub loader: Loader,
    pub intermediary: Intermediary,
    pub launcher_meta: LauncherMeta,
}

impl FabricLoaderMeta {
    pub fn get_classpaths(&self, handle: impl Borrow<AppHandle>) -> Result<Vec<PathBuf>> {

        let handle = handle.borrow();

        let mut vec = self.launcher_meta.libraries.get_classpaths(handle)?;

        vec.push(self.intermediary.get_classpath(handle)?);
        vec.push(self.loader.get_classpath(handle)?);

        Ok(vec)
    }

    pub fn get_downloader_items(&self, handle: impl Borrow<AppHandle>) -> Result<Vec<DownloaderItem>> {

        let handle = handle.borrow();

        let mut vec = self.launcher_meta.libraries.get_libraries_downloader_items(handle)?;

        vec.push(self.intermediary.get_downloader_item(handle)?);
        vec.push(self.loader.get_downloader_item(handle)?);

        Ok(vec)
    }
}

impl CachedGameLoaderVersionFile for FabricLoaderMeta {
    fn path(
        handle: impl Borrow<AppHandle>,
        game_version: impl Display + Into<String>,
        loader_version: impl Display + Into<String>,
    ) -> Result<PathBuf> {

        let mut path = get_meta_path(handle)?;
        path.push("fabric");
        path.push(format!("{game_version}_{loader_version}.json"));

        Ok(path)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Loader {
    pub separator: String,
    pub build: u32,
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

impl Loader {
    pub fn get_classpath(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
        let (path, _) =
            get_path_and_url_from_maven_name(&self.maven, handle, None)?;
        
        Ok(path)
    }
    pub fn get_downloader_item(&self, handle: impl Borrow<AppHandle>) -> Result<DownloaderItem> {
        let (path, url) =
            get_path_and_url_from_maven_name(&self.maven, handle, None)?;
        
        Ok(DownloaderItem::new(path, url, 0))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Intermediary {
    pub maven: String,
    pub version: String,
    pub stable: bool,
}

impl Intermediary {
    pub fn get_classpath(&self, handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
        let (path, _) =
            get_path_and_url_from_maven_name(&self.maven, handle, None)?;
        
        Ok(path)
    }
    pub fn get_downloader_item(&self, handle: impl Borrow<AppHandle>) -> Result<DownloaderItem> {
        let (path, url) =
            get_path_and_url_from_maven_name(&self.maven, handle, None)?;
        
        Ok(DownloaderItem::new(path, url, 0))
    }
}
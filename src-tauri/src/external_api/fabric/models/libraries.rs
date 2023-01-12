use std::{borrow::Borrow, path::PathBuf};

use reqwest::Url;
use serde::{Serialize, Deserialize};
use anyhow::Result;
use tauri::AppHandle;

use crate::{task::downloader::DownloaderItem, external_api::fabric::get_path_and_url_from_maven_name::get_path_and_url_from_maven_name};

#[derive(Serialize, Deserialize, Debug)]
pub struct Libraries {
    client: Vec<FabricLibrary>,
    common: Vec<FabricLibrary>,
    server: Vec<FabricLibrary>,
}

impl Libraries {

    pub fn get_classpaths(&self, handle: impl Borrow<AppHandle>) -> Result<Vec<PathBuf>> {
        let vec = (&self.common).into_iter()
            .flat_map(|library| {
                let (path, _) = get_path_and_url_from_maven_name(
                    &library.name,
                    handle.borrow(),
                    Some(&library.url)
                ).ok()?;
                Some(path)
            })
            .collect();

        Ok(vec)
    }

    pub fn get_libraries_downloader_items(&self, handle: impl Borrow<AppHandle>) -> Result<Vec<DownloaderItem>> {

        let vec = (&self.common).into_iter()
            .chain(&self.client)
            .flat_map(|library| {
                let (path, url) = get_path_and_url_from_maven_name(
                    &library.name,
                    handle.borrow(),
                    Some(&library.url)
                ).ok()?;
                Some(DownloaderItem::new(path, url, 0))
            })
            .collect::<Vec<_>>();

        Ok(vec)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FabricLibrary {
    pub name: String,
    pub url: Url,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MainClass {
    pub client: String,
    pub server: String,
}
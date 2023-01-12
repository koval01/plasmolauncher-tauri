use std::{collections::HashMap, borrow::Borrow, fmt::Display, path::PathBuf};
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use anyhow::Result;

use crate::{external_api::mojang::cached_file::CachedVersionFile, utils::{path::assets::{get_assets_index_path}}};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64
}

#[async_trait]
impl CachedVersionFile for AssetIndex {
    fn path(
        handle: impl Borrow<AppHandle>,
        asset_index_id: impl Display + Into<String>
    ) -> Result<PathBuf> {
        Ok(get_assets_index_path(handle, asset_index_id)?)
    }
}
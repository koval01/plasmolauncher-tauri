
use std::{borrow::Borrow, fmt::Display, path::PathBuf};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use anyhow::Result;
use rayon::prelude::*;

use crate::{external_api::mojang::cached_file::CachedVersionFile, utils::{path::{meta::get_version_meta_path, libraries::get_libraries_base_path}}, instance::get_version_client_jar_path};

use super::{version_meta_inner::{argument::Arguments, asset_index::AssetIndex, downloads::Downloads, java_version::JavaVersion, libraries::LibrariesItem, logging::Logging}, general::version_type::VersionType};

// #TODO: Implement full specification for older versions. E.g. extract, natives, classifiers
// https://minecraft.fandom.com/wiki/Client.json

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionMeta {
    pub arguments: Arguments,
    pub asset_index: AssetIndex,
    pub assets: String,
    pub compliance_level: u8,
    pub downloads: Downloads,
    pub id: String,
    pub java_version: JavaVersion,
    pub libraries: Vec<LibrariesItem>,
    pub logging: Logging,
    pub main_class: String,
    pub minimum_launcher_version: i32,
    pub release_time: DateTime<Utc>,
    pub time: DateTime<Utc>,
    pub r#type: VersionType,
}

#[async_trait]
impl CachedVersionFile for VersionMeta {
    fn path(
        handle: impl Borrow<AppHandle>,
        version: impl Display + Into<String>
    ) -> Result<PathBuf> {
        Ok(get_version_meta_path(handle, version)?)
    }
}


impl VersionMeta {
    pub fn get_classpaths(&self, handle: impl Borrow<AppHandle>) -> Result<Vec<PathBuf>> {

        let handle = handle.borrow();

        let base_path = get_libraries_base_path(handle)?;

        let client_jar_path = get_version_client_jar_path(handle, &self.id)?;

        let vec = self.libraries.par_iter()
            .filter(|library| library.check_if_rules_satisfied() )
            .map(|library| library.get_local_path(&base_path) )
            .chain(rayon::iter::once(client_jar_path))
            .collect();

        Ok(vec)
        // Vec::new()
    }
}
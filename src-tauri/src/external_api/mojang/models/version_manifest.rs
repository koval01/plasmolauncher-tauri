use std::{borrow::Borrow, path::PathBuf};

use chrono::{DateTime, Utc};
use futures::TryFutureExt;
use reqwest::Url;
use serde::{Serialize, Deserialize};
use rayon::prelude::*;
use anyhow::Result;
use tauri::{AppHandle, Manager};
use tokio::fs::write;

// use crate::task::tasks::download_meta::DownloadClientError;

use crate::{task::tasks::download_meta::DownloadMetaError, external_api::mojang::cached_file::CachedFile, utils::{path::{meta::get_version_manifest_path, create_dir_all_without_file_name::create_dir_all_without_file_name}, into_future::IntoFuture}};
use crate::state::config::LauncherConfig;

use super::general::version_type::VersionType;

#[derive(Serialize, Deserialize, Debug)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Latest {
    release: String,
    snapshot: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    r#type: VersionType,
    pub url: Url,
    time: DateTime<Utc>,
    release_time: DateTime<Utc>,
    pub sha1: String,
    compliance_level: u8,
}

impl CachedFile for VersionManifest {
    fn path(handle: impl Borrow<AppHandle>) -> Result<PathBuf>  {
        Ok(get_version_manifest_path(handle)?)
    }
}

impl VersionManifest {

    pub fn get_version(&self, version: impl Into<String>) -> Result<Version> {
        let version: String = version.into();
        self.versions
            .par_iter()
            .find_any(|item| item.id.as_str() == version.as_str())
            .and_then(|version| Some(version.clone()))
            .ok_or(DownloadMetaError::VersionNotFound(version).into())
    }

    pub fn download_url(handle: impl Borrow<AppHandle>) -> Url {
        let config = handle.borrow().state::<LauncherConfig>();
        config.mojang.version_manifest_url.clone()
    }

    pub async fn download(handle: impl Borrow<AppHandle>) -> Result<()> {

        let handle = handle.borrow();

        let url = Self::download_url(handle);
    
        let bytes = reqwest::get(url)
            .await?
            .bytes()
            .await?;
    
        let path = Self::path(handle)?;
    
        create_dir_all_without_file_name(&path).await?;
    
        write(path, bytes).await?;
    
        Ok(())
    }

    pub async fn get_safe(handle: impl Borrow<AppHandle>) -> Result<Self> {

        let handle = handle.borrow();

        if !Self::check_exists(handle)? { Self::download(handle).await? };

        let version_manifest = Self::get_cached(handle).await
            .into_future()
            .or_else(|_| async move {
                Self::download(handle).await?;
                Self::get_cached(handle).await
            })
            .await?;

        Ok(version_manifest)

    }
}

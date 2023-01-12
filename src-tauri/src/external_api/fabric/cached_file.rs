use std::{borrow::Borrow, fmt::Display, path::PathBuf};

use async_trait::async_trait;

use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};
use tokio::fs::write;

use anyhow::Result;

use crate::{utils::{read_and_parse_json::read_and_parse_json, path::create_dir_all_without_file_name::create_dir_all_without_file_name}, state::config::LauncherConfig};


#[async_trait]
pub trait CachedGameLoaderVersionFile
where Self: Sized + Send + Serialize + for<'a> Deserialize<'a>
{
    fn path(
        handle: impl Borrow<AppHandle>,
        game_version: impl Display + Into<String>,
        loader_version: impl Display + Into<String>,
    ) -> Result<PathBuf>;

    fn check_exists(
        handle: impl Borrow<AppHandle>,
        game_version: impl Display + Into<String>,
        loader_version: impl Display + Into<String>
    ) -> Result<bool> {
        let path = Self::path(handle, game_version, loader_version)?;
        Ok(path.exists())
    }
    
    async fn get_cached(
        handle: impl Borrow<AppHandle> + Send,
        game_version: impl Display + Into<String> + Send,
        loader_version: impl Display + Into<String> + Send
    ) -> Result<Self> {
        let path = Self::path(handle, game_version, loader_version)?;
        let version_meta = read_and_parse_json(path).await?;
        Ok(version_meta)
    }

    async fn validate_or_download(
        handle: impl Borrow<AppHandle> + Send,
        game_version: impl Display + Into<String> + Send,
        loader_version: impl Display + Into<String> + Send,
    ) -> Result<Self> {

        let handle = handle.borrow();
        let game_version: String = game_version.into();
        let loader_version: String = loader_version.into();

        let check = Self::check_exists(
            handle,
            &game_version,
            &loader_version
        )?;

        if check { return Ok(Self::get_cached(handle, &game_version, &loader_version).await?) };

        let api_url = handle.state::<LauncherConfig>()
            .fabric
            .api_url
            .clone();

        let url = format!("{api_url}/versions/loader/{game_version}/{loader_version}");

        let bytes = reqwest::get(url)
            .await?
            .bytes()
            .await?;

        let path = Self::path(handle, &game_version, &loader_version)?;
        
        create_dir_all_without_file_name(&path).await?;

        write(path, &bytes).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }
}
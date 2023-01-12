use std::{borrow::Borrow, fmt::Display, path::PathBuf};

use async_trait::async_trait;
use reqwest::IntoUrl;
use serde::{Serialize, Deserialize};
use tauri::AppHandle;
use anyhow::Result;
use tokio::fs::write;

use crate::utils::{path::create_dir_all_without_file_name::create_dir_all_without_file_name, check_path_exists_and_hash_matches::check_path_exists_and_hash_matches, read_and_parse_json::read_and_parse_json};

#[async_trait]
pub trait CachedVersionFile
where Self: Sized + Send + Serialize + for<'a> Deserialize<'a>
{
    fn path(
        handle: impl Borrow<AppHandle>,
        version: impl Display + Into<String>
    ) -> Result<PathBuf>;

    fn check_exists(
        handle: impl Borrow<AppHandle>,
        version: impl Display + Into<String>
    ) -> Result<bool> {
        let path = Self::path(handle, version)?;
        Ok(path.exists())
    }
    
    async fn get_cached(
        handle: impl Borrow<AppHandle> + Send,
        version: impl Display + Into<String> + Send
    ) -> Result<Self> {
        let path = Self::path(handle, version)?;
        let version_meta = read_and_parse_json(path).await?;
        Ok(version_meta)
    }
    
    async fn check_exists_and_hash_matches(
        handle: impl Borrow<AppHandle> + Send,
        version: impl Display + Into<String> + Send,
        hash: impl Into<String> + Send,
    ) -> Result<bool> {
        let path = Self::path(handle, version)?;
        check_path_exists_and_hash_matches(path, hash).await
    }

    async fn validate_or_download(
        handle: impl Borrow<AppHandle> + Send,
        version: impl Display + Into<String> + Send,
        hash: impl Into<String> + Send,
        url: impl IntoUrl + Send,
    ) -> Result<Self> {

        let handle = handle.borrow();
        let version: String = version.into();

        let check = Self::check_exists_and_hash_matches(
            handle,
            &version,
            hash
        ).await?;

        if check { return Ok(Self::get_cached(handle, &version).await?) };

        let bytes = reqwest::get(url)
            .await?
            .bytes()
            .await?;

        let path = Self::path(handle, version)?;
        
        create_dir_all_without_file_name(&path).await?;

        write(path, &bytes).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }
}

#[async_trait]
pub trait CachedFile
where Self: Sized + Send + Serialize + for<'a> Deserialize<'a>
{
    fn path(
        handle: impl Borrow<AppHandle>,
    ) -> Result<PathBuf>;

    fn check_exists(
        handle: impl Borrow<AppHandle>,
    ) -> Result<bool> {
        let path = Self::path(handle)?;
        Ok(path.exists())
    }
    
    async fn get_cached(
        handle: impl Borrow<AppHandle> + Send,
    ) -> Result<Self> {
        let path = Self::path(handle)?;
        let version_meta = read_and_parse_json(path).await?;
        Ok(version_meta)
    }
    
    async fn check_exists_and_hash_matches(
        handle: impl Borrow<AppHandle> + Send,
        hash: impl Into<String> + Send,
    ) -> Result<bool> {
        let path = Self::path(handle)?;
        check_path_exists_and_hash_matches(path, hash).await
    }

    async fn validate_or_download(
        handle: impl Borrow<AppHandle> + Send,
        hash: impl Into<String> + Send,
        url: impl IntoUrl + Send,
    ) -> Result<Self> {

        let handle = handle.borrow();

        let check = Self::check_exists_and_hash_matches(
            handle,
            hash
        ).await?;

        if check { return Ok(Self::get_cached(handle).await?) };

        let bytes = reqwest::get(url)
            .await?
            .bytes()
            .await?;

        let path = Self::path(handle)?;
        
        create_dir_all_without_file_name(&path).await?;

        write(path, &bytes).await?;

        Ok(serde_json::from_slice(&bytes)?)
    }
}
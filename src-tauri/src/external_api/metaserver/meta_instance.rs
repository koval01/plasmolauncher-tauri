
use std::{borrow::Borrow, path::PathBuf};

use anyhow::Result;
use plasmolauncher_common::{models::metaserver::meta_instance::MetaInstance, utils::create_dir_all_without_file_name::create_dir_all_without_file_name};
use tauri::{AppHandle, Manager};


use crate::{state::config::LauncherConfig, utils::path::meta::get_meta_path};

pub fn get_instance_meta_cache_path(instance_name: impl Into<String>, handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let handle = handle.borrow();
    let instance_name: String = instance_name.into();

    let mut path = get_meta_path(handle)?;
    path.push("metaserver");
    path.push(format!("{instance_name}.json"));

    Ok(path)
}

pub async fn cache_meta_instance(instance_name: impl Into<String>, handle: impl Borrow<AppHandle>) -> Result<()> {
    
    let handle = handle.borrow();
    let instance_name: String = instance_name.into();

    let config = handle.state::<LauncherConfig>();

    let base_url = &config.metaserver.api_url;

    let url = format!("{base_url}instances/{instance_name}");

    let meta_instance = reqwest::get(url)
        .await?
        .json::<MetaInstance>()
        .await?;

    let string = serde_json::to_string(&meta_instance)?;

    let path = get_instance_meta_cache_path(&instance_name, handle)?;

    create_dir_all_without_file_name(&path).await?;

    tokio::fs::write(path, string).await?;

    Ok(())
}

pub async fn get_meta_instance(instance_name: impl Into<String>, handle: impl Borrow<AppHandle>) -> Result<MetaInstance> {
    
    let handle = handle.borrow();
    let instance_name: String = instance_name.into();

    let path = get_instance_meta_cache_path(&instance_name, handle)?;

    let string = tokio::fs::read_to_string(path).await?;

    let value = serde_json::from_str(&string)?;

    Ok(value)
}


pub async fn try_get_meta_instance(instance_name: impl Into<String>, handle: impl Borrow<AppHandle>) -> Result<MetaInstance> {
    
    let handle = handle.borrow();
    let instance_name: String = instance_name.into();

    match get_meta_instance(&instance_name, handle).await {
        Ok(value) => Ok(value),
        Err(_) => {
            cache_meta_instance(&instance_name, handle).await?;
            get_meta_instance(&instance_name, handle).await
        },
    }
}
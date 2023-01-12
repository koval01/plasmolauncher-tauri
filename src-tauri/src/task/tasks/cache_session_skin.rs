use std::{borrow::Borrow, time::Duration, path::PathBuf};

use anyhow::{Result, anyhow};
use reqwest::{StatusCode, header::IF_NONE_MATCH};
use tauri::{AppHandle, Manager};
use tauri_plugin_synced_state::SyncedToml;

use crate::{state::{config::LauncherConfig, session::Session}, utils::{path::{get_data_dir, create_dir_all_without_file_name::create_dir_all_without_file_name}, minecraft::offline_nick_to_skin_name::offline_nick_to_skin_name, string_traits::AddDoubleQuoutes}};

pub async fn update_skin(
    handle: impl Borrow<AppHandle>
    // nick: impl Into<String>
// bool is true if the skin was modified
) -> Result<bool> {

    let handle = handle.borrow();

    let session = handle.state::<SyncedToml<Session>>();

    let nick = session.get()
        .await
        .get_nick_or_default();

    let client = reqwest::Client::new();

    let url = skin_url_from_nick(handle, nick);

    // dbg!(&url);

    let etag = get_avatar_md5(handle).await?
        .add_double_quotes();

    let res = client
        .get(url)
        .header(IF_NONE_MATCH, etag)
        .send()
        .await?;

    if let StatusCode::NOT_MODIFIED = res.status() {
        return Ok(false);
    }

    let bytes = res.bytes().await?;

    cache_skin_bytes(handle, bytes).await?;

    Ok(true)
}

fn skin_url_from_nick(
    handle: impl Borrow<AppHandle>,
    nick: impl Into<String>
) -> String {
    let config = handle.borrow().state::<LauncherConfig>();

    let base_url = &config.minecraft_avatar.api_url;
    let nick = nick.into();
    let query_params = &config.minecraft_avatar.query_params;

    format!("{base_url}/{nick}{query_params}")
}

async fn get_avatar_md5(handle: impl Borrow<AppHandle>) -> Result<String> {
    let handle = handle.borrow();
    let path = avatar_path(handle)?;

    let bytes = tokio::fs::read(path).await?;
    
    let md5 = md5::compute(bytes).0;

    Ok(hex::encode(md5))
}

pub async fn cache_skin_nick(
    handle: AppHandle,
    nick: impl Into<String>
) -> Result<bool> {

    let handle = handle.borrow();

    let nick: String = nick.into();

    let bytes = match fetch_remote_skin(handle, &nick).await {
        Ok(bytes) => bytes,
        Err(_) => fetch_local_skin(handle, &nick).await?,
    };

    cache_skin_bytes(handle, bytes).await?;

    Ok(true)
}

fn avatar_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let mut path = get_data_dir(handle)?;
    path.push("cache");
    path.push("avatar.png");

    Ok(path)
}

async fn cache_skin_bytes(
    handle: impl Borrow<AppHandle>,
    bytes: impl AsRef<[u8]>,
) -> Result<()> {
    let handle = handle.borrow();

    let path = avatar_path(handle)?;

    create_dir_all_without_file_name(&path).await?;

    tokio::fs::write(path, bytes).await?;

    Ok(())
}

async fn fetch_remote_skin(
    handle: impl Borrow<AppHandle>,
    nick: impl Into<String>
) -> Result<Vec<u8>> {

    let handle = handle.borrow();

    let url = skin_url_from_nick(handle, nick);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()?;

    let bytes = client
        .get(url)
        .send()
        .await?
        .bytes()
        .await?;

    Ok(bytes.into())
}

async fn fetch_local_skin(
    handle: impl Borrow<AppHandle>,
    nick: impl Into<String>
) -> Result<Vec<u8>> {

    let handle = handle.borrow();

    let skin = offline_nick_to_skin_name(nick);

    let path = handle.path_resolver()
        .resolve_resource(format!("resources/skins/{skin}.png"))
        .ok_or(anyhow!("Failed to resource default_skin.png resource"))?;

    let bytes = tokio::fs::read(path).await?;

    Ok(bytes)
}


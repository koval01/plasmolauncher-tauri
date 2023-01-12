use std::{borrow::Borrow, path::PathBuf};

use reqwest::Url;
use tauri::{AppHandle, Manager};
use anyhow::{Result};

use crate::{utils::path::libraries::get_libraries_base_path};
use crate::state::config::LauncherConfig;

pub fn get_path_and_url_from_maven_name(
    maven_name: &String,
    handle: impl Borrow<AppHandle>,
    maven_url: Option<&Url>
) -> Result<(PathBuf, Url)> {

    let mut colon_split = maven_name.split(":");

    let dot_split = colon_split.next()
        .unwrap_or("")
        .split(".");

    let file_name = colon_split.clone().collect::<Vec<_>>().join("-");
    let file_name = format!("{}.jar", file_name);

    let relative_path = dot_split.chain(colon_split)
        .collect::<Vec<_>>();
        // .join("/");

    let mut path = get_libraries_base_path(handle.borrow())?;
    path.extend(&relative_path);
    path.push(&file_name);

    // dbg!(&path);

    let config = handle.borrow().state::<LauncherConfig>();

    let base_url = maven_url.unwrap_or_else(|| &config.fabric.maven_url);

    let url_relative_path = relative_path.join("/");

    let url = Url::parse(
        &format!("{base_url}{url_relative_path}/{file_name}")
    )?;

    // dbg!(&url);

    Ok((path, url))
}
use url2::{Url2, try_url2};

// use crate::state::models::config::LauncherConfig;

use crate::state::config::LauncherConfig;

use super::{system_info::{get_adoptium_os, get_adoptium_arch}, error::AdoptiumError};

pub fn get_binary_download_url(java_version: impl Into<String>, config: &LauncherConfig) -> Result<Url2, AdoptiumError> {

    let api_url = config.adoptium.api_url.clone();
    // let java_version = config.java_version;
    let os = get_adoptium_os()?;
    let arch = get_adoptium_arch()?;
    let java_version: String = java_version.into();

    let url = try_url2!(
        "{api_url}/binary/latest/{java_version}/ga/{os}/{arch}/jre/hotspot/normal/eclipse"
    )?;

    Ok(url)
}
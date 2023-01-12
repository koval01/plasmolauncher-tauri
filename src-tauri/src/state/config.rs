use std::path::PathBuf;

use include_path::include_path_str;
use plasmolauncher_common::models::{instance::Instance, loader::Loader};
use reqwest::Url;
use serde::{Deserialize, Serialize};

// use crate::instance::{Instance, Loader};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct LauncherConfig {
    pub launcher_name: String,
    pub launcher_version: String,
    pub window_title: String,
    pub minecraft_avatar: MinecraftAvatarConfig,
    pub plasmo: PlasmoConfig,
    pub metaserver: MetaserverConfig,
    pub adoptium: AdoptiumConfig,
    pub fabric: FabricConfig,
    pub mojang: MojangConfig,
    pub default_instance: Instance,
}

const LAUNCHER_CONFIG: &str = include_path_str!("..", "..", "LauncherConfig.toml");

impl LauncherConfig {
    pub fn from_file() -> Self {
        let config = toml::from_str::<Self>(LAUNCHER_CONFIG).unwrap();
        config
    }
}

impl Default for LauncherConfig {
    fn default() -> Self {
        LauncherConfig {
            launcher_name: "Plasmo".into(),
            launcher_version: "69".into(),
            window_title: "Plasmo Launcher".into(),
            minecraft_avatar: MinecraftAvatarConfig {
                api_url: Url::parse(
                    "https://rp.plo.su/avatar"
                ).unwrap(),
                query_params: "?w=8".into(),
            },
            plasmo: PlasmoConfig {
                client_id: "NRznkZ5YxMFTD2ReuszFbvGngnDygvC7c693Seav94k3PJVc".into(),
                oauth_url: Url::parse(
                    "https://rp.plo.su/oauth2/"
                ).unwrap(),
                api_url: Url::parse(
                    "https://rp.plo.su/api/"
                ).unwrap(),
            },
            metaserver: MetaserverConfig {
                api_url: Url::parse(
                    "https://launcher-meta.plo.su/"
                ).unwrap(),
            },
            adoptium: AdoptiumConfig {
                api_url: "https://api.adoptium.net/v3".into(),
            },
            mojang: MojangConfig {
                version_manifest_url: Url::parse(
                    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json"
                ).unwrap(),
                resources_url: Url::parse(
                    "https://resources.download.minecraft.net"
                ).unwrap()
            },
            fabric: FabricConfig {
                api_url: Url::parse("https://meta.fabricmc.net/v2").unwrap(),
                maven_url: Url::parse("https://maven.fabricmc.net/").unwrap(),
            },
            default_instance: Instance {
                name: "Default".into(),
                game_version: "1.19.2".into(),
                path: Some(PathBuf::from("default")),
                loader: Loader::Fabric("0.14.11".into()),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdoptiumConfig {
    pub api_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlasmoConfig {
    pub client_id: String,
    pub oauth_url: Url,
    pub api_url: Url,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaserverConfig {
    pub api_url: Url,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FabricConfig {
    pub api_url: Url,
    pub maven_url: Url,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MojangConfig {
    pub version_manifest_url: Url,
    pub resources_url: Url,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MinecraftAvatarConfig {
    pub api_url: Url,
    pub query_params: String,
}
use std::{collections::HashMap, process::Stdio};

use async_trait::async_trait;

use plasmolauncher_common::models::instance::Instance;
use plasmolauncher_common::models::loader::Loader;
use rayon::prelude::*;
use serde_json::json;
use tauri::{AppHandle, Manager};
use tokio::{process::Command, io::AsyncWriteExt, sync::{oneshot::channel}};
use tokio::fs::create_dir_all;
use tokio_util::sync::CancellationToken;

use tauri_plugin_synced_state::{SyncedToml, SyncState};

use crate::{task::{task::{ExecutableTask, TaskResult}, progress::{NamedProgress, Progress, ProgressState}}, external_api::{mojang::{models::{version_meta::VersionMeta, version_meta_inner::argument::{ParseArguments}}, cached_file::{CachedVersionFile}}, fabric::{models::loader_meta::FabricLoaderMeta, cached_file::CachedGameLoaderVersionFile}}, utils::{path::{libraries::get_libraries_base_path, assets::{get_assets_base_path}, java::get_java_binary_path, launcher::get_launcher_path}, pathbuf_to_string::ToOptionString}, state::{config::LauncherConfig, session::Session, preferences::{Preferences}}, instance::InstanceTrait};

pub struct LaunchGameTask {
    instance: Instance,
}

#[async_trait]
impl ExecutableTask for LaunchGameTask {
    async fn execute(
        &self,
        handle: AppHandle,
        _cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {

        progress.start("task.launch_game.title").await;
        progress.set_subtitle("task.launch_game.preparing").await;

        let version_meta = VersionMeta::get_cached(&handle, &self.instance.game_version).await?;
        let assets_path = get_assets_base_path(&handle)?;
        let game_path = self.instance.get_dir_path(&handle)?;
        let config = handle.state::<LauncherConfig>();

        let preferences = handle.state::<SyncedToml<Preferences>>().get().await;
        let session = &handle.state::<SyncedToml<Session>>().get().await;

        create_dir_all(&game_path).await?;

        // Fabric classpaths

        let fabric_classpaths = match &self.instance.loader {
            Loader::Vanilla => Vec::new(),
            Loader::Fabric(fabric_version) => {
                FabricLoaderMeta::get_cached(&handle, &self.instance.game_version, fabric_version)
                    .await?
                    .get_classpaths(&handle)?          
            },
        };

        // JVM args

        let launcher_jar_path = get_launcher_path(&handle)?;

        let jvm_classpath = version_meta.get_classpaths(&handle)?
            .into_par_iter()
            .chain(rayon::iter::once(launcher_jar_path))
            .chain(fabric_classpaths.clone())
            .flat_map(ToOptionString::to_string)
            .collect::<Vec<_>>()
            .join(CLASSPATH_SEPARATOR);

        let natives_path = get_libraries_base_path(&handle)?;

        let jvm_vars: HashMap<String, String> = serde_json::from_value(json!({
            "classpath": jvm_classpath,
            "natives_directory": natives_path,
            "launcher_name": config.launcher_name,
            "launcher_version": config.launcher_version,
        }))?;


        let memory_allocation = preferences
            .java
            .maximum_memory_allocation
            .get_arg();
        
        let plasmo_token = match session {
            Session::Plasmo(plasmo_session) => {
                vec![plasmo_session.get_token_arg()]
            },
            _ => Vec::new()
        };

        let jvm_args = version_meta.arguments
            .get_jvm_rules_satisfied()
            .parse_arguments(jvm_vars)?
            .into_iter()
            .chain(plasmo_token)
            .chain(std::iter::once(memory_allocation))
            .chain(std::iter::once(
                String::from("org.multimc.EntryPoint")
            ))
            .collect::<Vec<_>>();

        // dbg!(&jvm_args.join(" "));


        // dbg!(&jvm_args.join(" "));

        // Player Name

        let player_name = session
            .get_nick_or_default();

        // Game args

        let game_vars: HashMap<String, String> = serde_json::from_value(json!({
            "auth_player_name": player_name,
            "version_name": version_meta.id,
            "game_directory": &game_path,
            "assets_root": assets_path,
            "assets_index_name": version_meta.assets,
            "auth_access_token": "",
            "version_type": version_meta.r#type,
        }))?;

        let game_args = version_meta.arguments
            .get_game_rules_satisfied()
            .parse_arguments(game_vars)?;
        
        // dbg!(&game_args);

        // Main Class

        let main_class = match &self.instance.loader {
            Loader::Vanilla => version_meta.main_class.clone(),
            Loader::Fabric(fabric_version) => {
                let loader_meta = FabricLoaderMeta::get_cached(&
                    handle,
                    &self.instance.game_version,
                    fabric_version
                ).await?;
                loader_meta.launcher_meta.main_class.client.clone()
            }
        };

        // Onesix Args

        let onesix_classpaths = version_meta
            .get_classpaths(&handle)?
            .into_par_iter()
            .chain(fabric_classpaths)
            .flat_map(ToOptionString::to_string)
            .map(|path| format!("cp {path}"));

        let onesix_game_args = game_args.into_par_iter()
            .map(|arg| format!("param {arg}"));

        let natives_path = get_libraries_base_path(&handle)?
            .to_string()
            .unwrap_or_default();

        let onesix_other = vec![
            format!("natives {natives_path}"),
            format!("userName KPidS"),
            format!("sessionId 0"),
            format!("mainClass {main_class}"),
            format!("traits noapplet"),
            format!("launcher onesix"),
            format!("launch")
        ];

        let onesix_args = onesix_classpaths
            .chain(onesix_game_args)
            .chain(onesix_other)
            .collect::<Vec<_>>()
            .join("\n");

        // Launching

        // Err("Test error")?;

        let java_path = get_java_binary_path(&handle, "17")?;

        let mut child = Command::new(java_path)
            .stdout(Stdio::null())
            .stdin(Stdio::piped())
            .current_dir(game_path)
            .args(jvm_args)
            .spawn()?;

        child.stdin
            .as_mut()
            // #TODO: Remove unwrap
            .unwrap()
            .write_all(onesix_args.as_bytes())
            .await?;

        progress.set_subtitle_and_progress(
            "task.launch_game.launched",
            Progress::GameLaunched
        ).await;

        // #TODO: ????

        // handle.get_window("launcher")
        //     .and_then(|window| {
        //         window.request_user_attention(None).ok()
        //     });

        let (send, recv) = channel::<()>();

        let kill_game_event = handle.once_global("kill_game", move |_| {
            send.send(()).ok();
        });

        tokio::select! {
            _ = child.wait() => handle.unlisten(kill_game_event),
            _ = recv => child.kill().await?,
        }

        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LaunchGameError {
    #[error("Error while parsing arguments: {0}")]
    ArgumentParse(String)
}

impl LaunchGameTask {
    pub fn new(
        instance: Instance,
    ) -> Box<Self> {
        Box::new(Self {
            instance,
        })
    }
}

#[cfg(target_family = "unix")]
const CLASSPATH_SEPARATOR: &str = ":";

#[cfg(target_family = "windows")]
const CLASSPATH_SEPARATOR: &str = ";";
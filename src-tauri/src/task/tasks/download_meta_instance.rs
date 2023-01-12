use async_trait::async_trait;
use futures::{stream, StreamExt};

use tauri::{AppHandle, Manager};
use tauri_plugin_synced_state::{SyncState, SyncedToml};
use rayon::prelude::*;

use tokio_util::sync::CancellationToken;

use std::ops::Not;
use std::path::PathBuf;

use crate::external_api::metaserver::meta_instance::{get_meta_instance};
use crate::instance::InstanceTrait;
use crate::state::preferences::Preferences;
use crate::task::downloader::{DownloaderItem, Downloader};
use crate::task::progress::{ProgressState, NamedProgress};
use crate::task::task::{ExecutableTask, TaskResult};
use plasmolauncher_common::models::metaserver::meta_instance::MOD_SPLIT_KEY;

use crate::utils::check_path_exists_and_hash_matches::{check_path_exists_and_hash_matches};
use log::warn;

pub struct DownloadMetaInstanceTask {
    meta_instance_name: String,
}

impl DownloadMetaInstanceTask {
    pub fn new(
        meta_instance_name: impl Into<String>,
    ) -> Box<Self> {
        Box::new(Self {
            meta_instance_name: meta_instance_name.into(),
        })
    }
}

#[async_trait]
impl ExecutableTask for DownloadMetaInstanceTask {
    async fn execute(
        &self,
        handle: AppHandle,
        cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {
        progress.start("task.download_meta_instance.title").await;
        progress.set_subtitle("task.download_meta_instance.preparing").await;

        let meta_instance = match get_meta_instance(&self.meta_instance_name, &handle).await {
            Ok(meta_instance) => meta_instance,
            Err(error) => {
                warn!("Failed to get meta instance cache. Skipping meta instance validation. Error: {error}");
                return Ok(());
            },
        };

        let instance = &meta_instance.instance;

        let game_dir = instance.get_dir_path(&handle)?;

        let files = stream::iter(meta_instance.files)
            .map(|meta_file| {
                let mut path = game_dir.clone();
                path.extend(&meta_file.path);

                async move {
                    match meta_file.validate {
                        true => check_path_exists_and_hash_matches(
                            &path, 
                            &meta_file.artifact.sha1
                        )
                            .await
                            .unwrap_or(false),
                        false => path.exists()
                    }
                        .not()
                        .then_some(())?;

                    Some(DownloaderItem::new(
                        path,
                        meta_file.artifact.download_url,
                        meta_file.artifact.size
                    ))
                }
            })
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await
            .into_par_iter()
            .flatten()
            .collect::<Vec<_>>();

        let mut mods_folder_path = game_dir.clone();
        mods_folder_path.push("mods");

        tokio::fs::create_dir_all(&mods_folder_path).await?;

        let mut installed_mods = std::fs::read_dir(&mods_folder_path)?
            .flatten()
            .map(|entry| entry.path() )
            .filter(|path| path.is_file() )
            .filter_map(|path| {

                let file_name = path.file_name()?.to_str()?;

                let mut split = file_name
                    .strip_prefix(MOD_SPLIT_KEY)?
                    .split(MOD_SPLIT_KEY);

                let installed_mod = InstalledMod::new(
                    split.next()?,
                    split.next()?,
                    path.clone()
                );

                Some(installed_mod)
            })
            .collect::<Vec<_>>();

        // let meta_mods = 

        let optional_mods_prefs = handle.state::<SyncedToml<Preferences>>()
            .get()
            .await
            .optional_mods;

        let meta_mods = meta_instance.mods
            .into_par_iter()
            .filter(|meta_mod| {
                !meta_mod.optional ||
                *optional_mods_prefs
                    .get(&meta_mod.id)
                    .unwrap_or(&false)
            })
            .collect::<Vec<_>>();

        let meta_mods_installed = meta_mods
            .into_iter()
            .map(|meta| {
                let installed = installed_mods.par_iter()
                    .position_any(|installed| {
                        &installed.id == &meta.id
                    })
                    .and_then(|position| {
                        Some(installed_mods.swap_remove(position))
                    });
                (meta, installed)
            })
            .collect::<Vec<_>>();

        let mods = stream::iter(meta_mods_installed)
            .map(|(meta, installed)| {
                let mods_folder_path = &mods_folder_path;
                async move {
                    if let Some(installed) = installed {
                        if installed.version != meta.version {
                            tokio::fs::remove_file(&installed.path).await.ok();
                        }
                        let check = check_path_exists_and_hash_matches(
                            &installed.path,
                            &meta.artifact.sha1
                        ).await
                            .unwrap_or(false);

                        if check { return None; }
                    }

                    let mut path = mods_folder_path.clone();
                    path.push(meta.jar_name());

                    Some(DownloaderItem::new(
                        path,
                        meta.artifact.download_url,
                        meta.artifact.size
                    ))
                }  
            })
            .buffer_unordered(10)
            .collect::<Vec<_>>()
            .await
            .into_par_iter()
            .flatten()
            .collect::<Vec<_>>();

        let downloader_items = files.into_par_iter()
            .chain(mods)
            .collect::<Vec<_>>();

        Downloader::download(downloader_items, progress.clone(), cancellation_token).await?;

        progress.set_subtitle("task.download_meta_instance.deleting").await;

        // Mods that were not filtered out by swap_remove are the mods that are installed, but not preset in the mod meta, therefore you need to delete these mods
        stream::iter(installed_mods)
            .for_each_concurrent(10, |installed_mod| async move {
                tokio::fs::remove_file(installed_mod.path).await.ok();
            })
            .await;

        Ok(())
    }
}

#[derive(Debug)]
struct InstalledMod {
    id: String,
    version: String,
    path: PathBuf,
}

impl InstalledMod {
    pub fn new(id: impl Into<String>, version: impl Into<String>, path: PathBuf) -> Self {
        Self {
            id: id.into(),
            version: version.into(),
            path,
        }
    }
}
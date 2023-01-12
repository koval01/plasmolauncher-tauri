use std::{ops::Not};

use async_trait::async_trait;
use plasmolauncher_common::models::instance::Instance;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use tauri::{AppHandle, Manager};
use futures::{StreamExt, stream};
use tauri_plugin_synced_state::SyncState;

use tokio_util::sync::CancellationToken;

use crate::{task::{task::{ExecutableTask, TaskResult}, progress::{NamedProgress, ProgressState}, downloader::{DownloaderItem, Downloader}}, external_api::mojang::{models::{asset_index::AssetIndex, version_meta::VersionMeta}, cached_file::CachedVersionFile}, utils::{path::assets::{get_assets_object_path, assets_relative_path}, check_path_exists_and_hash_matches::check_path_exists_and_hash_matches}, state::config::LauncherConfig};

pub struct DownloadAssetsTask {
    instance: Instance,
    validation: bool,
}

#[async_trait]
impl ExecutableTask for DownloadAssetsTask {
    async fn execute(
        &self,
        handle: AppHandle, 
        cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {

        progress.start("task.download_assets.title").await;
        progress.set_subtitle("task.download_assets.validating").await;

        let version_meta = VersionMeta::get_cached(&handle, &self.instance.game_version).await?;

        let assets_index = AssetIndex::get_cached(&handle, version_meta.asset_index.id)
            .await?
            .objects
            .into_values()
            .collect::<Vec<_>>();

        let downloader_items = stream::iter(assets_index)
            .map(|object| {

                let handle = handle.clone();
                let validation = self.validation.clone();

                async move {
                    let path = get_assets_object_path(&handle, &object.hash).ok()?;

                    match validation {
                        true => check_path_exists_and_hash_matches(&path, &object.hash)
                                .await
                                .unwrap_or(false),
                        false => path.exists(),
                    }
                        .not()
                        .then_some(())?;

                    let relative_path = assets_relative_path(&object.hash);
                    let relative_path = relative_path.to_str()?;

                    let url = handle.state::<LauncherConfig>()
                        .mojang
                        .resources_url
                        .join(relative_path)
                        .ok()?;

                    Some(DownloaderItem::new(path, url, object.size))
                }
            })
            .buffer_unordered(50)
            .collect::<Vec<_>>()
            .await
            .into_par_iter()
            .flatten()
            .collect::<Vec<_>>();

        Downloader::download(
            downloader_items,
            progress,
            cancellation_token,
        ).await?;

        Ok(())
    }
}

impl DownloadAssetsTask {
    pub fn new(
        instance: Instance,
        validation: bool,
    ) -> Box<Self> {
        Box::new(Self {
            instance,
            validation,
        })
    }
}
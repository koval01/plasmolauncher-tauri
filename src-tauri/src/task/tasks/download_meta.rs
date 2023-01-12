use async_trait::async_trait;
use futures::{TryFutureExt};
use plasmolauncher_common::models::{instance::Instance, loader::Loader};
use tauri::{AppHandle};
use tauri_plugin_synced_state::SyncState;
use tokio_util::sync::CancellationToken;

use crate::{task::{task::{ExecutableTask, TaskResult}, progress::{NamedProgress, ProgressState}}, external_api::{mojang::{models::{version_manifest::{Version, VersionManifest}, version_meta::VersionMeta, asset_index::AssetIndex}, cached_file::{CachedVersionFile, CachedFile}}, fabric::{models::loader_meta::FabricLoaderMeta, cached_file::CachedGameLoaderVersionFile}}, utils::into_future::IntoFuture};

pub struct DownloadMetaTask {
    instance: Instance,
}

#[async_trait]
impl ExecutableTask for DownloadMetaTask {
    async fn execute(
        &self,
        handle: AppHandle,
        _cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>,
    ) -> TaskResult {

        progress.start("task.download_meta.title").await;

        let version = VersionManifest::get_safe(&handle).await?
            .get_version(&self.instance.game_version)
            .into_future()
            .or_else(|_| {
                let handle = &handle;
                let self_version = &self.instance.game_version;
                async move {
                    VersionManifest::download(handle).await?;
                    let version = VersionManifest::get_cached(handle).await?
                        .get_version(self_version)?;
                    Ok::<Version, anyhow::Error>(version)
                }
            })
            .await?;

        let version_meta = VersionMeta::validate_or_download(
            &handle,
            &self.instance.game_version,
            &version.sha1,
            version.url.as_str()
        ).await?;

        AssetIndex::validate_or_download(
            &handle,
            version_meta.asset_index.id,
            version_meta.asset_index.sha1,
            version_meta.asset_index.url
        ).await?;

        if let Loader::Fabric(fabric_version) = &self.instance.loader {
            FabricLoaderMeta::validate_or_download(
                &handle,
                &self.instance.game_version,
                fabric_version
            ).await?;
        }

        Ok(())
    }
}

impl DownloadMetaTask {
    pub fn new(
        instance: Instance,
    ) -> Box<Self> {
        Box::new(Self {
            instance,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum DownloadMetaError {
    #[error("Version not found: {0}")]
    VersionNotFound(String),
}

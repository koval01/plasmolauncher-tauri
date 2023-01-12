use std::ops::Not;


use async_trait::async_trait;
use plasmolauncher_common::models::instance::Instance;
use plasmolauncher_common::models::loader::Loader;
use tauri::{AppHandle};
use rayon::prelude::*;
use tauri_plugin_synced_state::SyncState;

use tokio_util::sync::CancellationToken;
use std::fs::read;

use crate::external_api::fabric::cached_file::CachedGameLoaderVersionFile;
use crate::external_api::fabric::models::loader_meta::FabricLoaderMeta;
use crate::instance::InstanceTrait;
use crate::task::progress::ProgressState;
use crate::utils::hash_from_path::{hash_from_bytes, hash_from_path};
use crate::{task::{downloader::{Downloader, DownloaderItem}, progress::{NamedProgress}, task::{TaskResult, ExecutableTask}}, utils::{path::libraries::get_libraries_base_path}, external_api::mojang::{models::{version_meta::VersionMeta}, cached_file::CachedVersionFile}};

// use futures_util::{stream, StreamExt};

pub struct DownloadLibrariesTask {
    instance: Instance,
    validation: bool,
}

impl DownloadLibrariesTask {
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

// #TODO Validation optimisation
#[async_trait]
impl ExecutableTask for DownloadLibrariesTask {
    async fn execute(
        &self,
        handle: AppHandle,
        cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {

        progress.start("task.download_libraries.title").await;
        progress.set_subtitle("task.download_libraries.validating").await;

        let version_meta = VersionMeta::get_cached(&handle, &self.instance.game_version).await?;
        let base_libraries_path = get_libraries_base_path(&handle)?;

        //

        let client_jar_path = self.instance.get_client_jar_path(&handle)?;
        let client_jar_artifact = version_meta.downloads.client;

        let client_jar_download_item = match self.validation {
            false => client_jar_path.exists().not(),
            true => {
                hash_from_path(&client_jar_path).await? != client_jar_artifact.sha1
            }
        }.then_some(
            DownloaderItem::new(
                client_jar_path,
                client_jar_artifact.url,
                client_jar_artifact.size
            )
        );

        //

        let fabric_download_items = match &self.instance.loader {
            Loader::Vanilla => Vec::new(),
            Loader::Fabric(fabric_version) => {
                let loader_meta = FabricLoaderMeta::get_cached(&
                    handle,
                    &self.instance.game_version,
                    fabric_version
                ).await?;
                
                loader_meta.get_downloader_items(handle)?
            },
        }
            .into_par_iter()
            .filter(|item| item.path.exists().not())
            .collect::<Vec<_>>();

        // dbg!(&fabric_download_items);

        //

        let downloader_items = version_meta.libraries
            .into_par_iter()
            .filter(|library| library.check_if_rules_satisfied() )
            // Filter need downloading
            .filter(|library| {

                let path = library.get_local_path(&base_libraries_path);

                if !self.validation { return !path.exists() };
            
                let artifact = &library.downloads.artifact;
                let Ok(bytes) = read(path) else { return true };
                let local_hash = hash_from_bytes(bytes);

                local_hash != artifact.sha1
            })
            .map(|library| {
                let path = library.get_local_path(&base_libraries_path);
                let artifact = library.downloads.artifact;
                DownloaderItem::new(path, artifact.url, artifact.size)
            })
            .chain(client_jar_download_item)
            .chain(fabric_download_items)
            .collect::<Vec<DownloaderItem>>();

        Downloader::download(
            downloader_items,
            progress,
            cancellation_token,
        ).await?;

        Ok(())
    }
}
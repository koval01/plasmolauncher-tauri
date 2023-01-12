use std::{path::{PathBuf}};

use async_trait::async_trait;
use fs_extra::dir::CopyOptions;
use futures_util::{StreamExt};
use plasmolauncher_common::models::instance::Instance;
use tauri::{AppHandle, Manager};
use tauri_plugin_synced_state::SyncState;
use tokio::{fs::{File, create_dir_all, remove_dir_all}, io::AsyncWriteExt};
use tokio_util::sync::CancellationToken;

use crate::{utils::{unpack::unpack, path::java::{check_if_java_binary_exists, get_java_base_path, get_unpacked_contents_paths}, get_response_file_name::FileName}, task::{task::{ExecutableTask, TaskResult}, progress::{NamedProgress, Progress, ProgressState}}, external_api::{adoptium::api::get_binary_download_url, mojang::{models::version_meta::{VersionMeta}, cached_file::CachedVersionFile}}, state::config::LauncherConfig};

pub struct DownloadJavaTask {
    instance: Instance,
}

impl DownloadJavaTask {
    pub fn new(
        instance: Instance,
    ) -> Box<Self> {
        Box::new(Self {
            instance,
        })
    }
}

#[async_trait]
impl ExecutableTask for DownloadJavaTask {
    async fn execute(
        &self,
        handle: AppHandle,
        _cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {

        progress.start("task.download_java.title").await;
        progress.set_subtitle("task.download_java.checking").await;

        let version_meta = VersionMeta::get_cached(&handle, &self.instance.game_version).await?;

        let java_version = version_meta.java_version.major_version.to_string();

        let check = check_if_java_binary_exists(&handle, &java_version)?;
        
        if check { return Ok(()) };

        progress.set_subtitle_and_progress(
            "task.download_java.preparing",
            Progress::Indefinate
        ).await;

        let base_path = get_java_base_path(&handle, &java_version)?;

        remove_dir_all(&base_path).await.ok();

        create_dir_all(&base_path).await?;

        let config = handle.state::<LauncherConfig>();

        let url = get_binary_download_url(&java_version, &config)?;

        let client = reqwest::Client::new();

        let res = client.get(url.as_str())
            .send()
            .await?;

        let total_size = res.content_length()
            .ok_or("Failed to get content length")?;

        let mut file_path = PathBuf::from(&base_path);
        file_path.push(res.file_name());

        let mut file = File::create(&file_path).await?;
        let mut downloaded: u64 = 0;
        let mut stream = res.bytes_stream();

        progress.set_subtitle_and_progress(
            "task.download_java.downloading",
            Progress::Bytes(0, total_size)
        ).await;

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;
            progress.set_progress(Progress::Bytes(downloaded, total_size)).await;
        }

        progress.set_subtitle_and_progress(
            "task.download_java.unpacking",
            Progress::Indefinate
        ).await;

        // #TODO: Unpacking optimisation, takes a lot of time

        let unpacked_dir_path = unpack(file_path).await?;

        #[cfg(target_os = "linux")]
        {
            use crate::utils::path::java::get_unpacked_legal_path;
            let legal = get_unpacked_legal_path(&unpacked_dir_path).await?;
            remove_dir_all(legal).await.ok();
        }

        let contents = get_unpacked_contents_paths(&unpacked_dir_path).await?;

        fs_extra::move_items(&contents, base_path, &CopyOptions::new())?;

        remove_dir_all(unpacked_dir_path).await?;

        Ok(())
    }
}

// pub fn lii
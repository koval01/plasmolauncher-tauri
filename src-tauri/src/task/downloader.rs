
use std::{path::PathBuf};

use anyhow::Result;

use futures::prelude::*;
use futures::stream;
use rayon::prelude::*;

use tauri_plugin_synced_state::SyncState;
use tokio::fs::{File};
use tokio::io::AsyncWriteExt;
use tokio_util::sync::CancellationToken;
use url::Url;
// use anyhow::Result;

use crate::utils::path::create_dir_all_without_file_name::create_dir_all_without_file_name;

use super::progress::{NamedProgress, ProgressState};
use super::{progress::Progress};

pub struct Downloader;

impl Downloader {
    pub async fn download(
        items: Vec<DownloaderItem>,
        progress: SyncState<'_, NamedProgress>,
        cancellation_token: CancellationToken
    ) -> Result<()> {

        if items.len() <= 0 { return Ok(()) };

        let total_size = items.par_iter().map(|item| item.size).sum();

        progress.set_subtitle_and_progress(
            "task.download.downloading",
            Progress::Bytes(0, total_size)
        ).await;

        stream::iter(items)
            .map(|item| {
                let progress = progress.clone();
                let cancellation_token = cancellation_token.clone();
                async move {
                    if cancellation_token.is_cancelled() { return Ok(()); }
                    download_item(item, total_size, progress, cancellation_token).await
                }
            })
            .buffer_unordered(10)
            .try_collect::<Vec<_>>()
            .await?;

        Ok(())
    }
}

async fn download_item(
    item: DownloaderItem,
    _total_size: u64,
    progress: SyncState<'_, NamedProgress>,
    cancellation_token: CancellationToken
) -> Result<()> {

    let res = reqwest::get(item.url).await?;

    create_dir_all_without_file_name(&item.path).await?;

    let mut file = File::create(&item.path).await?;
    let mut stream = res.bytes_stream();

    while let Some(bytes) = stream.next().await {

        let bytes = bytes?;

        let do_stuff_future = async { tokio::try_join!(
            async {
                file.write_all(&bytes).await
            },
            async {
                progress.mutate(|progress| {
                    progress.add_bytes(bytes.len())
                }).await;
                Ok(())
            }
        )};

        tokio::select! {
            res = do_stuff_future => {
                res?;
            },
            _ = cancellation_token.cancelled() => {
                drop(file);
                tokio::fs::remove_file(&item.path).await?;
                return Ok(());
            }
        }

        // if cancellation_token.is_cancelled() {
        //     drop(file);
        //     tokio::fs::remove_file(&item.path).await?;
        //     return Ok(());
        // }
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct DownloaderItem {
    pub path: PathBuf,
    pub url: Url,
    pub size: u64,
}

impl DownloaderItem {
    pub fn new(path: PathBuf, url: Url, size: u64) -> Self {
        Self {
            path,
            url,
            size
        }
    }
}

// impl From<LibrariesItem> for DownloaderItem {
//     fn from(value: LibrariesItem) -> Self {
        
//     }
// }
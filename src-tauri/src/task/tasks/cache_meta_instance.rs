use async_trait::async_trait;
use log::info;
use tauri::{AppHandle};
use tauri_plugin_synced_state::SyncState;

use tokio_util::sync::CancellationToken;

use crate::external_api::metaserver::meta_instance::cache_meta_instance;
use crate::task::progress::{ProgressState, NamedProgress};
use crate::task::task::{ExecutableTask, TaskResult};

// use futures_util::{stream, StreamExt};

pub struct CacheMetaInstanceTask {
    meta_instance_name: String,
}

impl CacheMetaInstanceTask {
    pub fn new(
        meta_instance_name: impl Into<String>,
    ) -> Box<Self> {
        Box::new(Self {
            meta_instance_name: meta_instance_name.into(),
        })
    }
}

// #TODO Validation optimisation
#[async_trait]
impl ExecutableTask for CacheMetaInstanceTask {
    async fn execute(
        &self,
        handle: AppHandle,
        _cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult {

        progress.start("task.cache_meta_instance.title").await;

        match cache_meta_instance(&self.meta_instance_name, handle).await {
            Ok(_) => {},
            Err(error) => info!("Failed to cache meta instance: {error}"),
        }

        Ok(())
    }
}
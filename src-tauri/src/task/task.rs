use async_trait::async_trait;
use tauri::{AppHandle};
use tauri_plugin_synced_state::{SyncState};

use tokio_util::sync::CancellationToken;

use super::{progress::{NamedProgress}};

pub type TaskResult = Result<(), Box<dyn std::error::Error + Sync + Send>>;

// pub struct Task {
//     pub handle: AppHandle,
//     pub progress: NamedProgress,
// }

// impl Task {
//     pub fn new(
//         handle: AppHandle,
//         progress: NamedProgress,
//     ) -> Self {
//         Self {
//             handle,
//             progress,
//         }
//     }

//     fn emit_update(&self) {
//         self.handle.emit_all(
//             "task-progress-update",
//             self.progress.clone()
//         ).ok();
//     }

//     pub fn set_progress(&mut self, progress: Progress) {
//         self.progress.set_progress(progress);
//         self.emit_update();
//     }

//     pub fn set_subtitle(&mut self, subtitle: impl Into<String>) {
//         self.progress.set_subtitle(subtitle.into());
//         self.emit_update();
//     }

//     pub fn set_subtitle_and_progress(
//         &mut self, subtitle: impl Into<String>,
//         progress: Progress
//     ) {
//         self.progress.set_progress(progress);
//         self.progress.set_subtitle(subtitle.into());
//         self.emit_update();
//     }
// }


#[async_trait]
pub trait ExecutableTask {
    async fn execute(
        &self,
        handle: AppHandle,
        cancellation_token: CancellationToken,
        progress: SyncState<'_, NamedProgress>
    ) -> TaskResult;
}
use std::{sync::Arc, collections::VecDeque};


use derive_more::Deref;

use tauri::{async_runtime::Mutex, AppHandle, Manager};
use tauri_plugin_synced_state::Synced;
use tokio_util::sync::CancellationToken;

use super::{task::{ExecutableTask}, progress::NamedProgress};

#[derive(Deref, Clone)]
pub struct TaskQueue(
    pub Arc<Mutex<TaskQueueInner>>
);

impl TaskQueue {
    pub fn new(handle: AppHandle) -> Self {
        Self (
            Arc::new(
                Mutex::new(
                    TaskQueueInner {
                        handle,
                        processing: false,
                        queue: VecDeque::new(),
                    }
                )
            )
        )
    }
}

pub struct TaskQueueInner
{
    handle: AppHandle,
    processing: bool,
    queue: VecDeque<Box<dyn ExecutableTask + Sync + Send>>
}

impl TaskQueue {

    pub async fn get_processing(&self) -> bool {
        let lock = self.lock().await;
        lock.processing
    }

    pub async fn add_task(&self, task: Box<dyn ExecutableTask + Send + Sync>) {
        let mut lock = self.lock().await;
        lock.queue.push_back(task);
    }

    async fn get_task(&self) -> Option<Box<dyn ExecutableTask + Sync + Send>> {
        let mut lock = self.lock().await;
        let task = lock.queue.pop_front()?;
        Some(task)
    }

    pub async fn clear_queue(&self) {
        let mut lock = self.lock().await;
        lock.queue = VecDeque::new();
    }

    pub async fn start_queue(&self) {
        let mut lock = self.lock().await;
        if lock.processing { return };
        lock.processing = true;
        lock.handle.emit_all("task-queue-start", "").ok();

        let cancellation_token = CancellationToken::new();
        let cancellation_token_clone = cancellation_token.clone();

        let handle = lock.handle.clone();

        drop(lock);

        let cancel_queue_event = handle
            .once_global("cancel_task_queue", move |_| {
                cancellation_token_clone.cancel()
            });

        while let Some(task) = self.get_task().await {
            let lock = self.lock().await;
            let handle = lock.handle.clone();

            drop(lock);

            let progress = handle.state::<Synced<NamedProgress>>();

            let res = task.execute(
                handle.clone(),
                cancellation_token.clone(),
                progress
            ).await;

            if let Err(error) = res {
                handle.emit_all("global-error", error.to_string()).ok();
                self.clear_queue().await;
                break;
            }

            if cancellation_token.is_cancelled() {
                self.clear_queue().await;
                break;
            }
        }

        let mut lock = self.lock().await;
        lock.processing = false;
        lock.handle.unlisten(cancel_queue_event);
        lock.handle.state::<Synced<NamedProgress>>().reset().await;
        lock.handle.emit_all("task-queue-finish", ()).ok();
        drop(lock);
    }

}
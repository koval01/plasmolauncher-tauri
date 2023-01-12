use std::time::Duration;

use async_trait::async_trait;
use tauri::{AppHandle};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

use crate::task::{task::{Task, ExecutableTask, TaskResult}, progress::{NamedProgress, Progress}};

// use super::{task::{Task, ExecutableTask, TaskResult}, progress::{NamedProgress, Progress}};

pub struct TestTask {
    task: Task,
    total: u64,
}

impl TestTask {
    pub fn new(
        handle: AppHandle,
        progress: NamedProgress,
        total: u64,
    ) -> Box<Self> {
        Box::new(Self {
            task: Task::new(handle, progress),
            total,
        })
    }
}

#[async_trait]
impl ExecutableTask for TestTask {
    async fn execute(
        &mut self,
        _handle: AppHandle,
        _cancellation_token: CancellationToken
    ) -> TaskResult {

        for index in 1..self.total {
            sleep(Duration::from_millis(8)).await;
            self.task.set_progress(Progress::Bytes(index as u64 + 1, self.total));
        }

        Ok(())
    }
}
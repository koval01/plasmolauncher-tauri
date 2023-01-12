use std::cmp::min;

use serde::{Serialize, Deserialize};
use tauri_plugin_synced_state::SyncState;
use ts_rs::TS;
use async_trait::async_trait;

#[derive(Serialize, TS, Clone, Deserialize, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Progress {
    Bool(bool),
    Percentage(f32),
    Bytes(u64, u64),
    Count(u16, u16),
    Indefinate,
    GameLaunched,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct NamedProgress {
    pub title: String,
    pub subtitle: Option<String>,
    pub progress: Progress
}

impl Default for NamedProgress {
    fn default() -> Self {
        Self {
            title: "task.idle.title".into(),
            subtitle: None,
            progress: Progress::None,
        }
    }
}

impl NamedProgress {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
            progress: Progress::Indefinate,
        }
    }

    pub fn set_subtitle(&mut self, subtitle: String) {
        self.subtitle = Some(subtitle);
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn remove_subtitle(&mut self) {
        self.subtitle = None;
    }

    pub fn set_progress(&mut self, progress: Progress) {
        self.progress = progress;
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        let Progress::Bytes(current, max) = self.progress else { return };
        let new = min(current + bytes as u64, max);
        self.set_progress(Progress::Bytes(new, max));
    }
}

#[async_trait]
pub trait ProgressState {
    async fn set_progress(&self, progress: Progress);

    async fn set_subtitle(&self, subtitle: impl Into<String> + Sync + Send);

    async fn start(&self, title: impl Into<String> + Sync + Send);

    async fn set_subtitle_and_progress(
        &self,
        subtitle: impl Into<String> + Sync + Send,
        progress: Progress
    );
}

#[async_trait]
impl ProgressState for SyncState<'_, NamedProgress> {
    async fn set_progress(&self, progress: Progress) {
        self.mutate(move |named_progress| {
            named_progress.set_progress(progress.clone());
        }).await;
    }

    async fn set_subtitle(&self, subtitle: impl Into<String> + Sync + Send) {
        let subtitle: String = subtitle.into();
        self.mutate(move |named_progress| {
            named_progress.set_subtitle(subtitle.clone());
        }).await;
    }

    async fn start(&self, title: impl Into<String> + Sync + Send) {
        let title: String = title.into();
        self.mutate(move |named_progress| {
            named_progress.set_title(title.clone());
            named_progress.set_progress(Progress::Indefinate);
        }).await;
    }

    async fn set_subtitle_and_progress(
        &self,
        subtitle: impl Into<String> + Sync + Send,
        progress: Progress
    ) {
        let subtitle: String = subtitle.into();
        self.mutate(move |named_progress| {
            named_progress.set_progress(progress.clone());
            named_progress.set_subtitle(subtitle.clone());
        }).await;
    }
}
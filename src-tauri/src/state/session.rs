use serde::{Serialize, Deserialize};
use tauri_plugin_synced_state::SyncStateToml;
use ts_rs::TS;
use async_trait::async_trait;
// use crate::state::synced_state::{SyncedStateToml};

#[async_trait]
pub trait SessionState {
    async fn set_offline_session(
        &self,
        nick: impl Into<String> + Sync + Send,
    );

    async fn set_plasmo_session(
        &self,
        nick: impl Into<String> + Sync + Send,
        token: impl Into<String> + Sync + Send,
    );

    async fn clear_session(
        &self,
    );
}

#[async_trait]
impl SessionState for SyncStateToml<'_, Session> {
    async fn set_offline_session(
        &self,
        nick: impl Into<String> + Send + Sync,
    ) {
        let nick: String = nick.into();
        self.mutate(|session| {
            *session = Session::Offline(
                OfflineSession::new(&nick)
            );
        }).await;
    }

    async fn set_plasmo_session(
        &self,
        nick: impl Into<String> + Sync + Send,
        token: impl Into<String> + Sync + Send,
    ) {
        let nick: String = nick.into();
        let token: String = token.into();
        self.mutate(|session| {
            *session = Session::Plasmo(
                PlasmoSession::new(&nick, &token)
            );
        }).await;
    }

    async fn clear_session(
        &self,
    ) {
        self.mutate(|session| {
            *session = Session::None;
        }).await;
    }
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
#[serde(tag = "type", content = "content", rename_all = "lowercase")]
pub enum Session {
    None,
    Offline(OfflineSession),
    Plasmo(PlasmoSession),
}

impl Session {
    pub fn get_nick(&self) -> Option<String> {
        match self {
            Self::None => None,
            Self::Offline(session) => Some(session.nick.clone()),
            Self::Plasmo(session) => Some(session.nick.clone()),
        }
    }

    pub fn get_nick_or_default(&self) -> String {
        self.get_nick().unwrap_or("Player".into())
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct OfflineSession {
    pub nick: String,
}

impl OfflineSession {
    pub fn new(nick: impl Into<String>) -> Self {
        Self {
            nick: nick.into(),
        }
    }
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct PlasmoSession {
    pub nick: String,
    pub token: String,
}

impl PlasmoSession {
    pub fn new(
        nick: impl Into<String>,
        token: impl Into<String>,
    ) -> Self {
        Self {
            nick: nick.into(),
            token: token.into(),
        }
    }

    pub fn get_token_arg(&self) -> String {
        let token = &self.token;
        format!("-Dplasmo.token={token}")
    }
}
use std::borrow::Borrow;

use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};
use uuid::Uuid;
use anyhow::{Result, anyhow};

use crate::state::config::LauncherConfig;

use super::error::PlasmoError;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub status: bool,
    pub data: Option<UserData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub id: u32,
    pub discord_id: String,
    pub discord_name: String,
    pub nick: String,
    pub uuid: String,
    pub banned: bool,
    pub on_server: bool,
    pub access: bool,
}

impl User {
    pub async fn from_token(
        handle: impl Borrow<AppHandle>,
        token: impl Into<String>
    ) -> Result<Self, PlasmoError> {

        let handle = handle.borrow();
        let token: String = token.into();

        let config = handle.state::<LauncherConfig>();

        let base_url = &config.plasmo.api_url;
        let url = format!("{base_url}user");

        let client = reqwest::Client::new();
        
        let user = client.get(url)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await?
            .json::<Self>()
            .await?;

        if let Some(data) = &user.data {
            if !data.access {
                Err(PlasmoError::NotAuthorized)?
            }
        }

        Ok(user)
    }

    pub fn get_nick(&self) -> Result<String, PlasmoError> {
        let nick = &self.data
            .as_ref()
            .ok_or(PlasmoError::UserNotFound)?
            .nick;

        Ok(nick.to_string())
    }
}


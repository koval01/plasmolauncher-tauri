use serde::{Deserialize, Serialize};

use super::libraries::{Libraries, MainClass};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LauncherMeta {
    pub version: u32,
    pub libraries: Libraries,
    pub main_class: MainClass,
}
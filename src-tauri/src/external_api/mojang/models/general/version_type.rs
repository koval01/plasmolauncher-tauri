use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VersionType {
    Snapshot,
    Release,
    OldBeta,
    OldAlpha,
}
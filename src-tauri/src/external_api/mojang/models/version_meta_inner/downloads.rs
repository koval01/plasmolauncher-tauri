use serde::{Serialize, Deserialize};
use super::artifact::Artifact;

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    pub client: Artifact,
    pub client_mappings: Option<Artifact>,
    pub server: Artifact,
    pub server_mappings: Option<Artifact>,
}
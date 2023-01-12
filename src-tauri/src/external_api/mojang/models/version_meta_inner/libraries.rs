use std::path::PathBuf;

use rayon::prelude::*;
use serde::{Serialize, Deserialize};

use super::{rules::RulesItem, artifact::ArtifactWithPath};

#[derive(Debug, Serialize, Deserialize)]
pub struct LibrariesItem {
    pub downloads: LibrariesDownloads,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<RulesItem>>,
    pub name: String,
}

impl LibrariesItem {
    pub fn check_if_rules_satisfied(&self) -> bool {

        let rules_check = self.rules.as_ref().and_then(|rules| {
            let rules_check = rules.par_iter()
                .all(|rule| rule.check_if_satisfied());

            Some(rules_check)
        }).unwrap_or(true);

        return rules_check;
    }

    pub fn get_local_path(&self, base_path: &PathBuf) -> PathBuf {
        let mut library_path = PathBuf::from(&base_path);
        library_path.extend(&self.downloads.artifact.path);
        
        library_path
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibrariesDownloads {
    pub artifact: ArtifactWithPath,
}

use std::{path::PathBuf, collections::HashMap};

use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct Preferences {
    pub data_directory: Option<PathBuf>,
    pub java: JavaPreferences,
    pub optional_mods: HashMap<String, bool>
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            data_directory: None,
            java: JavaPreferences {
                maximum_memory_allocation: MemoryAllocation::_2G
            },
            optional_mods: HashMap::<String, bool>::new(),
        }
    }
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub struct JavaPreferences {
    pub maximum_memory_allocation: MemoryAllocation,
}

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export)]
pub enum MemoryAllocation {
    _1G,
    _2G,
    _4G,
    _6G,
    _8G,
    // Custom(String)
}

impl MemoryAllocation {
    pub fn get_arg(&self) -> String {
        let value = match self {
            MemoryAllocation::_1G => "1G",
            MemoryAllocation::_2G => "2G",
            MemoryAllocation::_4G => "4G",
            MemoryAllocation::_6G => "6G",
            MemoryAllocation::_8G => "8G",
            // MemoryAllocation::Custom(string) => string.as_str(),
        };

        format!("-Xmx{value}")
    }
}
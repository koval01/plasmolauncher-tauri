use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum StringOrStringVec {
    String(String),
    StringVec(Vec<String>),
}
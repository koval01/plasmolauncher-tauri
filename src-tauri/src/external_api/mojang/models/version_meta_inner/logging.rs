use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Logging {
    client: LoggingClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggingClient {
    argument: String,
    file: LoggingClientFile,
    r#type: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoggingClientFile {
    id: String,
    sha1: String,
    size: u64,
    url: Url,
}
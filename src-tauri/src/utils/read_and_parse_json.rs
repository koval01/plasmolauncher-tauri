use std::{path::Path};

use serde::{Deserialize};
use tokio::fs::read_to_string;
use anyhow::Result;

pub async fn read_and_parse_json<T: for<'a> Deserialize<'a>>(
    path: impl AsRef<Path>,
) -> Result<T> {

    let string = read_to_string(path).await?;
    let version_manifest = serde_json::from_str(string.as_str())?;

    Ok(version_manifest)
}
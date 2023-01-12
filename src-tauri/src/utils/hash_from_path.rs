use std::path::Path;

use anyhow::Result;
use tokio::fs::read;

use sha1::{Sha1, Digest};

pub async fn hash_from_path(path: impl AsRef<Path>) -> Result<String> {
    let bytes = read(path).await?;
    Ok(hash_from_bytes(bytes))
}

// pub fn hash_from_path_sync(path: impl AsRef<Path>) -> Result<String> {
//     let mut hasher = Sha1::new();
//     let bytes = std::fs::read(path)?;

//     hasher.update(bytes);
//     let result = hasher.finalize();

//     Ok(hex::encode(result))
// }

pub fn hash_from_bytes(bytes: Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    hex::encode(result)
}
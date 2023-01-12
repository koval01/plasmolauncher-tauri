#[derive(thiserror::Error, Debug)]
pub enum TauriError {
    #[error("Can't find app data directory")]
    DataDir,
}
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdoptiumError {
    #[error("Error when parsing url: {0}")]
    Url2(#[from] url2::Url2Error),
    #[error("Invalid system info for: {0}")]
    InvalidSystemInfo(String),
    #[error("Unknown error")]
    Unknown,
}
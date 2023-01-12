use std::path::PathBuf;

#[cfg(target_family = "windows")]
pub async fn unpack(archive_path: PathBuf) -> Result<PathBuf, UnpackError> {

    use std::fs::File;
    use tokio::fs::{remove_file};
    use zip_extensions::read::ZipArchiveExtensions;

    const EXTENSION: &str = ".zip";

    let archive_name = archive_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or(UnpackError::Parse("Path archive_name".into()))?;

    if !archive_name.ends_with(EXTENSION) {
        Err(UnpackError::Extension(archive_name.into(), EXTENSION.into()))?
    }

    let folder_name = archive_name.chars().take(
        archive_name.len() - EXTENSION.len()
    ).collect::<String>();

    let mut unpack_path = PathBuf::from(&archive_path);
    unpack_path.pop();
    unpack_path.push(folder_name);

    let file = File::open(&archive_path)?;
    let mut archive = zip::ZipArchive::new(file)?;
    archive.extract(&unpack_path)?;

    remove_file(archive_path).await?;

    Ok(unpack_path)
}

#[cfg(target_family = "unix")]
pub async fn unpack(archive_path: PathBuf) -> Result<PathBuf, UnpackError> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    use tokio::fs::{File, remove_file};

    const EXTENSION: &str = ".tar.gz";

    let archive_name = archive_path
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .ok_or(UnpackError::Parse("Path archive_name".into()))?;

    if !archive_name.ends_with(EXTENSION) {
        Err(UnpackError::Extension(archive_name.into(), EXTENSION.into()))?
    }

    let folder_name = archive_name.chars().take(
        archive_name.len() - EXTENSION.len()
    ).collect::<String>();

    let mut unpack_path = PathBuf::from(&archive_path);
    unpack_path.pop();
    unpack_path.push(folder_name);

    let file = File::open(&archive_path).await?.into_std().await;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);
    archive.unpack(&unpack_path)?;

    remove_file(archive_path).await?;

    Ok(unpack_path)
}

#[derive(thiserror::Error, Debug)]
pub enum UnpackError {
    #[error("Error while parsing: {0}")]
    Parse(String),
    #[error("Unsupported archive extension for file: {0}. Only {1} is supported on the current platform")]
    Extension(String, String),
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(target_family = "windows")]
    #[error("Zip Error: {0}")]
    Zip(#[from] zip::result::ZipError)
}
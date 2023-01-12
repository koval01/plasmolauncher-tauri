use std::{borrow::Borrow, path::{PathBuf}};

use tauri::AppHandle;
use anyhow::{Result, anyhow};

pub fn get_launcher_path(handle: impl Borrow<AppHandle>) -> Result<PathBuf> {
    let handle: &AppHandle = handle.borrow();
    let path = handle.path_resolver()
        .resolve_resource("resources/NewLaunch.jar")
        .ok_or(anyhow!("Failde to resolve NewLaunch.jar"))?;


    let path = dunce::canonicalize(path)?;     
    
    // println!("{}", path.as_os_str().to_str().unwrap());
    // .strip_prefix("\\\\?\\")?
        // .to_path_buf();
    Ok(path)
}
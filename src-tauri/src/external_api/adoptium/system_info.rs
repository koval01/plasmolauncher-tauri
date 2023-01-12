use super::error::AdoptiumError;

pub fn get_adoptium_arch() -> Result<String, AdoptiumError> {
    match std::env::consts::ARCH {
        "x86_64" => Ok("x64".into()),
        arch @ ("x86" | "arm" | "aarch64") => Ok(arch.into()),
        _ => Err(AdoptiumError::InvalidSystemInfo("arch".into()))
    }
}

pub fn get_adoptium_os() -> Result<String, AdoptiumError> {
    match std::env::consts::OS {
        "macos" => Ok("mac".into()),
        os @ ("linux" | "windows") => Ok(os.into()),
        _ => Err(AdoptiumError::InvalidSystemInfo("os".into()))
    }
}
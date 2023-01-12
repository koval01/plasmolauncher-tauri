use std::path::PathBuf;

pub trait ToOptionString {
    fn to_string(self) -> Option<String>;
}

impl ToOptionString for PathBuf {
    fn to_string(self) -> Option<String> {
        self.as_os_str().to_str().and_then(|str| Some(str.to_string()))
    }
}
use std::collections::HashMap;

use reqwest::{Response};

pub trait FileName {
    fn file_name(&self) -> String;
    fn file_name_from_content_disposition(&self) -> Option<String>;
    fn file_name_from_url(&self) -> Option<String>;
}

impl FileName for Response {

    fn file_name(&self) -> String {
        self
            .file_name_from_content_disposition()
            .or(
                self.file_name_from_url()
            )
            .unwrap_or("tmp.bin".into())
    }

    fn file_name_from_content_disposition(&self) -> Option<String> {
        let header_value = self.headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|value| Some(value.to_str().ok()?))?;

        let mut args = header_value.split("; ");

        let context = args.next()?;
        if context != "attachment" { return None };

        let args: HashMap<String, String> = args.filter_map(|arg| {
            let mut split = arg.split("=");
            let name = split.next()?;
            let value = split.next()?;
            Some((name.into(), value.into()))
        })
            .collect();

        let file_name = args.get("filename")?;

        Some(file_name.into())
    }

    fn file_name_from_url(&self) -> Option<String> {
        self.url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .and_then(|name| Some(name.to_string()))
    }
}
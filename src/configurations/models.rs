use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub not_path: PathBuf,
    pub language: String,
    pub work_day_last_hour: Option<u32>,
}

impl Config {
    pub fn keys() -> &'static [&'static str] {
        &["not_path", "language", "work_day_last_hour"]
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "not_path" => Some(self.not_path.to_string_lossy().into_owned()),
            "language" => Some(self.language.clone()),
            "work_day_last_hour" => self.work_day_last_hour.map(|h| h.to_string()),
            _ => None,
        }
    }
}

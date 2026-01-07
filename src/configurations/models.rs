use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub not_path: PathBuf,
    pub language: String,
    pub log_level: String,
}

impl Config {
    pub fn keys() -> &'static [&'static str] {
        &["not_path", "language", "log_level"]
    }

    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "not_path" => Some(self.not_path.to_string_lossy().into_owned()),
            "language" => Some(self.language.clone()),
            "log_level" => Some(self.log_level.clone()),
            _ => None,
        }
    }
}

use std::env;
use std::path::Path;
use std::path::PathBuf;

use crate::files::find::get_project_root;

pub fn find_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_project_root()?.join("config.toml"))
}

pub fn is_nost_config_file_exists() -> bool {
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });

    Path::new(&not_path).is_dir()
}

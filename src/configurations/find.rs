use std::path::PathBuf;

use crate::files::find::get_project_root;

pub fn find_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_project_root()?.join("config.toml"))
}

use std::env;
use std::path::Path;
use std::path::PathBuf;

use crate::files::find::get_project_root;

pub fn find_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(get_project_root()?.join("config.toml"))
}

/**
 * Checks if the configuration file `nost_config.json` exists
 * in the configuration directory in root/.nost
 */
pub fn is_config_exists() -> bool {
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });

    let config_path = format!("{}/.nost/nost_config.json", &not_path,);
    log::debug!("Checking if configuration exists at path: {}", &config_path);
    Path::new(&config_path).is_file()
}

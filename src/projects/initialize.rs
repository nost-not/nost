use serde_json::{json, Value};
use std::{env, fs::create_dir_all, path::Path};

use crate::dates::get::get_now_as_string;

pub fn get_project_config_path() -> String {
    // compose configuration path and create configuration folder
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });
    format!("{}/{}/", not_path, ".nost")
}

/**
 * Checks if the configuration file `project.json` exists
 * in the configuration directory in root/.nost
 */
pub fn is_project_initialized() -> bool {
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });

    let project_config_path = format!("{}/.nost/project.json", not_path);
    log::debug!(
        "Checking if configuration exists at path: {}",
        project_config_path
    );
    Path::new(&project_config_path).is_file()
}

/**
 * Create a project.json file in the not path/.nost folder
 * if it does not exist, or update the last_updated timestamp if it does.
 */
pub fn initialize_project() -> Result<String, Box<dyn std::error::Error>> {
    // check if there is an existing configuration folder with a file
    if is_project_initialized() {
        // if this file exists update the file
        let project_config_path: String = get_project_config_path();
        let config_content = std::fs::read_to_string(&project_config_path)?;
        let mut config: Value = serde_json::from_str(&config_content)?;

        config["last_updated"] = json!(get_now_as_string());
        let config_file = std::fs::File::create(&project_config_path)?;
        serde_json::to_writer_pretty(config_file, &config)?;

        return Ok(String::from("Configuration already exists."));
    }

    log::debug!("No configuration found. Initializing configuration...");
    let configuration_path = get_project_config_path();
    create_dir_all(&configuration_path)?;

    // create en empty configuration file
    let default_config_path = format!("{}{}", configuration_path, "project.json");
    let config_file = std::fs::File::create(&default_config_path)?;
    log::debug!("Configuration initialized at path: {}", configuration_path);

    // append inital content to the configuration file
    const NOST_VERSION: &str = env!("CARGO_PKG_VERSION");

    let initial_content = json!({
        "name": "My Project",
        "description": "This is my project description.",
        "created_at": get_now_as_string(),
        "last_updated": get_now_as_string(),
        "version": NOST_VERSION,
    });

    serde_json::to_writer_pretty(config_file, &initial_content)?;

    Ok(String::from(
        "Configuration has been initialized successfully!",
    ))
}

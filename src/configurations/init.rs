use serde_json::json;
use std::{env, fs::create_dir_all};

use crate::{
    configurations::{find::is_config_exists, init},
    dates::get::get_now_as_string,
};

pub fn init_configuration() -> Result<String, Box<dyn std::error::Error>> {
    // check if there is an existing configuration folder
    if is_config_exists() {
        return Ok(String::from("Configuration already exists."));
    }

    log::debug!("No configuration found. Initializing configuration...");

    // compose configuration path and create configuration folder
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });
    let configuration_path = format!("{}/{}/", &not_path, ".nost");
    create_dir_all(&configuration_path)?;

    // create en empty configuration file
    let default_config_path = format!("{}{}", &configuration_path, "nost_config.json");
    let config_file = std::fs::File::create(&default_config_path)?;
    log::debug!("Configuration initialized at path: {}", &configuration_path);

    // append inital content to the configuration file
    const NOST_VERSION: &str = env!("CARGO_PKG_VERSION");

    let initial_content = json!({
        "created_at": get_now_as_string(),
        "last_updated": get_now_as_string(),
        "version": NOST_VERSION
    });

    serde_json::to_writer_pretty(config_file, &initial_content)?;

    Ok(String::from(
        "Configuration has been initialized successfully!",
    ))
}

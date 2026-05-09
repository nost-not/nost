use serde_json::{json, Value};
use std::{env, fs::create_dir_all};

use crate::{
    configurations::{find::is_config_exists, get::get_config_path},
    dates::get::get_now_as_string,
};

pub fn upsert_configuration() -> Result<String, Box<dyn std::error::Error>> {
    // check if there is an existing configuration folder
    if is_config_exists() {
        let default_config_path: String = get_config_path();
        let config_content = std::fs::read_to_string(&default_config_path)?;
        let mut config: Value = serde_json::from_str(&config_content)?;

        config["last_updated"] = json!(get_now_as_string());
        let config_file = std::fs::File::create(&default_config_path)?;
        serde_json::to_writer_pretty(config_file, &config)?;

        return Ok(String::from("Configuration already exists."));
    }

    log::debug!("No configuration found. Initializing configuration...");
    let configuration_path = get_config_path();
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

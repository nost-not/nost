use std::{env, fs};

use crate::configurations::find::find_config_path;
use crate::configurations::models::Config;

// Load and parse the config file into a Config struct
pub fn get_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = find_config_path()?;
    let content = fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&content)?;

    Ok(config)
}

/**
 * Get a specific value from the configuration file based on the provided key.
 * i.e. get_value_from_config("not_path") will return the value of the "not_path" key in the configuration file.
 */
pub fn get_value_from_config(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let configuration = get_config().unwrap();

    if key.is_empty() {
        return Err("Config key cannot be empty".into());
    }

    let configurations_keys = Config::keys();
    if !configurations_keys.contains(&key) {
        return Err(format!("Key '{}' not found in configuration", key).into());
    }

    match configuration.get_value(key) {
        Some(value) => Ok(value),
        None => Err(format!("Key '{}' not found in configuration", key).into()),
    }
}

pub fn get_config_path() -> String {
    // compose configuration path and create configuration folder
    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        std::process::exit(1);
    });
    let project_configuration_path = format!("{}/{}/", &not_path, ".nost");
    format!("{}{}", &project_configuration_path, "project.json")
}

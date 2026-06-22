use std::{
    fs::{create_dir_all, read_to_string, write, File},
    io::Error,
    path::Path,
};

use log::debug;

use crate::{events::models::Event, projects::initialize::get_project_config_path};

pub fn record_event(event: Event) -> std::io::Result<String> {
    let config_path = get_project_config_path();
    debug!("Project config path: {:?}", config_path);

    // create journal folder if not exists
    if let Err(e) = create_dir_all(&config_path) {
        return Err(Error::other(format!(
            "🛑 Failed to create directory: {}",
            e
        )));
    }

    // create journal file if not exists
    let journal_file_path = format!("{}/journal.json", config_path);
    if !Path::new(&journal_file_path).exists() {
        if let Err(e) = File::create(&journal_file_path) {
            return Err(Error::other(format!(
                "🛑 Failed to create journal file: {}",
                e
            )));
        }

        // initialize the journal file with an empty array
        if let Err(e) = write(&journal_file_path, "[]") {
            return Err(Error::other(format!(
                "🛑 Failed to initialize journal file: {}",
                e
            )));
        }
    }

    // create the record
    let record = serde_json::to_value(&event)
        .map_err(|e| Error::other(format!("🛑 Failed to serialize event record: {}", e)))?;

    // append the record in the array (at the end)
    let journal_content = read_to_string(&journal_file_path).map_err(|e| {
        Error::other(format!(
            "🛑 Failed to read journal file '{}': {}",
            journal_file_path, e
        ))
    })?;

    let mut journal_json: serde_json::Value =
        serde_json::from_str(&journal_content).map_err(|e| {
            Error::other(format!(
                "🛑 Invalid JSON in journal file '{}': {}",
                journal_file_path, e
            ))
        })?;

    let journal_array = journal_json.as_array_mut().ok_or_else(|| {
        Error::other(format!(
            "🛑 Journal file '{}' must contain a JSON array",
            journal_file_path
        ))
    })?;

    journal_array.push(record);

    let updated_content = serde_json::to_string_pretty(&journal_json).map_err(|e| {
        Error::other(format!(
            "🛑 Failed to serialize updated journal '{}': {}",
            journal_file_path, e
        ))
    })?;

    write(&journal_file_path, format!("{}\n", updated_content)).map_err(|e| {
        Error::other(format!(
            "🛑 Failed to write updated journal '{}': {}",
            journal_file_path, e
        ))
    })?;

    Ok("Record has been added.".to_string())
}

use std::{
    fs::{create_dir_all, read_to_string, write, OpenOptions},
    io::{Error, Write},
};

use chrono::Local;

use crate::{
    dates::parse::parse_iso_date, events::models::Event,
    projects::initialize::get_project_config_path,
};

pub fn record(event: Event, date_in_string: Option<String>) -> Result<(), Error> {
    // add the event to the events file for the current month
    let month = match date_in_string {
        Some(date_in_string) => {
            let date = parse_iso_date(&date_in_string).unwrap_or_else(|_| {
                eprintln!(
                    "🛑 Invalid date format: {}. Expected YYYY-MM-DD.",
                    date_in_string
                );
                std::process::exit(1);
            });
            date.format("%Y-%m").to_string()
        }
        None => Local::now().format("%Y-%m").to_string(),
    };

    let config_path = get_project_config_path();
    let events_dir = format!("{}/events", config_path);
    let events_file_path = format!("{}/{}.ndjson", events_dir, month);

    // create the events directory if it doesn't exist
    if let Err(e) = create_dir_all(&events_dir) {
        return Err(Error::other(format!(
            "🛑 Failed to create events directory: {}",
            e
        )));
    }

    if event.stop.is_none() {
        // Start of session: always append a new NDJSON line.
        log::debug!("Recording new event: {:?}", event);
        let event_json = serde_json::to_string(&event)
            .map_err(|e| Error::other(format!("🛑 Failed to serialize event to JSON: {}", e)))?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&events_file_path)
            .map_err(|e| {
                Error::other(format!(
                    "🛑 Failed to open events file '{}': {}",
                    events_file_path, e
                ))
            })?;

        writeln!(file, "{}", event_json).map_err(|e| {
            Error::other(format!(
                "🛑 Failed to append event to '{}': {}",
                events_file_path, e
            ))
        })?;
    } else {
        // End of session: update the last open session (stop == null).
        log::debug!("Updating last event with stop time: {:?}", event);

        let content = read_to_string(&events_file_path).unwrap_or_default();
        let mut lines: Vec<String> = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| line.to_string())
            .collect();

        let last_open_index = lines.iter().rposition(|line| {
            serde_json::from_str::<Event>(line)
                .map(|e| e.stop.is_none())
                .unwrap_or(false)
        });

        let index = match last_open_index {
            Some(index) => index,
            None => {
                return Err(Error::other(format!(
                    "🛑 No open event found to close in '{}'",
                    events_file_path
                )));
            }
        };

        lines[index] = serde_json::to_string(&event)
            .map_err(|e| Error::other(format!("🛑 Failed to serialize event to JSON: {}", e)))?;

        let new_content = format!("{}\n", lines.join("\n"));

        write(&events_file_path, new_content).map_err(|e| {
            Error::other(format!(
                "🛑 Failed to update events file '{}': {}",
                events_file_path, e
            ))
        })?;
    }

    Ok(())
}

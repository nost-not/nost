use std::{fs::read_to_string, path::Path};

use crate::{
    events::models::{Event, EventName},
    projects::initialize::get_project_config_path,
};

/// Returns the most recent START_WORK or STOP_WORK event from journal.json,
/// or None if the journal does not exist or contains no work events.
pub fn find_last_work_event() -> Option<Event> {
    let config_path = get_project_config_path();
    let journal_file_path = format!("{}/journal.json", config_path);

    if !Path::new(&journal_file_path).exists() {
        return None;
    }

    let content = read_to_string(&journal_file_path).ok()?;
    let events: Vec<Event> = serde_json::from_str(&content).ok()?;

    events.into_iter().rev().find(|e| {
        e.event == EventName::StartWork.to_string()
            || e.event == EventName::StopWork.to_string()
    })
}

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
        e.event == EventName::StartWork.to_string() || e.event == EventName::StopWork.to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::models::{Event, EventName};
    use std::{env, fs};
    use tempfile::tempdir;

    /// Write a journal.json into `<dir>/.nost/journal.json`.
    fn write_journal(base: &str, events: &[Event]) {
        let journal_dir = format!("{}/.nost", base);
        fs::create_dir_all(&journal_dir).unwrap();
        let content = serde_json::to_string_pretty(events).unwrap();
        fs::write(format!("{}/journal.json", journal_dir), content).unwrap();
    }

    // All tests manipulate NOT_PATH; run serially to avoid interference.

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_no_journal_file() {
        let dir = tempdir().unwrap();
        // Point NOT_PATH at an empty temp dir — no journal.json exists yet.
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        let result = find_last_work_event();
        assert!(result.is_none(), "Expected None when no journal exists");
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_empty_journal() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        write_journal(dir.path().to_str().unwrap(), &[]);

        let result = find_last_work_event();
        assert!(result.is_none(), "Expected None for an empty journal");
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_returns_start_work() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        let events = vec![Event::now(EventName::StartWork, "work".to_string())];
        write_journal(dir.path().to_str().unwrap(), &events);

        let result = find_last_work_event();
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().event,
            EventName::StartWork.to_string(),
            "Last event should be START_WORK"
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_returns_stop_work_after_start() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        let events = vec![
            Event::now(EventName::StartWork, "work".to_string()),
            Event::now(EventName::StopWork, "work".to_string()),
        ];
        write_journal(dir.path().to_str().unwrap(), &events);

        let result = find_last_work_event();
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().event,
            EventName::StopWork.to_string(),
            "Last event should be STOP_WORK"
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_ignores_create_not() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        // CreateNot comes after StartWork — it must be ignored; StartWork wins.
        let events = vec![
            Event::now(EventName::StartWork, "work".to_string()),
            Event::now(EventName::CreateNot, "note".to_string()),
        ];
        write_journal(dir.path().to_str().unwrap(), &events);

        let result = find_last_work_event();
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().event,
            EventName::StartWork.to_string(),
            "CreateNot events must not shadow the last work event"
        );
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_work_event_multiple_sessions() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        // Two complete sessions followed by a fresh start.
        let events = vec![
            Event::now(EventName::StartWork, "work".to_string()),
            Event::now(EventName::StopWork, "work".to_string()),
            Event::now(EventName::StartWork, "work".to_string()),
            Event::now(EventName::StopWork, "work".to_string()),
            Event::now(EventName::StartWork, "work".to_string()),
        ];
        write_journal(dir.path().to_str().unwrap(), &events);

        let result = find_last_work_event();
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().event,
            EventName::StartWork.to_string(),
            "Should return the last (most recent) work event"
        );
    }
}

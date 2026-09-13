use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use crate::{events::models::Event, projects::initialize::get_project_config_path};

pub fn find_all_events_file_paths(date_in_string: Option<&String>) -> Option<Vec<PathBuf>> {
    let config_path = get_project_config_path();
    let events_dir = format!("{}/events", config_path);

    if !Path::new(&events_dir).exists() {
        log::debug!(
            "No events have been recorded. Events directory does not exist: {}",
            events_dir
        );
        return None;
    }

    let mut events_paths: Vec<_> = std::fs::read_dir(&events_dir)
        .ok()?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                let path = e.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "ndjson") {
                    Some(path)
                } else {
                    None
                }
            })
        })
        .collect();

    // if a specific month is provided, filter the files to only include those that match the month prefix (YYYY-MM)
    if let Some(date) = date_in_string {
        let month_prefix = &date[..7]; // YYYY-MM
        events_paths.retain(|path| {
            path.file_stem().map_or(false, |stem| {
                stem.to_string_lossy().starts_with(month_prefix)
            })
        });
    }

    // Sort descending
    events_paths.sort_by(|a, b| b.file_name().cmp(&a.file_name()));
    log::debug!("Found events files: {:?}", events_paths);

    Some(events_paths)
}

pub fn find_last_event(date_in_string: Option<String>) -> Option<Event> {
    let events_paths = find_all_events_file_paths(date_in_string.as_ref());

    // If no events file paths were found, return None
    if events_paths.is_none() {
        return None;
    }

    let last_month_events_path = events_paths.unwrap().first().cloned()?;
    let content = read_to_string(&last_month_events_path).ok()?;

    // Parse all lines as Event records (NDJSON format)
    let events: Vec<Event> = content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                None
            } else {
                serde_json::from_str::<Event>(trimmed).ok()
            }
        })
        .collect();

    if events.is_empty() {
        return None;
    }

    // Order the events by day and start time to ensure we get the last event correctly
    let mut ordered_events = events;
    ordered_events.sort_by(|a, b| {
        let day_cmp = a.day.cmp(&b.day);
        if day_cmp == std::cmp::Ordering::Equal {
            a.start.cmp(&b.start)
        } else {
            day_cmp
        }
    });

    // if a date is provided, filter the events to only include those that match the provided date
    if let Some(date_in_string) = date_in_string {
        ordered_events.retain(|event| event.day == date_in_string);
    }

    ordered_events.last().cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fs};
    use tempfile::tempdir;

    /// Write Event records as NDJSON into `<dir>/.nost/events/<month>.ndjson`.
    fn write_ndjson_sessions(base: &str, month: &str, sessions: &[Event]) {
        let events_dir = format!("{}/.nost/events", base);
        fs::create_dir_all(&events_dir).unwrap();
        let lines: Vec<String> = sessions
            .iter()
            .map(|s| serde_json::to_string(s).unwrap())
            .collect();
        fs::write(format!("{}/{}.ndjson", events_dir, month), lines.join("\n")).unwrap();
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_event_no_events_file() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        let result = find_last_event(None);
        assert!(result.is_none(), "Expected None when no events file exists");
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_event_single_session() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        let sessions = vec![Event {
            day: "2026-08-13".to_string(),
            start: "2026-08-13T09:00:00+09:00".to_string(),
            stop: None,
        }];

        write_ndjson_sessions(dir.path().to_str().unwrap(), "2026-08", &sessions);
        let result = find_last_event(None);

        assert!(result.is_some());
        let session = result.unwrap();
        assert_eq!(session.day, "2026-08-13");
        assert_eq!(session.start, "2026-08-13T09:00:00+09:00");
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_event_filters_last_day() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        let sessions = vec![
            Event {
                day: "2026-08-12".to_string(),
                start: "2026-08-12T09:00:00+09:00".to_string(),
                stop: Some("2026-08-12T17:00:00+09:00".to_string()),
            },
            Event {
                day: "2026-08-13".to_string(),
                start: "2026-08-13T09:00:00+09:00".to_string(),
                stop: None,
            },
        ];

        write_ndjson_sessions(dir.path().to_str().unwrap(), "2026-08", &sessions);
        let result = find_last_event(None);

        assert!(result.is_some());
        let session = result.unwrap();
        assert_eq!(session.day, "2026-08-13", "Should return last day");
        assert_eq!(session.start, "2026-08-13T09:00:00+09:00");
        assert!(session.stop.is_none());
    }

    #[test]
    #[serial_test::serial]
    fn test_find_last_event_returns_last_session_of_day() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        let sessions = vec![
            Event {
                day: "2026-08-13".to_string(),
                start: "2026-08-13T09:00:00+09:00".to_string(),
                stop: Some("2026-08-13T12:00:00+09:00".to_string()),
            },
            Event {
                day: "2026-08-13".to_string(),
                start: "2026-08-13T13:00:00+09:00".to_string(),
                stop: Some("2026-08-13T17:00:00+09:00".to_string()),
            },
            Event {
                day: "2026-08-13".to_string(),
                start: "2026-08-13T18:00:00+09:00".to_string(),
                stop: None,
            },
        ];

        write_ndjson_sessions(dir.path().to_str().unwrap(), "2026-08", &sessions);
        let result = find_last_event(None);

        assert!(result.is_some());
        let session = result.unwrap();
        assert_eq!(
            session.start, "2026-08-13T18:00:00+09:00",
            "Should return last session"
        );
        assert!(session.stop.is_none());
    }
}

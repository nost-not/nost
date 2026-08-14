use chrono::Local;

use crate::{
    dates::validate::is_valid_string_date,
    events::{
        journal::{append_session, load_month_sessions, update_last_stop},
        models::{Event, EventName, WorkSession},
    },
    files::create::create_note_file_with_folders,
    projects::initialize::initialize_project,
};

/// Pure function: decides which work event to record next based on the most
/// recent work event. No I/O; easy to unit-test.
///
/// Returns the `EventName` to record: `StartWork` to open a session,
/// `StopWork` to close the current one.
#[allow(dead_code)]
pub fn determine_next_work_event(last_event: Option<&Event>) -> EventName {
    match last_event {
        // Last event was a start → close the current session
        Some(e) if e.event == EventName::StartWork.to_string() => EventName::StopWork,
        // No previous event, a previous stop, or any non-work event → start a
        // fresh session
        _ => EventName::StartWork,
    }
}

pub fn work(date_in_string: Option<String>) {
    let _ = initialize_project();

    // Validate the provided date if any, then resolve to a concrete YYYY-MM-DD string.
    let resolved_date = match date_in_string {
        Some(ref date) if !is_valid_string_date(date) => {
            eprintln!("🛑 Invalid date format: {}. Expected YYYY-MM-DD.", date);
            std::process::exit(1);
        }
        Some(ref date) => date.clone(),
        None => Local::now().format("%Y-%m-%d").to_string(),
    };

    // Create (or reuse) the work file.
    let _not_path =
        create_note_file_with_folders("work".to_string(), Some(resolved_date.clone())).unwrap();

    // Determine the current month for the NDJSON journal.
    let month = &resolved_date[..7]; // YYYY-MM

    // Check if there is an open session in the monthly NDJSON file.
    let sessions = load_month_sessions(month).unwrap_or_default();
    let has_open_session = sessions.last().map(|s| s.stop.is_none()).unwrap_or(false);

    if has_open_session {
        // Close the open session.
        let stop_datetime = Local::now().to_rfc3339();
        update_last_stop(month, &stop_datetime)
            .expect("🛑 Failed to close work session.");
        println!("✅ Work session closed.");
    } else {
        // Open a new session.
        let start_datetime = Local::now().to_rfc3339();
        let session = WorkSession {
            workday: resolved_date.clone(),
            start: start_datetime,
            stop: None,
        };
        append_session(&session).expect("🛑 Failed to start work session.");
        println!("✅ Work session started.");
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::models::{Event, EventName};

    /// Helper: build a minimal Event with a given name.
    fn make_event(event_name: EventName) -> Event {
        Event::new(event_name, "work".to_string(), "2026-08-13".to_string())
    }

    #[test]
    fn test_determine_next_work_event_no_previous_event() {
        // No journal entry at all → should start
        assert_eq!(determine_next_work_event(None), EventName::StartWork);
    }

    #[test]
    fn test_determine_next_work_event_after_stop_work() {
        // Last event is STOP_WORK → should start a new session
        let event = make_event(EventName::StopWork);
        assert_eq!(
            determine_next_work_event(Some(&event)),
            EventName::StartWork
        );
    }

    #[test]
    fn test_determine_next_work_event_after_start_work() {
        // Last event is START_WORK → should close the current session
        let event = make_event(EventName::StartWork);
        assert_eq!(determine_next_work_event(Some(&event)), EventName::StopWork);
    }

    #[test]
    fn test_determine_next_work_event_after_create_not() {
        // A non-work event (e.g. CreateNot) → start a fresh session
        let event = make_event(EventName::CreateNot);
        assert_eq!(
            determine_next_work_event(Some(&event)),
            EventName::StartWork
        );
    }
}

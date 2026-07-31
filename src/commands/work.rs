use crate::{
    events::{
        find::find_last_work_event,
        models::{Event, EventName},
        record::record_event,
    },
    files::create::create_note_file_with_folders,
    projects::initialize::initialize_project,
};

/// Pure function: decides which work event to record next based on the most
/// recent work event. No I/O; easy to unit-test.
///
/// Returns the `EventName` to record: `StartWork` to open a session,
/// `StopWork` to close the current one.
pub fn determine_next_work_event(last_event: Option<&Event>) -> EventName {
    match last_event {
        // Last event was a start → close the current session
        Some(e) if e.event == EventName::StartWork.to_string() => EventName::StopWork,
        // No previous event, a previous stop, or any non-work event → start a
        // fresh session
        _ => EventName::StartWork,
    }
}

pub fn work() {
    let _ = initialize_project();

    // Create (or reuse) today's work file using the new folder structure:
    // <not_path>/<year>/<month>/<week>/<day>/<YYYY-MM-DD>.work.md
    let _not_path = create_note_file_with_folders("work".to_string()).unwrap();

    // Read journal.json to determine the current session state.
    let last_event = find_last_work_event();

    match determine_next_work_event(last_event.as_ref()) {
        EventName::StartWork => {
            record_event(Event::now(EventName::StartWork, "work".to_string()))
                .expect("🛑 Failed to record START_WORK event.");
            println!("✅ Work session started.");
        }
        EventName::StopWork => {
            record_event(Event::now(EventName::StopWork, "work".to_string()))
                .expect("🛑 Failed to record STOP_WORK event.");
            println!("✅ Work session closed.");
        }
        _ => unreachable!("determine_next_work_event only returns StartWork or StopWork"),
    }

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::models::{Event, EventName};

    /// Helper: build a minimal Event with a given name.
    fn make_event(event_name: EventName) -> Event {
        Event::now(event_name, "work".to_string())
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

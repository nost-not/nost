use crate::{
    events::{
        find::find_last_work_event,
        models::{Event, EventName},
        record::record_event,
    },
    files::create::create_note_file_with_folders,
    projects::initialize::initialize_project,
};

/// Outcome of the work-toggle decision.
#[derive(Debug, PartialEq)]
pub enum WorkAction {
    Start,
    Stop,
}

/// Pure function: decides whether to start or stop a work session based on
/// the most recent work event.  No I/O; easy to unit-test.
pub fn determine_work_action(last_event: Option<&Event>) -> WorkAction {
    match last_event {
        // No previous event → start a fresh session
        None => WorkAction::Start,
        // Last event was a stop → open a new session
        Some(e) if e.event == EventName::StopWork.to_string() => WorkAction::Start,
        // Last event was a start → close the current session
        _ => WorkAction::Stop,
    }
}

pub fn work() {
    let _ = initialize_project();

    // Create (or reuse) today's work file using the new folder structure:
    // <not_path>/<year>/<month>/<week>/<day>/<YYYY-MM-DD>.work.md
    let _not_path = create_note_file_with_folders("work".to_string()).unwrap();

    // Read journal.json to determine the current session state.
    let last_event = find_last_work_event();

    match determine_work_action(last_event.as_ref()) {
        WorkAction::Start => {
            record_event(Event::now(EventName::StartWork, "work".to_string()))
                .expect("🛑 Failed to record START_WORK event.");
            println!("✅ Work session started.");
        }
        WorkAction::Stop => {
            record_event(Event::now(EventName::StopWork, "work".to_string()))
                .expect("🛑 Failed to record STOP_WORK event.");
            println!("✅ Work session closed.");
        }
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
    fn test_determine_work_action_no_previous_event() {
        // No journal entry at all → should start
        assert_eq!(determine_work_action(None), WorkAction::Start);
    }

    #[test]
    fn test_determine_work_action_after_stop_work() {
        // Last event is STOP_WORK → should start a new session
        let event = make_event(EventName::StopWork);
        assert_eq!(determine_work_action(Some(&event)), WorkAction::Start);
    }

    #[test]
    fn test_determine_work_action_after_start_work() {
        // Last event is START_WORK → should close the current session
        let event = make_event(EventName::StartWork);
        assert_eq!(determine_work_action(Some(&event)), WorkAction::Stop);
    }

    #[test]
    fn test_determine_work_action_after_create_not() {
        // A non-work event (e.g. CreateNot) is treated as "still open" → stop
        // (shouldn't normally appear as last work event, but defensive check)
        let event = make_event(EventName::CreateNot);
        assert_eq!(determine_work_action(Some(&event)), WorkAction::Stop);
    }
}

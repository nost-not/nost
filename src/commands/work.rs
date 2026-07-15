use crate::{
    events::{
        find::find_last_work_event,
        models::{Event, EventName},
        record::record_event,
    },
    files::create::create_note_file_with_folders,
    projects::initialize::initialize_project,
};

pub fn work() {
    let _ = initialize_project();

    // Create (or reuse) today's work file using the new folder structure:
    // <not_path>/<year>/<month>/<week>/<day>/<YYYY-MM-DD>.work.md
    let _not_path = create_note_file_with_folders("work".to_string()).unwrap();

    // Read journal.json to determine the current session state.
    // No work event found, or last event is STOP_WORK → open a new session.
    // Last event is START_WORK → close the current session.
    let last_event = find_last_work_event();

    let should_start = match &last_event {
        None => true,
        Some(e) => e.event == EventName::StopWork.to_string(),
    };

    if should_start {
        record_event(Event::now(EventName::StartWork, "work".to_string()))
            .expect("🛑 Failed to record START_WORK event.");
        println!("✅ Work session started.");
    } else {
        record_event(Event::now(EventName::StopWork, "work".to_string()))
            .expect("🛑 Failed to record STOP_WORK event.");
        println!("✅ Work session closed.");
    }

    std::process::exit(0);
}

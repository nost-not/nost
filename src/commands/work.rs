use chrono::Local;

use crate::{
    dates::validate::is_valid_string_date, events::record::record,
    files::create::create_note_file_with_folders, projects::initialize::initialize_project,
};

fn compute_next_event(
    last_event: Option<crate::events::models::Event>,
    resolved_date: &str,
    now_datetime: &str,
) -> crate::events::models::Event {
    match last_event {
        Some(mut event) if event.stop.is_none() => {
            event.stop = Some(now_datetime.to_string());
            event
        }
        _ => crate::events::models::Event {
            day: resolved_date.to_string(),
            start: now_datetime.to_string(),
            stop: None,
        },
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

    let last_event = crate::events::find::find_last_event(date_in_string.clone());
    let _next_event = compute_next_event(last_event, &resolved_date, &Local::now().to_rfc3339());

    record(_next_event, date_in_string.clone()).unwrap();

    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    use crate::events::models::Event;

    #[test]
    fn test_compute_next_event_starts_when_no_previous_event() {
        let result = super::compute_next_event(None, "2026-09-13", "09:00");

        assert_eq!(result.day, "2026-09-13");
        assert_eq!(result.start, "09:00");
        assert!(result.stop.is_none());
    }

    #[test]
    fn test_compute_next_event_starts_when_previous_event_closed() {
        let previous = Event {
            day: "2026-09-13".to_string(),
            start: "08:00".to_string(),
            stop: Some("08:30".to_string()),
        };

        let result = super::compute_next_event(Some(previous), "2026-09-13", "09:00");

        assert_eq!(result.day, "2026-09-13");
        assert_eq!(result.start, "09:00");
        assert!(result.stop.is_none());
    }

    #[test]
    fn test_compute_next_event_closes_when_previous_event_open() {
        let previous = Event {
            day: "2026-09-13".to_string(),
            start: "08:00".to_string(),
            stop: None,
        };

        let result = super::compute_next_event(Some(previous), "2026-09-13", "09:00");

        assert_eq!(result.day, "2026-09-13");
        assert_eq!(result.start, "08:00");
        assert_eq!(result.stop, Some("09:00".to_string()));
    }
}

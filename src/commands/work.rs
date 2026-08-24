use chrono::{DateTime, Local, Timelike};

use crate::{
    dates::validate::is_valid_string_date,
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

const DEFAULT_WORK_DAY_LAST_HOUR: u32 = 6;

pub fn define_current_work_day(datetime: DateTime<Local>, last_hour: u32) -> String {
    let today = datetime.format("%Y-%m-%d").to_string();
    // if time is before the configured last hour of the work day, consider it still the previous day
    if datetime.hour() < last_hour {
        let yesterday = datetime.date_naive() - chrono::Duration::days(1);
        return yesterday.format("%Y-%m-%d").to_string();
    }
    today
}

pub fn work(date_in_string: Option<String>) {
    let _ = initialize_project();

    // Validate the provided date if any, then resolve to a concrete YYYY-MM-DD string.
    // This single resolved date is used for both the file path and the journal event,
    // ensuring they are always consistent.
    let resolved_date = match date_in_string {
        Some(ref date) if !is_valid_string_date(date) => {
            eprintln!("🛑 Invalid date format: {}. Expected YYYY-MM-DD.", date);
            std::process::exit(1);
        }
        Some(ref date) => date.clone(),
        None => {
            let last_hour = crate::configurations::get::get_config()
                .ok()
                .and_then(|c| c.work_day_last_hour)
                .unwrap_or(DEFAULT_WORK_DAY_LAST_HOUR);
            define_current_work_day(Local::now(), last_hour)
        }
    };

    // Create (or reuse) the work file using the new folder structure:
    // <not_path>/<year>/<month>/<week>/<day>/<YYYY-MM-DD>.work.md
    let _not_path =
        create_note_file_with_folders("work".to_string(), Some(resolved_date.clone())).unwrap();

    // Read journal.json to determine the current session state.
    let last_event = find_last_work_event();

    match determine_next_work_event(last_event.as_ref()) {
        EventName::StartWork => {
            record_event(Event::new(
                EventName::StartWork,
                "work".to_string(),
                resolved_date.clone(),
            ))
            .expect("🛑 Failed to record START_WORK event.");
            println!("✅ Work session started.");
        }
        EventName::StopWork => {
            record_event(Event::new(
                EventName::StopWork,
                "work".to_string(),
                resolved_date.clone(),
            ))
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

    fn make_datetime(date: &str, hour: u32) -> DateTime<Local> {
        use chrono::{NaiveDate, TimeZone};
        Local
            .from_local_datetime(
                &NaiveDate::parse_from_str(date, "%Y-%m-%d")
                    .unwrap()
                    .and_hms_opt(hour, 0, 0)
                    .unwrap(),
            )
            .unwrap()
    }

    #[test]
    fn test_work_day_normal_hour() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 10), 6),
            "2026-08-25"
        );
    }

    #[test]
    fn test_work_day_before_6am() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 5), 6),
            "2026-08-24"
        );
    }

    #[test]
    fn test_work_day_exactly_midnight() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 0), 6),
            "2026-08-24"
        );
    }

    #[test]
    fn test_work_day_exactly_6am() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 6), 6),
            "2026-08-25"
        );
    }

    #[test]
    fn test_work_day_month_boundary() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-09-01", 3), 6),
            "2026-08-31"
        );
    }

    #[test]
    fn test_work_day_year_boundary() {
        assert_eq!(
            define_current_work_day(make_datetime("2026-01-01", 3), 6),
            "2025-12-31"
        );
    }

    #[test]
    fn test_work_day_custom_start_hour() {
        // with start_hour=4, 03:00 is still previous day but 04:00 is the new day
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 3), 4),
            "2026-08-24"
        );
        assert_eq!(
            define_current_work_day(make_datetime("2026-08-25", 4), 4),
            "2026-08-25"
        );
    }
}

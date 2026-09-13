use chrono::{DateTime, Local, Timelike};

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

use std::{collections::HashMap, collections::HashSet, fs::read_to_string, path::Path};

use chrono::{DateTime, Datelike, FixedOffset, Local};

use crate::events::models::{Event, EventName};
use crate::plugins::gdarquie_work::work::{
    compose_monthly_work_stats, MonthlyWorkStats, WeekId, WorkStats, WorkStatsByWeek,
};
use crate::projects::initialize::get_project_config_path;

/// New `work-stats` implementation that computes the exact same result as the
/// legacy `nost work-stats` command, but sourcing its data from
/// `journal.json` (recorded work events) instead of the annotations embedded
/// in the `.md` note files.
///
/// Command: `nost new-work-stats [YYYY-MM]` (alias `nws`).
pub fn new_work_stats(args: Vec<String>) {
    // Optional first arg is month in format YYYY-MM
    let month = if args.len() > 2 {
        let m = args[2].as_str();
        if !is_valid_year_month(m) {
            eprintln!("Invalid month format. Please use YYYY-MM.");
            std::process::exit(1);
        }
        Some(m.to_string())
    } else {
        None
    };

    let stats = match compute_monthly_work_stats_from_journal(month.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("💥 Cannot compute stats from journal: \"{}\".", e);
            eprintln!("Is there a journal.json with work events for this month?");
            std::process::exit(1);
        }
    };

    // Reuse the exact same rendering as the legacy command so the output is
    // identical.
    let stats_content = compose_monthly_work_stats(stats);
    println!("{}", stats_content);
}

/// Load every event from `journal.json`. Returns an empty vec if the journal
/// does not exist yet.
fn load_journal_events() -> Result<Vec<Event>, std::io::Error> {
    let config_path = get_project_config_path();
    let journal_file_path = format!("{}/journal.json", config_path);

    if !Path::new(&journal_file_path).exists() {
        return Ok(Vec::new());
    }

    let content = read_to_string(&journal_file_path)?;
    let events: Vec<Event> = serde_json::from_str(&content).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid JSON in journal file: {}", e),
        )
    })?;

    Ok(events)
}

/// Compute the total work time in minutes from a chronological slice of work
/// events (only StartWork / StopWork matter). Same pairing logic as the
/// annotation-based `compute_work_time_from_annotations`.
pub fn compute_work_time_from_events(events: &[Event]) -> i32 {
    let mut total_time_in_minutes = 0;
    let mut start_time: Option<DateTime<FixedOffset>> = None;

    for event in events {
        let datetime = match DateTime::parse_from_rfc3339(&event.datetime) {
            Ok(dt) => dt,
            Err(_) => continue,
        };

        match event.event.parse::<EventName>() {
            Ok(EventName::StartWork) => {
                start_time = Some(datetime);
            }
            Ok(EventName::StopWork) => {
                if let Some(start) = start_time {
                    total_time_in_minutes += (datetime - start).num_minutes() as i32;
                    start_time = None;
                }
            }
            _ => { /* ignore other events */ }
        }
    }

    total_time_in_minutes
}

/// Compute monthly work stats from journal events, filtered to the requested
/// month (defaults to the current month). Mirrors
/// `work::compute_monthly_work_stats` but reads events instead of annotations.
pub fn compute_monthly_work_stats_from_journal(
    month: Option<&str>,
) -> Result<MonthlyWorkStats, std::io::Error> {
    let date = match month {
        Some(m) => {
            chrono::NaiveDate::parse_from_str(&format!("{}-01", m), "%Y-%m-%d").map_err(|e| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid month format. Please use YYYY-MM. Error: {}", e),
                )
            })?
        }
        None => Local::now().date_naive(),
    };

    let month_prefix = date.format("%Y-%m").to_string();
    println!("Computing work stats for month: {}", month_prefix);

    let events = load_journal_events()?;

    // Keep only StartWork / StopWork events that belong to the requested month.
    let work_events: Vec<Event> = events
        .into_iter()
        .filter(|e| {
            matches!(
                e.event.parse::<EventName>(),
                Ok(EventName::StartWork) | Ok(EventName::StopWork)
            )
        })
        .collect();

    Ok(compute_stats_from_work_events(&work_events, &month_prefix))
}

/// Pure aggregation core: group work events by day, keep only the requested
/// month, then roll up into weekly and monthly totals. No I/O — unit-testable.
pub fn compute_stats_from_work_events(events: &[Event], month_prefix: &str) -> MonthlyWorkStats {
    // group events by workday (based on the event day, RFC3339-derived)
    let mut events_by_day: HashMap<String, Vec<Event>> = HashMap::new();
    for event in events {
        let day = day_of_event(event);
        events_by_day.entry(day).or_default().push(event.clone());
    }

    // Discard days that do not belong to the requested month.
    events_by_day.retain(|day, _| day.starts_with(month_prefix));

    let mut work_stats_by_week: HashMap<WeekId, WorkStatsByWeek> = HashMap::new();
    let mut total_duration = 0;
    let mut worked_days_set = HashSet::new();

    for (day, day_events) in events_by_day.iter() {
        let length_in_minutes = compute_work_time_from_events(day_events);

        let parsed = match chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };
        let week_id = WeekId {
            year: parsed.iso_week().year(),
            week: parsed.iso_week().week(),
        };

        work_stats_by_week
            .entry(week_id)
            .and_modify(|week_stats| {
                week_stats.total_duration_in_minutes += length_in_minutes;
                week_stats.work_stats.push(WorkStats {
                    day: day.clone(),
                    length_in_minutes,
                });
            })
            .or_insert_with(|| WorkStatsByWeek {
                total_duration_in_minutes: length_in_minutes,
                work_stats: vec![WorkStats {
                    day: day.clone(),
                    length_in_minutes,
                }],
            });

        total_duration += length_in_minutes;
        worked_days_set.insert(day.clone());
    }

    MonthlyWorkStats {
        total_duration_in_minutes: total_duration,
        total_work_days: worked_days_set.len() as i32,
        work_stats_by_week,
    }
}

/// Resolve the workday (YYYY-MM-DD) for an event. Prefer the RFC3339 datetime
/// (matches how durations are computed); fall back to the stored `day` field.
fn day_of_event(event: &Event) -> String {
    match DateTime::parse_from_rfc3339(&event.datetime) {
        Ok(dt) => dt.format("%Y-%m-%d").to_string(),
        Err(_) => event.day.clone(),
    }
}

/// Validate a string as year-month in format YYYY-MM (01..12).
fn is_valid_year_month(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[4] != b'-' {
        return false;
    }
    let year = &s[0..4];
    let month = &s[5..7];
    if !year.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !month.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(month.parse::<u32>(), Ok(m) if (1..=12).contains(&m))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::models::{Event, EventName};

    /// Helper: build an Event at a given RFC3339 datetime.
    fn make_event(event_name: EventName, datetime: &str) -> Event {
        let dt = DateTime::parse_from_rfc3339(datetime).unwrap();
        Event {
            datetime: datetime.to_string(),
            event: format!("{}", event_name),
            day: dt.format("%Y-%m-%d").to_string(),
            not_type: "work".to_string(),
            uid: "test-uid".to_string(),
        }
    }

    #[test]
    fn test_compute_work_time_single_session() {
        let events = vec![
            make_event(EventName::StartWork, "2025-09-01T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T10:00:00+00:00"),
        ];
        assert_eq!(compute_work_time_from_events(&events), 60);
    }

    #[test]
    fn test_compute_work_time_ignores_unpaired_start() {
        let events = vec![make_event(
            EventName::StartWork,
            "2025-09-01T09:00:00+00:00",
        )];
        assert_eq!(compute_work_time_from_events(&events), 0);
    }

    #[test]
    fn test_compute_work_time_multiple_sessions_same_day() {
        let events = vec![
            make_event(EventName::StartWork, "2025-09-01T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T10:00:00+00:00"),
            make_event(EventName::StartWork, "2025-09-01T14:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T16:30:00+00:00"),
        ];
        assert_eq!(compute_work_time_from_events(&events), 60 + 150);
    }

    #[test]
    fn test_stats_single_day() {
        let events = vec![
            make_event(EventName::StartWork, "2025-09-01T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T10:00:00+00:00"),
        ];
        let stats = compute_stats_from_work_events(&events, "2025-09");
        assert_eq!(stats.total_duration_in_minutes, 60);
        assert_eq!(stats.total_work_days, 1);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }

    #[test]
    fn test_stats_multiple_days_one_week() {
        let events = vec![
            make_event(EventName::StartWork, "2025-09-01T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T10:00:00+00:00"),
            make_event(EventName::StartWork, "2025-09-02T10:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-02T12:00:00+00:00"),
        ];
        let stats = compute_stats_from_work_events(&events, "2025-09");
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }

    #[test]
    fn test_stats_spanning_two_iso_weeks() {
        let events = vec![
            make_event(EventName::StartWork, "2025-08-31T09:00:00+00:00"), // week 35
            make_event(EventName::StopWork, "2025-08-31T10:00:00+00:00"),
            make_event(EventName::StartWork, "2025-09-01T10:00:00+00:00"), // week 36
            make_event(EventName::StopWork, "2025-09-01T12:00:00+00:00"),
        ];
        // Only September days survive the month filter.
        let stats = compute_stats_from_work_events(&events, "2025-09");
        assert_eq!(stats.total_work_days, 1);
        assert_eq!(stats.total_duration_in_minutes, 120);
    }

    #[test]
    fn test_stats_filters_out_other_months() {
        let events = vec![
            make_event(EventName::StartWork, "2025-08-15T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-08-15T11:00:00+00:00"),
            make_event(EventName::StartWork, "2025-09-01T09:00:00+00:00"),
            make_event(EventName::StopWork, "2025-09-01T10:00:00+00:00"),
        ];
        let stats = compute_stats_from_work_events(&events, "2025-09");
        assert_eq!(stats.total_work_days, 1);
        assert_eq!(stats.total_duration_in_minutes, 60);
    }
}

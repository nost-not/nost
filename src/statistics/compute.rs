use crate::{
    events::models::Event,
    projects::initialize::get_project_config_path,
    statistics::models::{MonthStats, Stats, WeekId, WeekStats, WorkEvent, WorkEventKind},
};
use chrono::{Datelike, Local, NaiveDate};
use log::debug;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::BufReader,
    path::Path,
};

fn load_events() -> Result<Vec<Event>, std::io::Error> {
    let config_path = get_project_config_path();
    let journal_file_path = format!("{}/journal.json", config_path);

    if !Path::new(&journal_file_path).exists() {
        return Ok(Vec::new());
    }

    let file = File::open(&journal_file_path)?;
    let reader = BufReader::new(file);
    let events: Vec<Event> = serde_json::from_reader(reader).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("Invalid JSON in journal file: {}", e),
        )
    })?;

    Ok(events)
}

/// Parse raw events into `WorkEvent`s, keeping only the requested month.
/// This is the single validation boundary: invalid datetimes and non-work
/// events are silently dropped here. Everything downstream is panic-free.
fn parse_month_events(month: &str, events: Vec<Event>) -> Vec<WorkEvent> {
    events
        .into_iter()
        .filter(|e| e.day.starts_with(month))
        .filter_map(|e| WorkEvent::try_from_raw(&e.datetime, &e.event, &e.day))
        .collect()
}

pub fn compute_month_stats(month: Option<&str>) -> Result<MonthStats, std::io::Error> {
    let date = match month {
        Some(m) => NaiveDate::parse_from_str(&format!("{}-01", m), "%Y-%m-%d").map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid month format. Please use YYYY-MM. Error: {}", e),
            )
        })?,
        None => Local::now().date_naive(),
    };

    let month = date.format("%Y-%m").to_string();
    debug!("Computing stats for month: {}", &month);
    println!("Computing stats for month: {}", &month);

    let raw_events = load_events()?;
    let work_events = parse_month_events(&month, raw_events);

    debug!("Loaded {} work events from journal.", work_events.len());

    Ok(compute_stats_from_events(work_events))
}

pub fn compute_stats_from_events(events: Vec<WorkEvent>) -> MonthStats {
    // Group events by workday
    let mut events_by_day: HashMap<String, Vec<WorkEvent>> = HashMap::new();
    for event in events {
        let day = event.day.clone();
        events_by_day.entry(day).or_default().push(event);
    }

    let mut work_stats_by_week: HashMap<WeekId, WeekStats> = HashMap::new();
    let mut total_duration = 0;
    let mut worked_days_set = HashSet::new();

    for (day, day_events) in events_by_day.iter() {
        let length_in_minutes = compute_workday_duration(day_events);

        let parsed_date = match NaiveDate::parse_from_str(day, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };
        let week_id = WeekId {
            year: parsed_date.iso_week().year(),
            week: parsed_date.iso_week().week(),
        };

        work_stats_by_week
            .entry(week_id)
            .and_modify(|week_stats| {
                week_stats.total_duration_in_minutes += length_in_minutes;
                week_stats.work_stats.push(Stats {
                    day: day.clone(),
                    length_in_minutes,
                });
            })
            .or_insert_with(|| WeekStats {
                total_duration_in_minutes: length_in_minutes,
                work_stats: vec![Stats {
                    day: day.clone(),
                    length_in_minutes,
                }],
            });

        total_duration += length_in_minutes;
        worked_days_set.insert(day.clone());
    }

    MonthStats {
        total_duration_in_minutes: total_duration,
        total_work_days: worked_days_set.len() as i32,
        work_stats_by_week,
    }
}

/// Compute the total work time in minutes from a slice of already-validated work events.
/// No parsing, no error handling — WorkEvent guarantees valid datetimes and kinds.
pub fn compute_workday_duration(events: &[WorkEvent]) -> i32 {
    let mut sorted: Vec<&WorkEvent> = events.iter().collect();
    sorted.sort_by_key(|e| e.datetime);

    let mut total_time_in_minutes = 0;
    let mut start_time = None;

    for event in sorted {
        match event.kind {
            WorkEventKind::Start => {
                start_time = Some(event.datetime);
            }
            WorkEventKind::Stop => {
                if let Some(start) = start_time.take() {
                    total_time_in_minutes += (event.datetime - start).num_minutes() as i32;
                }
            }
        }
    }

    total_time_in_minutes
}

#[cfg(test)]
mod tests {
    use super::{compute_stats_from_events, compute_workday_duration, parse_month_events};
    use crate::events::models::Event;
    use crate::statistics::models::{WorkEvent, WorkEventKind};
    use chrono::DateTime;

    fn make_raw_event(day: &str, uid: &str) -> Event {
        Event {
            datetime: format!("{}T09:00:00+00:00", day),
            event: "START_WORK".to_string(),
            day: day.to_string(),
            not_type: "work".to_string(),
            uid: uid.to_string(),
        }
    }

    fn make_work_event(kind: WorkEventKind, datetime: &str) -> WorkEvent {
        let dt = DateTime::parse_from_rfc3339(datetime).unwrap();
        WorkEvent {
            datetime: dt,
            kind,
            day: dt.format("%Y-%m-%d").to_string(),
        }
    }

    #[test]
    fn parse_month_events_keeps_only_requested_month() {
        let events = vec![
            make_raw_event("2026-08-01", "a"),
            make_raw_event("2026-08-15", "b"),
            make_raw_event("2026-07-31", "c"),
            make_raw_event("2026-09-01", "d"),
        ];
        let parsed = parse_month_events("2026-08", events);
        assert_eq!(parsed.len(), 2);
        assert!(parsed.iter().all(|e| e.day.starts_with("2026-08")));
    }

    #[test]
    fn parse_month_events_drops_invalid_datetime() {
        let bad = Event {
            datetime: "not-a-date".to_string(),
            event: "START_WORK".to_string(),
            day: "2026-08-01".to_string(),
            not_type: "work".to_string(),
            uid: "bad".to_string(),
        };
        let parsed = parse_month_events("2026-08", vec![bad]);
        assert!(parsed.is_empty());
    }

    #[test]
    fn parse_month_events_drops_non_work_events() {
        let mut other = make_raw_event("2026-08-01", "other");
        other.event = "CREATE_NOT".to_string();
        let parsed = parse_month_events("2026-08", vec![other]);
        assert!(parsed.is_empty());
    }

    #[test]
    fn compute_workday_duration_single_session() {
        let events = vec![
            make_work_event(WorkEventKind::Start, "2026-08-05T09:00:00+00:00"),
            make_work_event(WorkEventKind::Stop, "2026-08-05T10:30:00+00:00"),
        ];
        assert_eq!(compute_workday_duration(&events), 90);
    }

    #[test]
    fn compute_workday_duration_events_out_of_order() {
        let events = vec![
            make_work_event(WorkEventKind::Stop, "2026-08-05T18:00:00+00:00"),
            make_work_event(WorkEventKind::Start, "2026-08-05T09:00:00+00:00"),
        ];
        assert_eq!(compute_workday_duration(&events), 9 * 60);
    }

    #[test]
    fn compute_workday_duration_ignores_unpaired_start() {
        let events = vec![make_work_event(
            WorkEventKind::Start,
            "2026-08-05T09:00:00+00:00",
        )];
        assert_eq!(compute_workday_duration(&events), 0);
    }

    #[test]
    fn compute_workday_duration_multiple_sessions() {
        let events = vec![
            make_work_event(WorkEventKind::Start, "2026-08-05T09:00:00+00:00"),
            make_work_event(WorkEventKind::Stop, "2026-08-05T10:00:00+00:00"),
            make_work_event(WorkEventKind::Start, "2026-08-05T14:00:00+00:00"),
            make_work_event(WorkEventKind::Stop, "2026-08-05T16:30:00+00:00"),
        ];
        assert_eq!(compute_workday_duration(&events), 60 + 150);
    }

    #[test]
    fn compute_stats_from_events_aggregates_days_and_total() {
        let events = vec![
            make_work_event(WorkEventKind::Start, "2026-08-05T09:00:00+00:00"),
            make_work_event(WorkEventKind::Stop, "2026-08-05T10:00:00+00:00"),
            make_work_event(WorkEventKind::Start, "2026-08-06T10:00:00+00:00"),
            make_work_event(WorkEventKind::Stop, "2026-08-06T12:00:00+00:00"),
        ];
        let stats = compute_stats_from_events(events);
        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }
}

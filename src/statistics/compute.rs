use crate::{
    events::models::{Event, EventName},
    projects::initialize::get_project_config_path,
    statistics::models::{MonthStats, Stats, WeekId, WeekStats},
};
use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate};
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

    // if there is no journal file yet, return an empty vec
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

fn filter_month_events(month: &str, events: Vec<Event>) -> Vec<Event> {
    events
        .into_iter()
        .filter(|event| {
            event.day.starts_with(month)
                && matches!(event.event.as_str(), "START_WORK" | "STOP_WORK")
        })
        .collect()
}

pub fn compute_month_stats(month: Option<&str>) -> Result<MonthStats, std::io::Error> {
    // get the month to compute stats for, defaulting to the current month if not provided
    let date = match month {
        Some(m) => NaiveDate::parse_from_str(&format!("{}-01", m), "%Y-%m-%d").map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid month format. Please use YYYY-MM. Error: {}", e),
            )
        })?,
        None => Local::now().date_naive(),
    };

    debug!("Computing stats for month: {}", date.format("%Y-%m"));

    let month = date.format("%Y-%m").to_string();
    let events = load_events()?;
    let month_events = filter_month_events(&month, events);

    log::debug!("Loaded {} events from journal.", month_events.len());

    // we have the month events, now we can compute the stats
    Ok(compute_stats_from_events(month_events))
}

pub fn compute_stats_from_events(events: Vec<Event>) -> MonthStats {
    // group events by workday (based on the event day, RFC3339-derived)
    let mut events_by_day: HashMap<String, Vec<Event>> = HashMap::new();
    for event in events {
        let day = event.day.clone();
        events_by_day.entry(day).or_default().push(event);
    }

    // prepare to compute stats by week
    let mut work_stats_by_week: HashMap<WeekId, WeekStats> = HashMap::new();
    let mut total_duration = 0;
    let mut worked_days_set = HashSet::new();

    // compute stats for each day and aggregate by week
    for (day, day_events) in events_by_day.iter() {
        let length_in_minutes = compute_workday_duration(day_events);

        let parsed_date = match chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => continue,
        };
        let week_id = WeekId {
            year: parsed_date.iso_week().year(),
            week: parsed_date.iso_week().week(),
        };

        // add computed stats to week stats
        work_stats_by_week
            // if week stat already exists, update with current day and adjust week total duration
            .entry(week_id)
            .and_modify(|week_stats| {
                week_stats.total_duration_in_minutes += length_in_minutes;
                week_stats.work_stats.push(Stats {
                    day: day.clone(),
                    length_in_minutes,
                });
            })
            // otherwise create a new week stat
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

    // regroup the result in a month Stat and return it
    MonthStats {
        total_duration_in_minutes: total_duration,
        total_work_days: worked_days_set.len() as i32,
        work_stats_by_week,
    }
}

/// Compute the total work time in minutes from a slice of work events
pub fn compute_workday_duration(events: &[Event]) -> i32 {
    // Sort events by datetime
    let mut sorted_events: Vec<&Event> = events.iter().collect();
    sorted_events.sort_by_key(|event| {
        DateTime::parse_from_rfc3339(&event.datetime)
            .expect("Invalid RFC3339 datetime in work event")
    });

    // compute work length sessions by pairing START_WORK and STOP_WORK events
    let mut total_time_in_minutes = 0;
    let mut start_time: Option<DateTime<FixedOffset>> = None;

    for event in sorted_events {
        // todo: handle validation in one place somewhere, and not here
        let datetime = DateTime::parse_from_rfc3339(&event.datetime)
            .expect("Invalid RFC3339 datetime in work event");
        let event_name = event
            .event
            .parse::<EventName>()
            .expect("Invalid event name in work event");

        match event_name {
            EventName::StartWork => {
                start_time = Some(datetime);
            }
            EventName::StopWork => {
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

#[cfg(test)]
mod tests {
    use super::{compute_stats_from_events, compute_workday_duration, filter_month_events};
    use crate::events::models::{Event, EventName};
    use chrono::DateTime;

    fn make_event(day: &str, uid: &str) -> Event {
        Event {
            datetime: format!("{}T09:00:00+00:00", day),
            event: "START_WORK".to_string(),
            day: day.to_string(),
            not_type: "work".to_string(),
            uid: uid.to_string(),
        }
    }

    fn make_event_at(event_name: EventName, datetime: &str, uid: &str) -> Event {
        let dt = DateTime::parse_from_rfc3339(datetime).unwrap();
        Event {
            datetime: datetime.to_string(),
            event: event_name.to_string(),
            day: dt.format("%Y-%m-%d").to_string(),
            not_type: "work".to_string(),
            uid: uid.to_string(),
        }
    }

    #[test]
    fn filter_month_events_keeps_only_requested_month() {
        let events = vec![
            make_event("2026-08-01", "a"),
            make_event("2026-08-15", "b"),
            make_event("2026-07-31", "c"),
            make_event("2026-09-01", "d"),
        ];

        let filtered = filter_month_events("2026-08", events);

        assert_eq!(filtered.len(), 2);
        assert!(filtered.iter().all(|e| e.day.starts_with("2026-08")));
    }

    #[test]
    fn filter_month_events_returns_empty_when_no_match() {
        let events = vec![make_event("2026-07-31", "a"), make_event("2026-09-01", "b")];

        let filtered = filter_month_events("2026-08", events);

        assert!(filtered.is_empty());
    }

    #[test]
    fn filter_month_events_handles_empty_input() {
        let events: Vec<Event> = Vec::new();

        let filtered = filter_month_events("2026-08", events);

        assert!(filtered.is_empty());
    }

    #[test]
    fn filter_month_events_preserves_input_order() {
        let events = vec![
            make_event("2026-08-20", "first"),
            make_event("2026-08-01", "second"),
            make_event("2026-08-10", "third"),
        ];

        let filtered = filter_month_events("2026-08", events);

        let uids: Vec<String> = filtered.into_iter().map(|e| e.uid).collect();
        assert_eq!(uids, vec!["first", "second", "third"]);
    }

    #[test]
    fn filter_month_events_keeps_only_start_or_stop_work() {
        let mut start = make_event("2026-08-01", "start");
        start.event = "START_WORK".to_string();

        let mut stop = make_event("2026-08-01", "stop");
        stop.event = "STOP_WORK".to_string();

        let mut other = make_event("2026-08-01", "other");
        other.event = "CREATE_NOT".to_string();

        let events = vec![start, other, stop];
        let filtered = filter_month_events("2026-08", events);

        let kept: Vec<String> = filtered.into_iter().map(|e| e.uid).collect();
        assert_eq!(kept, vec!["start", "stop"]);
    }

    #[test]
    fn compute_workday_duration_single_session() {
        let events = vec![
            make_event_at(EventName::StartWork, "2026-08-05T09:00:00+00:00", "a"),
            make_event_at(EventName::StopWork, "2026-08-05T10:30:00+00:00", "b"),
        ];

        assert_eq!(compute_workday_duration(&events), 90);
    }

    #[test]
    fn compute_workday_duration_events_out_of_order() {
        let events = vec![
            make_event_at(EventName::StopWork, "2026-08-05T18:00:00+00:00", "a"),
            make_event_at(EventName::StartWork, "2026-08-05T09:00:00+00:00", "b"),
        ];

        assert_eq!(compute_workday_duration(&events), 9 * 60);
    }

    #[test]
    fn compute_stats_from_events_aggregates_days_and_total() {
        let events = vec![
            make_event_at(EventName::StartWork, "2026-08-05T09:00:00+00:00", "a"),
            make_event_at(EventName::StopWork, "2026-08-05T10:00:00+00:00", "b"),
            make_event_at(EventName::StartWork, "2026-08-06T10:00:00+00:00", "c"),
            make_event_at(EventName::StopWork, "2026-08-06T12:00:00+00:00", "d"),
        ];

        let stats = compute_stats_from_events(events);

        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }
}

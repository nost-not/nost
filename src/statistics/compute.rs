use crate::{
    events::{find::find_all_events_file_paths, models::Event},
    statistics::models::{MonthStats, Stats, WeekId, WeekStats},
};
use chrono::{DateTime, Datelike, Local, NaiveDate};
use log::debug;
use std::collections::{HashMap, HashSet};

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

    let month_str = date.format("%Y-%m").to_string();

    // get all events for the month
    let events = find_all_events_file_paths(Some(&month_str))
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No events found for month: {}", month_str), // todo: decide: an error of just a message?
            )
        })?
        .into_iter()
        .flat_map(|path| {
            std::fs::read_to_string(&path)
                .ok()
                .into_iter()
                .flat_map(|content| {
                    content
                        .lines()
                        .filter_map(|line| {
                            let trimmed = line.trim();
                            if trimmed.is_empty() {
                                None
                            } else {
                                serde_json::from_str::<Event>(trimmed).ok()
                            }
                        })
                        .collect::<Vec<Event>>()
                })
        })
        .collect::<Vec<Event>>();

    log::debug!("Loaded {} events from journal.", events.len());
    Ok(compute_stats_from_events(events))
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
        let length_in_minutes = compute_events_duration(day_events);

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
pub fn compute_events_duration(events: &[Event]) -> i32 {
    // Sort events by datetime
    let mut sorted_events = events.to_vec();
    sorted_events.sort_by(|a, b| a.start.cmp(&b.start));

    // compute total time between stop and start for each events
    events
        .iter()
        .filter_map(|event| {
            if let Some(stop) = &event.stop {
                let start_dt = DateTime::parse_from_rfc3339(&event.start).ok()?;
                let stop_dt = DateTime::parse_from_rfc3339(stop).ok()?;
                let duration = stop_dt.signed_duration_since(start_dt);
                Some(duration.num_minutes() as i32)
            } else {
                None
            }
        })
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::{compute_events_duration, compute_month_stats, compute_stats_from_events};
    use crate::events::models::Event;
    use std::{env, fs};
    use tempfile::tempdir;

    fn make_event(day: &str, start: &str, stop: Option<&str>) -> Event {
        Event {
            day: day.to_string(),
            start: start.to_string(),
            stop: stop.map(|value| value.to_string()),
        }
    }

    fn write_ndjson_events(base: &str, month: &str, events: &[Event]) {
        let events_dir = format!("{}/.nost/events", base);
        fs::create_dir_all(&events_dir).unwrap();
        let content = events
            .iter()
            .map(|event| serde_json::to_string(event).unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(format!("{}/{}.ndjson", events_dir, month), content).unwrap();
    }

    #[test]
    fn compute_events_duration_single_session() {
        let events = vec![make_event(
            "2026-08-05",
            "2026-08-05T09:00:00+00:00",
            Some("2026-08-05T10:30:00+00:00"),
        )];

        assert_eq!(compute_events_duration(&events), 90);
    }

    #[test]
    fn compute_events_duration_ignores_open_sessions() {
        let events = vec![
            make_event(
                "2026-08-05",
                "2026-08-05T09:00:00+00:00",
                Some("2026-08-05T10:00:00+00:00"),
            ),
            make_event("2026-08-05", "2026-08-05T14:00:00+00:00", None),
        ];

        assert_eq!(compute_events_duration(&events), 60);
    }

    #[test]
    fn compute_stats_from_events_aggregates_days_and_total() {
        let events = vec![
            make_event(
                "2026-08-05",
                "2026-08-05T09:00:00+00:00",
                Some("2026-08-05T10:00:00+00:00"),
            ),
            make_event(
                "2026-08-06",
                "2026-08-06T10:00:00+00:00",
                Some("2026-08-06T12:00:00+00:00"),
            ),
        ];

        let stats = compute_stats_from_events(events);

        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }

    #[test]
    fn compute_stats_from_events_merges_multiple_sessions_same_day() {
        let events = vec![
            make_event(
                "2026-08-05",
                "2026-08-05T09:00:00+00:00",
                Some("2026-08-05T10:00:00+00:00"),
            ),
            make_event(
                "2026-08-05",
                "2026-08-05T14:00:00+00:00",
                Some("2026-08-05T16:00:00+00:00"),
            ),
        ];

        let stats = compute_stats_from_events(events);

        assert_eq!(stats.total_work_days, 1);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.work_stats_by_week.len(), 1);
        let week_stats = stats.work_stats_by_week.values().next().unwrap();
        assert_eq!(week_stats.total_duration_in_minutes, 180);
        assert_eq!(week_stats.work_stats.len(), 1);
        assert_eq!(week_stats.work_stats[0].day, "2026-08-05");
        assert_eq!(week_stats.work_stats[0].length_in_minutes, 180);
    }

    #[test]
    #[serial_test::serial]
    fn compute_month_stats_reads_requested_month_from_ndjson_files() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());

        write_ndjson_events(
            dir.path().to_str().unwrap(),
            "2026-08",
            &[
                make_event(
                    "2026-08-05",
                    "2026-08-05T09:00:00+00:00",
                    Some("2026-08-05T10:00:00+00:00"),
                ),
                make_event(
                    "2026-08-06",
                    "2026-08-06T10:00:00+00:00",
                    Some("2026-08-06T12:00:00+00:00"),
                ),
            ],
        );
        write_ndjson_events(
            dir.path().to_str().unwrap(),
            "2026-09",
            &[make_event(
                "2026-09-01",
                "2026-09-01T09:00:00+00:00",
                Some("2026-09-01T17:00:00+00:00"),
            )],
        );

        let stats = compute_month_stats(Some("2026-08")).unwrap();

        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.work_stats_by_week.len(), 1);
    }
}

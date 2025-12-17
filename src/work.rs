use crate::annotation::extract_annotations_from_path;
use crate::annotation::filter_annotation_by_events;
use crate::annotation::Annotation;
use crate::not::compose_file_path;
use crate::not::NotEvent;
use chrono::Datelike;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct WorkStats {
    pub day: String, // in format "YYYY-MM-DD"
    pub length: i32, // in minutes
}

#[derive(Debug, Clone)]
pub struct WorkStatsByWeek {
    pub total_duration_in_minutes: i32,
    pub work_stats: Vec<WorkStats>,
}

#[derive(Debug, Clone)]
pub struct PeriodWorkStats {
    pub total_duration_in_minutes: i32,
    pub total_work_days: i32,
    pub work_stats_by_week: HashMap<WeekId, WorkStatsByWeek>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeekId {
    pub year: i32,
    pub week: u32,
}

pub fn compute_work_time(annotations: &Vec<Annotation>) -> i32 {
    let mut total_time_in_minutes = 0;
    let mut start_time = None;

    for annotation in annotations {
        match annotation.event {
            NotEvent::StartWork => {
                start_time = Some(annotation.datetime);
            }
            NotEvent::StopWork => {
                if let Some(start) = start_time {
                    total_time_in_minutes += (annotation.datetime - start).num_minutes() as i32;
                    start_time = None;
                }
            }
            _ => { /* ignore other events */ }
        }
    }
    total_time_in_minutes
}

pub fn compute_work_stats() -> Result<PeriodWorkStats, std::io::Error> {
    // get current month path
    let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOST_NOT_PATH environment variable not set.");
        panic!("NOST_NOT_PATH not set");
    });

    let pathes = Path::new(&compose_file_path(&not_path))
        .parent()
        .unwrap()
        .to_path_buf();

    // extract annotations and filter work annotations
    let annotations = match extract_annotations_from_path(pathes.clone()) {
        Ok(a) => a,
        Err(e) => {
            return Err(e);
        }
    };

    let work_annotations =
        filter_annotation_by_events(annotations, vec![NotEvent::StartWork, NotEvent::StopWork]);

    // group annotations by day using a hashmap
    let mut annotations_hmap: HashMap<String, Vec<Annotation>> = HashMap::new();
    for annotation in work_annotations {
        let day = annotation.datetime.format("%Y-%m-%d").to_string();
        annotations_hmap.entry(day).or_default().push(annotation);
    }

    // prepare work stats by week
    let mut work_stats_by_week: HashMap<WeekId, WorkStatsByWeek> = HashMap::new();
    let mut total_duration = 0;
    let mut worked_days_set = HashSet::new();

    // todo: adapt the code to export correct Period work stats
    for (day, annotation) in annotations_hmap.iter() {
        let length = compute_work_time(annotation);

        let date = chrono::NaiveDate::parse_from_str(day, "%Y-%m-%d").unwrap();
        let week_id = WeekId {
            year: date.iso_week().year(),
            week: date.iso_week().week(),
        };

        if let std::collections::hash_map::Entry::Vacant(e) = work_stats_by_week.entry(week_id) {
            e.insert(WorkStatsByWeek {
                total_duration_in_minutes: length,
                work_stats: vec![WorkStats {
                    day: day.clone(),
                    length,
                }],
            });
        } else {
            let week_stats = work_stats_by_week.get_mut(&week_id).unwrap();
            week_stats.total_duration_in_minutes += length;
            week_stats.work_stats.push(WorkStats {
                day: day.clone(),
                length,
            });
        }

        total_duration += length;

        // count the total of work days
        worked_days_set.insert(day.clone());
    }

    let monthly_stats = PeriodWorkStats {
        total_duration_in_minutes: total_duration,
        total_work_days: worked_days_set.len() as i32,
        work_stats_by_week,
    };

    log::debug!("Work stats computed: {:?}", monthly_stats);
    Ok(monthly_stats)
}

pub fn compose_work_stats(stats: PeriodWorkStats) -> String {
    let header =
        "\n| Day | Date       | Hours | Acc |\n|-----|------------|-------|-----|\n".to_string();
    let mut stats_content: String = String::new();

    // collect and sort weeks by date (year, then week)
    let mut sorted_weeks: Vec<(&WeekId, &WorkStatsByWeek)> =
        stats.work_stats_by_week.iter().collect();
    sorted_weeks
        .sort_by(|(a_id, _), (b_id, _)| a_id.year.cmp(&b_id.year).then(a_id.week.cmp(&b_id.week)));

    // for each week in work_stats_by_week add an header and then the stats
    for (_week_id, week_stats) in sorted_weeks {
        // Add week header
        stats_content.push_str(&header);

        // sort the days by day (ascending)
        let mut sorted_work_stats = week_stats.work_stats.clone();
        sorted_work_stats.sort_by(|a, b| a.day.cmp(&b.day));

        let mut cumulative_week_hours = 0.0;
        for work_stat in sorted_work_stats.iter() {
            let date = chrono::NaiveDate::parse_from_str(&work_stat.day, "%Y-%m-%d").unwrap();
            let weekday = date.weekday();
            let hours = work_stat.length as f32 / 60.0;
            cumulative_week_hours += hours;

            stats_content.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} |\n",
                &weekday, &work_stat.day, hours, cumulative_week_hours
            ));
        }
    }

    stats_content.push_str(&format!(
        "\n| Work Days | {}     |\n",
        stats.total_work_days
    ));
    stats_content.push_str(&format!(
        "| Total     | {:.2} |\n",
        stats.total_duration_in_minutes as f32 / 60.0
    ));

    let daily_rate: f32 = env::var("NOST_WORK_SALARY")
        .unwrap_or_else(|_| {
            eprintln!("NOST_WORK_SALARY environment variable not set.");
            "0".to_string()
        })
        .parse()
        .unwrap_or(0.0);

    let currency = env::var("NOST_WORK_CURRENCY").unwrap_or_else(|_| {
        eprintln!("NOST_WORK_CURRENCY environment variable not set.");
        "EUR".to_string()
    });

    let salary = if stats.total_work_days > 0 {
        daily_rate * stats.total_work_days as f32
    } else {
        0.0
    };

    stats_content.push_str(&format!("| Salary    | {:.2} {} |\n", salary, currency));

    stats_content
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local, TimeZone};
    use uuid::Uuid;

    #[test]
    #[serial_test::serial]
    fn test_compute_work_time() {
        let start = Local::now();
        let stop = start + Duration::hours(1);
        let start_annotation = Annotation {
            _uid: Uuid::new_v4(),
            event: NotEvent::StartWork,
            datetime: start,
        };

        let stop_annotation = Annotation {
            _uid: Uuid::new_v4(),
            event: NotEvent::StopWork,
            datetime: stop,
        };
        let annotations = vec![start_annotation, stop_annotation];
        assert_eq!(compute_work_time(&annotations), 60);
    }

    fn make_annotation(event: NotEvent, datetime: chrono::DateTime<Local>) -> Annotation {
        Annotation {
            _uid: Uuid::new_v4(),
            event,
            datetime,
        }
    }

    // todo: check if we really need this function in tests and why not use compute_work_time
    fn compute_work_stats_from_annotations(annotations: Vec<Annotation>) -> PeriodWorkStats {
        // group annotations by day using a hashmap
        let mut annotations_hmap: HashMap<String, Vec<Annotation>> = HashMap::new();
        for annotation in annotations {
            let day = annotation.datetime.format("%Y-%m-%d").to_string();
            annotations_hmap.entry(day).or_default().push(annotation);
        }

        let mut work_stats_by_week: HashMap<WeekId, WorkStatsByWeek> = HashMap::new();
        let mut total_duration = 0;
        let mut worked_days_set = HashSet::new();

        for (day, annotation) in annotations_hmap.iter() {
            let length = compute_work_time(annotation);

            let date = chrono::NaiveDate::parse_from_str(&day, "%Y-%m-%d").unwrap();
            let week_id = WeekId {
                year: date.iso_week().year(),
                week: date.iso_week().week(),
            };

            if work_stats_by_week.contains_key(&week_id) {
                let week_stats = work_stats_by_week.get_mut(&week_id).unwrap();
                week_stats.total_duration_in_minutes += length;
                week_stats.work_stats.push(WorkStats {
                    day: day.clone(),
                    length,
                });
            } else {
                work_stats_by_week.insert(
                    week_id,
                    WorkStatsByWeek {
                        total_duration_in_minutes: length,
                        work_stats: vec![WorkStats {
                            day: day.clone(),
                            length,
                        }],
                    },
                );
            }

            total_duration += length;
            worked_days_set.insert(day.clone());
        }

        PeriodWorkStats {
            total_duration_in_minutes: total_duration,
            total_work_days: worked_days_set.len() as i32,
            work_stats_by_week,
        }
    }

    #[test]
    fn test_compute_work_stats_single_day() {
        let start = Local.with_ymd_and_hms(2025, 9, 1, 9, 0, 0).unwrap();
        let stop = start + Duration::hours(1);
        let annotations = vec![
            make_annotation(NotEvent::StartWork, start),
            make_annotation(NotEvent::StopWork, stop),
        ];
        let stats = compute_work_stats_from_annotations(annotations);
        assert_eq!(stats.total_duration_in_minutes, 60);
        assert_eq!(stats.total_work_days, 1);
        assert_eq!(stats.work_stats_by_week.len(), 1);
        let week_stats = stats.work_stats_by_week.values().next().unwrap();
        assert_eq!(week_stats.work_stats.len(), 1);
        assert_eq!(week_stats.work_stats[0].length, 60);
    }

    #[test]
    fn test_compute_work_stats_multiple_days() {
        let start1 = Local.with_ymd_and_hms(2025, 9, 1, 9, 0, 0).unwrap();
        let stop1 = start1 + Duration::hours(1);
        let start2 = Local.with_ymd_and_hms(2025, 9, 2, 10, 0, 0).unwrap();
        let stop2 = start2 + Duration::hours(2);
        let annotations = vec![
            make_annotation(NotEvent::StartWork, start1),
            make_annotation(NotEvent::StopWork, stop1),
            make_annotation(NotEvent::StartWork, start2),
            make_annotation(NotEvent::StopWork, stop2),
        ];
        let stats = compute_work_stats_from_annotations(annotations);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.work_stats_by_week.len(), 1);
        let week_stats = stats.work_stats_by_week.values().next().unwrap();
        assert_eq!(week_stats.work_stats.len(), 2);
        let lengths: Vec<i32> = week_stats.work_stats.iter().map(|ws| ws.length).collect();
        assert!(lengths.contains(&60));
        assert!(lengths.contains(&120));
    }

    #[test]
    fn test_compute_work_stats_multiple_weeks() {
        let start1 = Local.with_ymd_and_hms(2025, 8, 31, 9, 0, 0).unwrap(); // week 35
        let stop1 = start1 + Duration::hours(1);
        let start2 = Local.with_ymd_and_hms(2025, 9, 1, 10, 0, 0).unwrap(); // week 36
        let stop2 = start2 + Duration::hours(2);
        let annotations = vec![
            make_annotation(NotEvent::StartWork, start1),
            make_annotation(NotEvent::StopWork, stop1),
            make_annotation(NotEvent::StartWork, start2),
            make_annotation(NotEvent::StopWork, stop2),
        ];
        let stats = compute_work_stats_from_annotations(annotations);
        assert_eq!(stats.total_duration_in_minutes, 180);
        assert_eq!(stats.total_work_days, 2);
        assert_eq!(stats.work_stats_by_week.len(), 2);
    }
}

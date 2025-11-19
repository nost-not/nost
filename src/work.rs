use crate::annotation::extract_annotations_from_path;
use crate::annotation::filter_annotation_by_events;
use crate::not::append;
use crate::not::compose_file_path;

// use regex::Regex;
use crate::annotation::Annotation;
use crate::not::get_or_create_not;
use crate::not::NotEvent;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::path::PathBuf;

use chrono::Datelike;

#[derive(Debug)]
pub struct WorkStats {
    pub day: String, // in format "YYYY-MM-DD"
    pub length: i32, // in minutes
}

#[derive(Debug)]
pub struct MonthlyWorkStats {
    pub total_duration_in_minutes: i32,
    pub total_work_days: i32,
    pub work_stats: Vec<WorkStats>,
}
pub fn get_salary() -> String {
    env::var("NOST_WORK_SALARY").unwrap_or_else(|_| {
        eprintln!("NOST_WORK_SALARY environment variable not set.");
        "0".to_string()
    })
}
pub fn get_salary_currency() -> String {
    env::var("NOST_WORK_CURRENCY").unwrap_or_else(|_| {
        eprintln!("NOST_WORK_CURRENCY environment variable not set.");
        "EUR".to_string()
    })
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

pub fn compute_work_stats() -> Result<MonthlyWorkStats, std::io::Error> {
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

    // compute work time for each day
    let mut work_stats: Vec<WorkStats> = Vec::new();
    let mut total_duration = 0;
    for (day, annotation) in annotations_hmap.iter() {
        let length = compute_work_time(annotation);
        work_stats.push(WorkStats {
            day: (day.clone()).to_string(),
            length,
        });
        total_duration += length;
    }

    let monthly_stats = MonthlyWorkStats {
        total_duration_in_minutes: total_duration,
        total_work_days: work_stats.len() as i32,
        work_stats,
    };

    println!("Work stats computed: {:?}", monthly_stats);
    Ok(monthly_stats)
}

pub fn compose_work_stats(stats: MonthlyWorkStats) -> String {
    let header =
        "\n| Day | Date       | Hours | Acc |\n|-----|------------|-------|-------|\n".to_string();
    let mut stats_content: String = String::new();

    // Sort work_stats alphabetically by day
    let mut sorted_stats = stats.work_stats;
    sorted_stats.sort_by(|a, b| a.day.cmp(&b.day));

    let mut current_week = None;
    let mut cumulative_week_hours = 0.0;

    for work_stat in sorted_stats {
        let date = chrono::NaiveDate::parse_from_str(&work_stat.day, "%Y-%m-%d").unwrap();
        let weekday = date.weekday();
        let week = date.iso_week().week();
        let year = date.iso_week().year();

        if current_week != Some((year, week)) {
            cumulative_week_hours = 0.0;
            // todo: add the work day length to the cumulative week hours
            current_week = Some((year, week));
        }

        if weekday == chrono::Weekday::Mon {
            stats_content.push_str(&header);
        }

        let hours = work_stat.length as f32 / 60.0;

        stats_content.push_str(&format!(
            "| {} | {} | {:.2} | {:.2} |\n",
            &weekday, &work_stat.day, hours, cumulative_week_hours
        ));
    }

    let total_hours = stats.total_duration_in_minutes as f32 / 60.0;
    stats_content.push_str(&format!("\n| Total     | {:.2} |\n", total_hours));
    stats_content.push_str(&format!("| Work Days | {}     |\n", stats.total_work_days));

    let daily_rate: f32 = get_salary().parse().unwrap_or(0.0);
    let currency = get_salary_currency();
    let salary = if stats.total_work_days > 0 {
        daily_rate * stats.total_work_days as f32
    } else {
        0.0
    };

    stats_content.push_str(&format!("| Salary    | {:.2} {} |\n", salary, currency));
    stats_content
}

pub fn display_work_stats(stats_content: String, in_not: bool) {
    if in_not {
        let file_path = get_or_create_not(None).unwrap();
        let _ = append(PathBuf::from(file_path), &stats_content);
        println!("Stats appended to the current not.");
    } else {
        println!("{}", stats_content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial_test::serial]
    fn test_get_salary_env_set() {
        env::set_var("NOST_WORK_SALARY", "1234");
        assert_eq!(get_salary(), "1234");
        env::remove_var("NOST_WORK_SALARY");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_salary_env_not_set() {
        env::remove_var("NOST_WORK_SALARY");
        assert_eq!(get_salary(), "0");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_salary_currency_env_set() {
        env::set_var("NOST_WORK_CURRENCY", "USD");
        assert_eq!(get_salary_currency(), "USD");
        env::remove_var("NOST_WORK_CURRENCY");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_salary_currency_env_not_set() {
        env::remove_var("NOST_WORK_CURRENCY");
        assert_eq!(get_salary_currency(), "EUR");
    }

    #[test]
    #[serial_test::serial]
    fn test_compute_work_time() {
        use chrono::{Duration, Local};
        use uuid::Uuid;

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

    #[test]
    #[serial_test::serial]
    fn test_compose_work_stats() {
        env::set_var("NOST_WORK_SALARY", "500");
        env::set_var("NOST_WORK_CURRENCY", "EUR");

        let stats = MonthlyWorkStats {
            total_duration_in_minutes: 120,
            total_work_days: 2,
            work_stats: vec![
                WorkStats {
                    day: "2025-09-01".to_string(),
                    length: 60,
                },
                WorkStats {
                    day: "2025-09-02".to_string(),
                    length: 60,
                },
            ],
        };

        let content = compose_work_stats(stats);
        assert!(content.contains("| Day | Date       | Hours | Acc |"));
        assert!(content.contains("| 2025-09-01 | 1.00 |"));
        assert!(content.contains("| 2025-09-02 | 1.00 |"));
        assert!(content.contains("| Total     | 2.00 |"));
        assert!(content.contains("| Work Days | 2     |"));
        assert!(content.contains("| Salary    | 1000.00 EUR |"));
    }
}

use crate::annotation::convert_into_annotation;
use crate::annotation::extract_annotations_from_path;
use crate::annotation::filter_annotation_by_events;
use crate::not::compose_file_path;
use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::not::get_or_create_not;

// use regex::Regex;
use crate::annotation::Annotation;
use crate::not::NotEvent;
use std::collections::HashMap;
use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct WorkStats {
    pub day: String, // in format "YYYY-MM-DD"
    pub length: i32, // in minutes
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

// for now we only handle pairs of start/stop work
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

pub fn compute_work_stats() -> Vec<WorkStats> {
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
    let annotations = extract_annotations_from_path(pathes);

    let work_annotations =
        filter_annotation_by_events(annotations, vec![NotEvent::StartWork, NotEvent::StopWork]);

    // group annotations by day using a hashmap
    let mut annotations_hmap: HashMap<String, Vec<Annotation>> = HashMap::new();
    for annotation in work_annotations {
        let day = annotation.datetime.format("%Y-%m-%d").to_string();
        annotations_hmap
            .entry(day)
            .or_insert_with(Vec::new)
            .push(annotation);
    }

    // compute work time for each day
    let mut work_stats: Vec<WorkStats> = Vec::new();
    for (day, annotation) in annotations_hmap.iter() {
        work_stats.push(WorkStats {
            day: (day.clone()).to_string(),
            length: compute_work_time(annotation),
        });
    }

    println!("Work stats computed: {:?}", work_stats);
    work_stats
}

pub fn display_work_stats(stats: Vec<WorkStats>) {
    // todo: implement this function to display work stats
}

// todo: to delete after moving in compute works stats
// pub fn generate_work_stats(path_as_string: &str) -> std::io::Result<String> {
//     let path = Path::new(path_as_string);

//     // get the current month path
//     let month_path = path.parent().unwrap().parent().unwrap();

//     println!("Folder path: {}", month_path.display());

//     // get all the path inside the month path
//     let pathes = get_not_pathes(month_path.to_path_buf()).unwrap();

//     // process the notes
//     for path in pathes {
//         let annotations = extract_annotations_from_one_file(&path);
//         println!("{:?}", annotations);
//     }

//     // define the stats
//     let stats_lines = "| 2025/08/12 | 4.5   | 1          |";
//     let stats = format!(
//         "\
// ## Work Stats

// | Day        | Hours | Cumulative |
// | ---------- | ----- | ---------- |
// | 2025/08/10 | 4.5   | 1          |
// | 2025/08/11 | 7     | 2          |
// {lines}
// | Total      | 11.5  | 2          |
// ",
//         lines = stats_lines
//     );

//     Ok(stats)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_salary_env_set() {
        env::set_var("NOST_WORK_SALARY", "1234");
        assert_eq!(get_salary(), "1234");
        env::remove_var("NOST_WORK_SALARY");
    }

    #[test]
    fn test_get_salary_env_not_set() {
        env::remove_var("NOST_WORK_SALARY");
        assert_eq!(get_salary(), "0");
    }

    #[test]
    fn test_get_salary_currency_env_set() {
        env::set_var("NOST_WORK_CURRENCY", "USD");
        assert_eq!(get_salary_currency(), "USD");
        env::remove_var("NOST_WORK_CURRENCY");
    }

    #[test]
    fn test_get_salary_currency_env_not_set() {
        env::remove_var("NOST_WORK_CURRENCY");
        assert_eq!(get_salary_currency(), "EUR");
    }

    #[test]
    fn test_compute_work_time() {
        use chrono::{Duration, Local};
        use uuid::Uuid;

        let start = Local::now();
        let stop = start + Duration::hours(1);
        let start_annotation = Annotation {
            uid: Uuid::new_v4(),
            event: NotEvent::StartWork,
            datetime: start,
        };

        let stop_annotation = Annotation {
            uid: Uuid::new_v4(),
            event: NotEvent::StopWork,
            datetime: stop,
        };
        let annotations = vec![start_annotation, stop_annotation];
        assert_eq!(compute_work_time(&annotations), 60);
    }
}

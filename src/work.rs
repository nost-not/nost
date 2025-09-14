use crate::not::convert_into_annotation;
use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::not::get_or_create_not;
use crate::not::Annotation;
// use regex::Regex;
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

pub fn compute_work_stats() -> Vec<WorkStats> {
    // get current month path
    // todo: get current month path without using get or create not
    let current_not_ref = get_or_create_not(None).unwrap();
    let current_path = Path::new(&current_not_ref);
    let month_path = current_path.parent().unwrap().parent().unwrap();

    // extract annotations
    let pathes = get_not_pathes(current_path.to_path_buf()).unwrap();

    let mut annotations = Vec::new();
    for path in pathes {
        let annotations_for_current_file = extract_annotations_from_one_file(&path).unwrap();
        annotations.extend(annotations_for_current_file);
    }

    // filter work annotations
    let mut work_annotations: Vec<String> = annotations
        .into_iter()
        .filter(|annotation| annotation.contains("start-work") || annotation.contains("stop-work"))
        .collect();

    // order the annotations by date
    // work_annotations.sort_by_key(|annotation| annotation.date);

    // first use a hashmap to store the work stats
    let mut annotations_hmap: HashMap<String, Annotation> = HashMap::new();

    // compute the work stats
    // todo: correct here, there is a problem with scope for annotations_hmap.insert
    work_annotations.iter().for_each(|annotation_text| {
        // convert string into annotation
        let annotation = convert_into_annotation(annotation_text);
        let day_in_string = annotation
            .datetime
            .to_string()
            .split(' ')
            .next()
            .unwrap()
            .to_string();
        annotations_hmap.insert(day_in_string, annotation);
    });

    let mut work_stats: Vec<WorkStats> = Vec::new();
    for (day, annotation) in annotations_hmap.iter() {
        let length = 0;
        // we need to handle the cases where start or stop time is missing
        // and the cases where there are multiple start or stop times in a day
        // if let (Some(start), Some(stop)) = (annotation.event, annotation.stop_time) {
        //     (stop - start) / 60
        // } else {
        //     0
        // };
        work_stats.push(WorkStats {
            day: (day.clone()).to_string(),
            length,
        });
    }

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
}

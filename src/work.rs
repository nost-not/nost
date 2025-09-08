use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::not::get_or_create_not;
use crate::work;
use regex::Regex;
use std::env;
use std::path::Path;

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

pub fn compute_work_stats() {
    // get current month path
    let current_not_ref = get_or_create_not(None).unwrap();
    let current_path = Path::new(&current_not_ref);

    let month_path = current_path.parent().unwrap().parent().unwrap();

    // get pathes
    let pathes = get_not_pathes(month_path.to_path_buf()).unwrap();

    // get annotations
    let mut annotations = Vec::new();
    for path in pathes {
        let annotations_for_current_file = extract_annotations_from_one_file(&path).unwrap();
        annotations.extend(annotations_for_current_file);
    }

    // clean the annotation
    let re = Regex::new(r#"\[//\]: # "not:(\{.*\})""#).unwrap();

    let cleaned_annotations: Vec<String> = annotations
        .into_iter()
        .filter_map(|annotation| {
            re.captures(&annotation)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        })
        .collect();

    // filter work
    let work_annotations: Vec<String> = cleaned_annotations
        .into_iter()
        .filter(|annotation| annotation.contains("start-work") || annotation.contains("stop-work"))
        .collect();

    work_annotations.iter().for_each(|annotation| {
        // from json to object
        // event: "start-work" or "stop-work"

        // if let Ok(annotation_object) = serde_json::from_str::<serde_json::Value>(annotation) {
        //     println!("{:?}", annotation_object);
        //     // Do something with the annotation object
        // }
    });

    // println!("{:?}", work_annotations);

    // todo: prepare works data
    // todo: then in another function, display the data
}

// todo: to delete after moving in compute works stats
pub fn generate_work_stats(path_as_string: &str) -> std::io::Result<String> {
    let path = Path::new(path_as_string);

    // get the current month path
    let month_path = path.parent().unwrap().parent().unwrap();

    println!("Folder path: {}", month_path.display());

    // get all the path inside the month path
    let pathes = get_not_pathes(month_path.to_path_buf()).unwrap();

    // process the notes
    for path in pathes {
        let annotations = extract_annotations_from_one_file(&path);
        println!("{:?}", annotations);
    }

    // define the stats
    let stats_lines = "| 2025/08/12 | 4.5   | 1          |";
    let stats = format!(
        "\
## Work Stats

| Day        | Hours | Cumulative |
| ---------- | ----- | ---------- |
| 2025/08/10 | 4.5   | 1          |
| 2025/08/11 | 7     | 2          |
{lines}
| Total      | 11.5  | 2          |
",
        lines = stats_lines
    );

    Ok(stats)
}

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

use crate::not::get_not_pathes;
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

pub fn generate_work_stats(path_as_string: &str) -> std::io::Result<String> {
    let path = Path::new(path_as_string);

    // get the current month path
    let month_path = path.parent().unwrap().parent().unwrap();

    println!("Folder path: {}", month_path.display());

    // get all the path inside the month path
    let pathes = get_not_pathes(month_path.to_path_buf())
        .unwrap()
        .into_iter()
        .for_each(|p| println!("Found note: {}", p.display()));

    // process the notes
    // todo: extract the anntotations from each note

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

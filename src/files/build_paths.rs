use chrono::{Datelike, Local, NaiveDate};

use crate::dates::get::{get_week_of_month, get_week_of_month_for_date};

pub fn build_file_path_for_month(base_path: &str, date: chrono::NaiveDate) -> String {
    let year = date.year();
    let month = format!("{:02}", date.month());

    format!("{}/{}/{}/", base_path, year, month)
}

pub fn build_file_path_for_now(base_path: &str) -> String {
    let today: chrono::NaiveDate = Local::now().date_naive();
    build_file_path_for_date(base_path, today)
}

pub fn build_file_path_for_date(base_path: &str, date: chrono::NaiveDate) -> String {
    let year = date.year();
    let month = format!("{:02}", date.month());

    format!(
        "{}/{}/{}/{}/",
        base_path,
        year,
        month,
        get_week_of_month_for_date(date)
    )
}

pub fn build_folder_path(base_path: &str, date: NaiveDate) -> String {
    format!(
        "{}/{}/{}/{}/{}/",
        base_path,
        date.year(),
        date.month(),
        get_week_of_month(date),
        date.day()
    )
}

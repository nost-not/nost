use chrono::{Datelike, Local};

use crate::dates::get::get_week_of_month;

pub fn build_file_path_for_month(base_path: &str, date: chrono::NaiveDate) -> String {
    let year = date.year();
    let month = format!("{:02}", date.month());

    format!("{}/{}/{}/", base_path, year, month)
}

pub fn build_file_path_for_now(base_path: &str) -> String {
    let today: chrono::NaiveDate = Local::now().date_naive();
    let year = today.year();
    let month = format!("{:02}", today.month());

    format!("{}/{}/{}/{}/", base_path, year, month, get_week_of_month())
}

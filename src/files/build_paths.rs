use chrono::{Datelike, NaiveDate};

use crate::dates::get::get_week_of_month;

pub fn build_folder_path(base_path: &str, date: NaiveDate) -> String {
    format!(
        "{}/{}/{:02}/{}/{:02}/",
        base_path,
        date.year(),
        date.month(),
        get_week_of_month(date),
        date.day()
    )
}

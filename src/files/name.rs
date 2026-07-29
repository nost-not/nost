use chrono::{Datelike, Local, NaiveDate};

pub fn name() -> String {
    name_for_date(Local::now().date_naive())
}

pub fn name_for_date(date: NaiveDate) -> String {
    let day_of_month = date.day();
    format!("{:02}.md", day_of_month)
}

use chrono::{Datelike, Local, NaiveDate};

pub fn name() -> String {
    name_for_date(Local::now().date_naive())
}

// for legacy notes, return also the extension, e.g., "07.md"
pub fn name_for_date(date: NaiveDate) -> String {
    let day_of_month = date.day();
    format!("{:02}.md", day_of_month)
}

// return the day of the month as a string, e.g., "07"
pub fn name_from_iso_date_str(date_str: &str) -> String {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let day_of_month = date.day();
    format!("{:02}", day_of_month)
}

#[cfg(test)]
mod tests {
    use super::{name_for_date, name_from_iso_date_str};
    use chrono::NaiveDate;

    #[test]
    fn test_name_for_date_formats_single_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 8, 3).unwrap();
        let file_name = name_for_date(date);
        assert_eq!(file_name, "03.md");
    }

    #[test]
    fn test_name_for_date_formats_double_digit_day() {
        let date = NaiveDate::from_ymd_opt(2026, 8, 14).unwrap();
        let file_name = name_for_date(date);
        assert_eq!(file_name, "14.md");
    }

    #[test]
    fn test_name_from_iso_date_str_formats_single_digit_day() {
        let file_name = name_from_iso_date_str("2026-08-01");
        assert_eq!(file_name, "01");
    }

    #[test]
    fn test_name_from_iso_date_str_formats_double_digit_day() {
        let file_name = name_from_iso_date_str("2026-08-14");
        assert_eq!(file_name, "14");
    }
}

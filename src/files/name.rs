use chrono::{Datelike, NaiveDate};

// return the day of the month as a string, e.g., "07"
pub fn name_from_iso_date_str(date_str: &str) -> String {
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    let day_of_month = date.day();
    format!("{:02}", day_of_month)
}

#[cfg(test)]
mod tests {
    use super::name_from_iso_date_str;

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

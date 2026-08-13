// Validate a string as year-month in format YYYY-MM (01..12)
pub fn is_valid_month_string(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }

    let bytes = s.as_bytes();
    if bytes[4] != b'-' {
        return false;
    }

    let year = &s[0..4];
    let month = &s[5..7];

    if !year.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if !month.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    matches!(month.parse::<u32>(), Ok(m) if (1..=12).contains(&m))
}

pub fn is_valid_string_date(s: &str) -> bool {
    if s.len() != 10 {
        return false;
    }

    let bytes = s.as_bytes();
    if bytes[4] != b'-' || bytes[7] != b'-' {
        return false;
    }

    let year = &s[0..4];
    let month = &s[5..7];
    let day = &s[8..10];

    if !year.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if !month.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    if !day.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }

    matches!(month.parse::<u32>(), Ok(m) if (1..=12).contains(&m))
        && matches!(day.parse::<u32>(), Ok(d) if (1..=31).contains(&d))
}

#[cfg(test)]
mod tests {
    use super::{is_valid_month_string, is_valid_string_date};

    #[test]
    fn valid_month_string_is_accepted() {
        assert!(is_valid_month_string("2026-08"));
        assert!(is_valid_month_string("0001-01"));
        assert!(is_valid_month_string("9999-12"));
    }

    #[test]
    fn invalid_month_format_is_rejected() {
        assert!(!is_valid_month_string("2026-8"));
        assert!(!is_valid_month_string("26-08"));
        assert!(!is_valid_month_string("2026/08"));
        assert!(!is_valid_month_string("abcd-08"));
    }

    #[test]
    fn invalid_month_value_is_rejected() {
        assert!(!is_valid_month_string("2026-00"));
        assert!(!is_valid_month_string("2026-13"));
    }

    #[test]
    fn valid_date_string_is_accepted() {
        assert!(is_valid_string_date("2026-08-13"));
        assert!(is_valid_string_date("2024-02-29"));
        assert!(is_valid_string_date("9999-12-31"));
    }

    #[test]
    fn invalid_date_format_is_rejected() {
        assert!(!is_valid_string_date("2026-8-13"));
        assert!(!is_valid_string_date("26-08-13"));
        assert!(!is_valid_string_date("2026/08/13"));
        assert!(!is_valid_string_date("abcd-08-13"));
        assert!(!is_valid_string_date("2026-08-ab"));
    }

    #[test]
    fn invalid_date_month_or_day_range_is_rejected() {
        assert!(!is_valid_string_date("2026-00-10"));
        assert!(!is_valid_string_date("2026-13-10"));
        assert!(!is_valid_string_date("2026-08-00"));
        assert!(!is_valid_string_date("2026-08-32"));
    }

    #[test]
    fn validator_is_format_and_range_only() {
        // This function does not validate real calendar rules.
        assert!(is_valid_string_date("2026-02-31"));
    }
}

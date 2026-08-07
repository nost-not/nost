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

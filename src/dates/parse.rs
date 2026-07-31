use chrono::NaiveDate;

/// Parse a date string in ISO 8601 format (YYYY-MM-DD) into a `NaiveDate`.
///
/// Returns a user-facing error message on failure, suitable for display.
pub fn parse_iso_date(s: &str) -> Result<NaiveDate, String> {
    // Enforce strict zero-padded YYYY-MM-DD (chrono's %Y-%m-%d otherwise
    // accepts unpadded values like "2026-7-31").
    if s.len() != 10 {
        return Err(format!("🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD", s));
    }
    NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|_| format!("🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD", s))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_date() {
        let result = parse_iso_date("2026-07-31");
        assert_eq!(result, Ok(NaiveDate::from_ymd_opt(2026, 7, 31).unwrap()));
    }

    #[test]
    fn rejects_invalid_format() {
        assert!(parse_iso_date("31/07/2026").is_err());
        assert!(parse_iso_date("2026-7-31").is_err());
        assert!(parse_iso_date("not-a-date").is_err());
        assert!(parse_iso_date("").is_err());
    }

    #[test]
    fn rejects_impossible_date() {
        assert!(parse_iso_date("2026-02-30").is_err());
        assert!(parse_iso_date("2026-13-01").is_err());
    }

    #[test]
    fn error_message_mentions_expected_format() {
        let err = parse_iso_date("bad").unwrap_err();
        assert!(err.contains("YYYY-MM-DD"));
    }
}

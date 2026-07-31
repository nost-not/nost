use chrono::NaiveDate;
use regex::Regex;
use std::sync::LazyLock;

static ISO_DATE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").expect("valid ISO date regex"));

/// Parses an ISO 8601 date (`YYYY-MM-DD`) into a `NaiveDate`.
///
/// Returns a user-facing error message if parsing fails.
pub fn parse_iso_date(date_str: &str) -> Result<NaiveDate, String> {
    if !ISO_DATE_REGEX.is_match(date_str) {
        return Err(format!(
            "🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD",
            date_str
        ));
    }
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| {
        format!(
            "🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD",
            date_str
        )
    })
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

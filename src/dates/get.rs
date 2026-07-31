use chrono::{DateTime, Datelike, Local};

pub fn get_now_as_string() -> String {
    let now: DateTime<Local> = Local::now();
    format!("{}{}", now.format("%Y-%m-%dT%H:%M:%S"), now.format("%:z"))
}

pub fn get_day_as_string(datetime: DateTime<Local>) -> String {
    format!("{}", datetime.format("%d"))
}

pub fn get_week_of_month() -> u32 {
    let today = chrono::Local::now().date_naive();
    get_week_of_month_for_date(today)
}

pub fn get_week_of_month_for_date(date: chrono::NaiveDate) -> u32 {
    // Get the first day of the month
    let first_of_month = chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap();

    // Get the weekday of the first day (0 = Monday, 6 = Sunday)
    let first_weekday = first_of_month.weekday().num_days_from_monday();

    // Calculate days since the first Monday of the month
    let days_since_first_monday = (date.day() - 1) + first_weekday;

    (days_since_first_monday / 7) + 1
}

fn get_day_suffix(day: u32) -> &'static str {
    match day {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

pub fn get_date_as_text_en(datetime: DateTime<Local>) -> String {
    let weekday = datetime.format("%A").to_string(); // e.g., "Thursday"
    let day = datetime.day(); // e.g., 7
    let month = datetime.format("%B").to_string(); // e.g., "August"
    let year = datetime.year(); // e.g., 2025

    let suffix = get_day_suffix(day); // e.g., "th"
    let formatted_date = format!("{}, {} {}{}, {}", weekday, month, day, suffix, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_date_as_text_fr(datetime: DateTime<Local>) -> String {
    let weekdays = [
        "Dimanche", "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi",
    ];
    let months = [
        "janvier",
        "février",
        "mars",
        "avril",
        "mai",
        "juin",
        "juillet",
        "août",
        "septembre",
        "octobre",
        "novembre",
        "décembre",
    ];

    let weekday = weekdays[datetime.weekday().num_days_from_sunday() as usize];
    let day = datetime.day();
    let month = months[(datetime.month() - 1) as usize];
    let year = datetime.year();

    let formatted_date = format!("{} {} {} {}", weekday, day, month, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(y: i32, m: u32, d: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(y, m, d).unwrap()
    }

    // July 2026: the 1st is a Wednesday. Weeks start on Monday, so the first
    // (partial) week is Wed 1 -> Sun 5, and week 2 starts Mon 6.
    #[test]
    fn first_day_of_month_is_week_one() {
        assert_eq!(get_week_of_month_for_date(date(2026, 7, 1)), 1);
    }

    #[test]
    fn last_day_of_first_partial_week_is_week_one() {
        // Sunday July 5th still belongs to week 1.
        assert_eq!(get_week_of_month_for_date(date(2026, 7, 5)), 1);
    }

    #[test]
    fn first_monday_starts_week_two() {
        // Monday July 6th is the start of week 2.
        assert_eq!(get_week_of_month_for_date(date(2026, 7, 6)), 2);
    }

    #[test]
    fn end_of_month_week_number() {
        // Friday July 31st -> week 5.
        assert_eq!(get_week_of_month_for_date(date(2026, 7, 31)), 5);
    }

    // June 2026: the 1st is a Monday, so week boundaries align cleanly.
    #[test]
    fn month_starting_on_monday() {
        assert_eq!(get_week_of_month_for_date(date(2026, 6, 1)), 1);
        assert_eq!(get_week_of_month_for_date(date(2026, 6, 7)), 1);
        assert_eq!(get_week_of_month_for_date(date(2026, 6, 8)), 2);
    }

    // February 2026: the 1st is a Sunday (worst case: first week is a single day).
    #[test]
    fn month_starting_on_sunday() {
        // Sunday Feb 1st is week 1 (alone), Monday Feb 2nd starts week 2.
        assert_eq!(get_week_of_month_for_date(date(2026, 2, 1)), 1);
        assert_eq!(get_week_of_month_for_date(date(2026, 2, 2)), 2);
    }
}

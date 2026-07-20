use chrono::{DateTime, Datelike, Local, NaiveDate};

pub fn get_now_as_string() -> String {
    let now: DateTime<Local> = Local::now();
    format!("{}{}", now.format("%Y-%m-%dT%H:%M:%S"), now.format("%:z"))
}

pub fn get_day_as_string(datetime: DateTime<Local>) -> String {
    format!("{}", datetime.format("%d"))
}

pub fn get_week_of_month() -> u32 {
    let today = chrono::Local::now().date_naive();

    // Get the first day of the month
    let first_of_month = chrono::NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();

    // Get the weekday of the first day (0 = Monday, 6 = Sunday)
    let first_weekday = first_of_month.weekday().num_days_from_monday();

    // Calculate days since the first Monday of the month
    // If month starts on Monday (0), offset is 0
    // If month starts on Tuesday (1), offset is 1, etc.
    let days_since_first_monday = (today.day() - 1) + first_weekday;

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

pub fn get_date_as_text_en(custom_date: Option<String>) -> String {
    let datetime = match custom_date {
        Some(date_str) => {
            // Parse YYYY-MM-DD format
            match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(naive_date) => {
                    naive_date.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Local).unwrap()
                }
                Err(_) => Local::now(),
            }
        }
        None => Local::now(),
    };

    let weekday = datetime.format("%A").to_string(); // e.g., "Thursday"
    let day = datetime.day(); // e.g., 7
    let month = datetime.format("%B").to_string(); // e.g., "August"
    let year = datetime.year(); // e.g., 2025

    let suffix = get_day_suffix(day); // e.g., "th"
    let formatted_date = format!("{}, {} {}{}, {}", weekday, month, day, suffix, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_date_as_text_fr(custom_date: Option<String>) -> String {
    let datetime = match custom_date {
        Some(date_str) => {
            // Parse YYYY-MM-DD format
            match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
                Ok(naive_date) => {
                    naive_date.and_hms_opt(0, 0, 0).unwrap().and_local_timezone(Local).unwrap()
                }
                Err(_) => Local::now(),
            }
        }
        None => Local::now(),
    };

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

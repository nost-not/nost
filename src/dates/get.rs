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

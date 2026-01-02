use crate::files::name_generator::name_file;
use crate::files::path_builder::compose_file_path_for_now;
use crate::nots::creator::create_not;
use chrono::Datelike;
use chrono::Local;
use std::env;
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn get_now_as_string() -> String {
    Local::now().to_rfc3339()
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

pub fn get_date_as_text_en() -> String {
    let now = Local::now();

    let weekday = now.format("%A").to_string(); // e.g., "Thursday"
    let day = now.day(); // e.g., 7
    let month = now.format("%B").to_string(); // e.g., "August"
    let year = now.year(); // e.g., 2025

    let suffix = get_day_suffix(day); // e.g., "th"
    let formatted_date = format!("{}, {} {}{}, {}", weekday, month, day, suffix, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_date_as_text_fr() -> String {
    let now = Local::now();

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

    let weekday = weekdays[now.weekday().num_days_from_sunday() as usize];
    let day = now.day();
    let month = months[(now.month() - 1) as usize];
    let year = now.year();

    let formatted_date = format!("{} {} {} {}", weekday, day, month, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_or_create_not(title: Option<String>) -> std::io::Result<String> {
    // get all existing notes

    match title {
        Some(title) => {
            // todo: check if not title is correct
            create_not(Some(title.clone())).unwrap();

            let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
                eprintln!("NOST_NOT_PATH environment variable not set.");
                panic!("NOST_NOT_PATH not set");
            });
            let not_file_path = compose_file_path_for_now(&not_path);
            let not_file_name = name_file();
            let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

            println!(
                "Using NOST_NOT_PATH in get_or_create_not: {}",
                full_not_file_path
            );

            Ok(full_not_file_path)
        }
        None => {
            let new_not_path = create_not(None);
            Ok(new_not_path.unwrap())
        }
    }
}

use chrono::Datelike;
use chrono::Local;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)?;

    writeln!(file, "{}", content)?;
    println!("Content appended successfully to {}", file_path.display());
    Ok(())
}

pub fn name_file() -> String {
    let today = Local::now().date_naive();
    let day_of_month = today.day();
    let file_name = format!("{:02}.md", day_of_month);
    file_name.to_string()
}

pub fn compose_file_path(base_path: &str) -> String {
    let today = Local::now().date_naive();
    let year = today.year();
    let month = format!("{:02}", today.month());

    format!("{}/temp/{}/{}/{}/", base_path, year, month, week_of_month())
}

fn week_of_month() -> u32 {
    let today = chrono::Local::now().date_naive();
    // Get the first day of the month
    let first_day = today.with_day(1).unwrap();
    // Calculate the week number for today and the first day of the month
    let week_today = today.iso_week().week();
    let week_first = first_day.iso_week().week();
    // Week of month is difference + 1
    week_today - week_first + 1
}

fn get_day_suffix(day: u32) -> &'static str {
    match day {
        11 | 12 | 13 => "th",
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

pub fn handle_not_command() {
    // handle pathes
    let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOST_NOT_PATH environment variable not set.");
        Err("NOST_NOT_PATH not set").unwrap()
    });

    println!("Using NOST_NOT_PATH: {}", not_path);

    let not_file_path = compose_file_path(&not_path);
    let not_file_name = name_file();
    let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

    // create folders if needed
    if let Err(e) = create_dir_all(&not_file_path) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    // only create the file if it does not exist
    if Path::new(&full_not_file_path).exists() {
        eprintln!(
            "This note already exists. Nothing has been done.: {}",
            full_not_file_path
        );
        return;
    }

    // create the file
    let mut _not_file = match File::create(&full_not_file_path) {
        Ok(_file) => {
            println!("File created at: {}", full_not_file_path);
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    };

    // add the not header metadata
    // eg: [//]: # "not:{uid: '5ef05459-9af2-4760-8f46-3262b49803fc', created_at: 2025-06-11 01:50:56 +02:00, version: '0.1.0'}"
    let not_info = format!(
        "\"not:{{uid: '{}', created_at: {}, version: '0.0.0'}}\"",
        uuid::Uuid::new_v4(),
        Local::now().format("%Y-%m-%d %H:%M:%S %Z")
    );
    let header = format!("[//]: # {}\n", not_info);
    append(full_not_file_path.clone().into(), &header).expect("Failed to append not metatadata.");

    // append the current date as text
    // let date = Local::now().format("%Y-%m-%d");
    // let date_line = format!("# {}\n", date);
    let date_line = match env::var("NOST_LANGUAGE")
        .unwrap_or_else(|_| "en".to_string())
        .as_str()
    {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to French
    };
    append(full_not_file_path.clone().into(), &date_line).expect("Failed to append date as text.");

    println!("File created at: {}", full_not_file_path);
}

use chrono::Datelike;
use chrono::Local;
use dotenv::dotenv;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

// todo: give the possibility to create a note for a specific day, eg: 2025-12-31
// todo: add the templates content to the file
// todo: add a version number for nost and for not
// todo: create a repo for not in md
// todo: correct the tests
fn main() {
    dotenv().ok();

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
    let date_line = get_date_as_text();

    append(full_not_file_path.clone().into(), &date_line).expect("Failed to append date as text.");

    println!("File created at: {}", full_not_file_path);
}

fn append(file_path: PathBuf, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)?;

    writeln!(file, "{}", content)?;
    println!("Content appended successfully to {}", file_path.display());
    Ok(())
}

fn name_file() -> String {
    let today = Local::now().date_naive();
    let day_of_month = today.day();
    let file_name = format!("{:02}.md", day_of_month);
    file_name.to_string()
}

fn compose_file_path(base_path: &str) -> String {
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

fn get_date_as_text() -> String {
    let now = Local::now();

    let weekday = now.format("%A").to_string(); // e.g., "Thursday"
    let day = now.day(); // e.g., 7
    let month = now.format("%B").to_string(); // e.g., "August"
    let year = now.year(); // e.g., 2025

    let suffix = get_day_suffix(day); // e.g., "th"
    let formatted_date = format!("{}, {} {}{}, {}", weekday, month, day, suffix, year);

    let date_line = format!("# {}\n", formatted_date);

    return date_line;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compose_file_path() {
        let expected_path = "./temp/not";
        assert_eq!(compose_file_path(), expected_path);
    }

    #[test]
    fn test_name_file() {
        let expected_name = "example.txt";
        assert_eq!(name_file(), expected_name);
    }

    #[test]
    fn test_file_creation() {
        let not_file_path = compose_file_path();
        let not_file_name = name_file();
        let full_not_file_path = format!("{}{}", not_file_path, not_file_name);

        // Ensure the directory exists
        assert!(create_dir_all(&not_file_path).is_ok());

        // Create the file
        let file_result = File::create(&full_not_file_path);
        assert!(file_result.is_ok());

        // Check if the file exists
        assert!(fs::metadata(&full_not_file_path).is_ok());

        // Cleanup
        fs::remove_file(&full_not_file_path).unwrap();
        fs::remove_dir_all(&not_file_path).unwrap();
    }
}

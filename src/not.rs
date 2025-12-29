use chrono::Datelike;
use chrono::Local;
use regex::Regex;
use std::env;
use std::fmt;
use std::fs::create_dir_all;
use std::fs::read_dir;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use crate::annotation::annotate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotEvent {
    StartWork,
    StopWork,
    CreateNot,
}

impl fmt::Display for NotEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotEvent::StartWork => write!(f, "START_WORK"),
            NotEvent::StopWork => write!(f, "STOP_WORK"),
            NotEvent::CreateNot => write!(f, "CREATE_NOT"),
        }
    }
}

impl std::str::FromStr for NotEvent {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "START_WORK" => Ok(NotEvent::StartWork),
            "STOP_WORK" => Ok(NotEvent::StopWork),
            "CREATE_NOT" => Ok(NotEvent::CreateNot),
            _ => Err(()),
        }
    }
}

pub fn extract_annotations_from_one_file(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let re = Regex::new(r#"^\[//\]: # "not.*"\s*$"#).unwrap();

    let extracted: Vec<String> = content
        .lines()
        .filter_map(|line| {
            if re.is_match(line) {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect();

    Ok(extracted)
}

pub fn get_not_pathes(path: PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut pathes: Vec<PathBuf> = vec![path];

    let folder_regex = Regex::new(r"^\d+$").unwrap();
    let file_regex = Regex::new(r".*\d+\.md$").unwrap();

    while let Some(current) = pathes.pop() {
        match read_dir(&current) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let current_path = entry.path();
                    if let Some(name) = current_path.file_name().and_then(|name| name.to_str()) {
                        if current_path.is_dir() {
                            if folder_regex.is_match(name) {
                                pathes.push(current_path);
                            }
                        } else if file_regex.is_match(name) {
                            files.push(current_path);
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        }
    }

    files.sort();

    Ok(files)
}

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn name_file() -> String {
    let today = Local::now().date_naive();
    let day_of_month = today.day();
    let file_name = format!("{:02}.md", day_of_month);
    file_name.to_string()
}

pub fn compose_file_path_for_now(base_path: &str) -> String {
    let today: chrono::NaiveDate = Local::now().date_naive();
    let year = today.year();
    let month = format!("{:02}", today.month());

    format!("{}/{}/{}/{}/", base_path, year, month, get_week_of_month())
}

pub fn get_now_as_string() -> String {
    Local::now().to_rfc3339()
}

fn get_week_of_month() -> u32 {
    let today = chrono::Local::now().date_naive();
    let day_of_month = today.day();
    let week_of_month = ((day_of_month - 1) / 7) + 1;

    println!("Week of month: {}", week_of_month);
    week_of_month
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

pub fn create_not(title: Option<String>) -> std::io::Result<String> {
    // handle pathes
    let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOST_NOT_PATH environment variable not set.");
        panic!("NOST_NOT_PATH not set");
    });

    println!("Using NOST_NOT_PATH: {}", not_path);

    let not_file_path = compose_file_path_for_now(&not_path);

    let not_file_name = match &title {
        Some(t) => t.clone(), // todo: validate t here
        None => name_file(),
    };

    let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

    // create folders if needed
    if let Err(e) = create_dir_all(&not_file_path) {
        return Err(Error::other(format!(
            "🛑 Failed to create directory: {}",
            e
        )));
    }

    // only create the file if it does not exist
    if Path::new(&full_not_file_path).exists() {
        println!("Not already existed.");
        return Ok(full_not_file_path);
    }

    // create the file
    match File::create(&full_not_file_path) {
        Ok(_file) => {
            println!("✅ File created: {}", full_not_file_path);
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    };

    annotate(
        None,
        NotEvent::CreateNot,
        None,
        full_not_file_path.as_str(),
        None,
    );

    let date_line = match env::var("NOST_LANGUAGE")
        .unwrap_or_else(|_| "en".to_string())
        .as_str()
    {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to French
    };
    append(full_not_file_path.clone().into(), &date_line)
        .expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(full_not_file_path)
}

#[cfg(test)]
mod tests {
    #[test]
    #[serial_test::serial]
    fn test_get_not_pathes() {
        use std::fs::{self, File};
        use std::io::Write;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Create subfolders and files
        let week_folder = base.join("1");
        fs::create_dir(&week_folder).unwrap();

        let file1 = week_folder.join("01.md");
        let file2 = week_folder.join("02.md");
        let file3 = week_folder.join("not_a_note.txt");

        File::create(&file1).unwrap().write_all(b"note 1").unwrap();
        File::create(&file2).unwrap().write_all(b"note 2").unwrap();
        File::create(&file3)
            .unwrap()
            .write_all(b"not a note")
            .unwrap();

        // Should find only .md files in numeric folders
        let found = super::get_not_pathes(base.to_path_buf()).unwrap();

        let found_files: Vec<_> = found
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        assert!(found_files.contains(&"01.md".to_string()));
        assert!(found_files.contains(&"02.md".to_string()));
        assert!(!found_files.contains(&"not_a_note.txt".to_string()));
        assert_eq!(found_files.len(), 2);
    }
}

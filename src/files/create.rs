use std::{
    fs::{create_dir_all, File},
    io::Error,
    path::Path,
};

use chrono::{DateTime, Local};

use crate::{
    configurations::get::get_value_from_config,
    dates::{
        get::{get_date_as_text_en, get_date_as_text_fr},
        parse::parse_iso_date,
        validate::is_valid_string_date,
    },
    files::{append::append, build_paths::build_folder_path, name::name_from_iso_date_str},
};

fn resolve_file_date(date_in_string: Option<&str>, now: DateTime<Local>) -> String {
    match date_in_string {
        Some(date) if is_valid_string_date(date) => date.to_string(),
        _ => now.format("%Y-%m-%d").to_string(),
    }
}

pub fn create_note_file_with_folders(
    note_type: String,
    date_in_string: Option<String>,
) -> std::io::Result<String> {
    // get the path of the folder to create
    let not_path = get_value_from_config("not_path").unwrap();
    let file_date = resolve_file_date(date_in_string.as_deref(), Local::now());
    let file_name = name_from_iso_date_str(&file_date);
    let file_naive_date = parse_iso_date(&file_date).unwrap();
    let today_folder_path = build_folder_path(&not_path, file_naive_date);

    log::debug!(
        "🚨 Creating note file with folders at path: {}",
        today_folder_path
    );

    let today_file_path = format!(
        "{}{}{}{}{}",
        today_folder_path, file_name, ".", note_type, ".md"
    );

    // only create if not does not already exists
    if Path::new(&today_file_path).exists() {
        println!("Not already existed.");
        return Ok(today_file_path);
    }

    // create folders if needed
    if let Err(e) = create_dir_all(&today_folder_path) {
        return Err(Error::other(format!(
            "🛑 Failed to create directory: {}",
            e
        )));
    }

    log::debug!(
        "🚨 Creating note file with folders at path: {}",
        today_file_path
    );

    // create the file
    match File::create(&today_file_path) {
        Ok(_file) => {
            println!("✅ File created: {}", today_file_path);
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    };

    let date_line = match get_value_from_config("language").unwrap().as_str() {
        "fr" => get_date_as_text_fr(Local::now()),
        _ => get_date_as_text_en(Local::now()), // default to English
    };

    append(today_file_path.clone().into(), &date_line).expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(today_file_path)
}

#[cfg(test)]
mod tests {
    use super::resolve_file_date;
    use chrono::{Local, TimeZone};

    #[test]
    fn test_resolve_file_date_keeps_valid_input() {
        let now = Local
            .with_ymd_and_hms(2026, 8, 13, 12, 0, 0)
            .single()
            .unwrap();
        let resolved = resolve_file_date(Some("2026-08-01"), now);
        assert_eq!(resolved, "2026-08-01");
    }

    #[test]
    fn test_resolve_file_date_falls_back_on_invalid_input() {
        let now = Local
            .with_ymd_and_hms(2026, 8, 13, 12, 0, 0)
            .single()
            .unwrap();
        let resolved = resolve_file_date(Some("20260801"), now);
        assert_eq!(resolved, "2026-08-13");
    }

    #[test]
    fn test_resolve_file_date_falls_back_when_missing() {
        let now = Local
            .with_ymd_and_hms(2026, 8, 13, 12, 0, 0)
            .single()
            .unwrap();
        let resolved = resolve_file_date(None, now);
        assert_eq!(resolved, "2026-08-13");
    }
}

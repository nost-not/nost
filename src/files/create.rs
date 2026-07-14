use std::{
    fs::{create_dir_all, File},
    io::Error,
    path::Path,
};

use chrono::{DateTime, Local};

use crate::{
    annotations::annotate::annotate,
    configurations::get::get_value_from_config,
    dates::get::{get_date_as_text_en, get_date_as_text_fr, get_day_as_string},
    events::{
        models::{Event, EventName},
        record::record_event,
    },
    files::{
        append::append,
        build_paths::{build_file_path_for_now, build_folder_path_for_now},
        name::name,
    },
};

pub fn create_file(title: Option<String>) -> std::io::Result<String> {
    // handle paths
    let not_path = get_value_from_config("not_path").unwrap();
    let not_file_path = build_file_path_for_now(&not_path);

    let not_file_name = match &title {
        Some(file_title) => file_title.clone(), // todo: validate title here
        None => name(),
    };

    let full_not_file_path = format!("{}{}", not_file_path, not_file_name);

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
        EventName::CreateNot,
        None,
        full_not_file_path.as_str(),
        None,
    );

    let date_line = match get_value_from_config("language").unwrap().as_str() {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to English
    };

    append(full_not_file_path.clone().into(), &date_line)
        .expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(full_not_file_path)
}

pub fn create_note_file_with_folders(note_type: String) -> std::io::Result<String> {
    // get the path of the folder to create
    let not_path = get_value_from_config("not_path").unwrap();
    let today_folder_path = build_folder_path_for_now(&not_path);

    log::debug!(
        "🚨 Creating note file with folders at path: {}",
        today_folder_path
    );

    let now: DateTime<Local> = Local::now();
    let today_file_name = get_day_as_string(now);
    let today_file_path = format!(
        "{}{}{}{}{}",
        today_folder_path, today_file_name, ".", note_type, ".md"
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
            record_event(Event::now(EventName::CreateNot, note_type.clone()))?;
            println!("✅ File created: {}", today_file_path);
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    };

    let date_line = match get_value_from_config("language").unwrap().as_str() {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to English
    };

    append(today_file_path.clone().into(), &date_line).expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(today_file_path)
}

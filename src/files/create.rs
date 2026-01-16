use std::{
    fs::{create_dir_all, File},
    io::Error,
    path::Path,
};

use crate::{
    annotations::annotate::annotate,
    configurations::get::get_value_from_config,
    dates::get::{get_date_as_text_en, get_date_as_text_fr},
    events::models::NotEvent,
    files::{append::append, build_paths::build_file_path_for_now, name::name},
};

pub fn create_note(title: Option<String>) -> std::io::Result<String> {
    // handle paths
    let not_path = get_value_from_config("not_path").unwrap();
    let not_file_path = build_file_path_for_now(&not_path);

    let not_file_name = match &title {
        Some(file_title) => file_title.clone(), // todo: validate title here
        None => name(),
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

    let date_line = match get_value_from_config("language").unwrap().as_str() {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to English
    };

    append(full_not_file_path.clone().into(), &date_line)
        .expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(full_not_file_path)
}

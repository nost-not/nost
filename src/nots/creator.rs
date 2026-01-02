use std::{
    env,
    fs::{create_dir_all, File},
    io::Error,
    path::Path,
};

use crate::{
    annotations::annotate::annotate,
    events::models::NotEvent,
    files::{name_generator::name_file, path_builder::compose_file_path_for_now},
    not::{append, get_date_as_text_en, get_date_as_text_fr},
};

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

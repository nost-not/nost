use chrono::NaiveDate;

use crate::{
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};

pub fn new_legacy(args: Vec<String>) {
    // Optional date argument in YYYY-MM-DD format
    let date_input = args.get(2);

    let date = match date_input {
        Some(arg) => match NaiveDate::parse_from_str(arg, "%Y-%m-%d") {
            Ok(d) => Some(d),
            Err(_) => {
                eprintln!("🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD", arg);
                std::process::exit(1);
            }
        },
        None => None,
    };

    match date {
        Some(d) => println!("Creating legacy note for date: {}", d),
        None => println!("Creating legacy note for today..."),
    }

    create_file(date).unwrap();

    std::process::exit(0);
}

pub fn new() {
    println!("Creating new note for today...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string());

    println!("✅ Note has been created successfully!");
    std::process::exit(0);
}

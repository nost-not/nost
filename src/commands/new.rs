use chrono::NaiveDate;

use crate::{
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};

pub fn new_legacy(args: Vec<String>) {
    // Expect exactly one argument: a date in YYYY-MM-DD format
    let date_arg = args.get(2);

    let date = match date_arg {
        Some(arg) => match NaiveDate::parse_from_str(arg, "%Y-%m-%d") {
            Ok(d) => d,
            Err(_) => {
                eprintln!("🛑 Invalid date format: '{}'. Expected: YYYY-MM-DD", arg);
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("🛑 Missing date argument. Expected: YYYY-MM-DD");
            std::process::exit(1);
        }
    };

    println!("Creating legacy note for date: {}", date);

    create_file(None, Some(date)).unwrap();

    std::process::exit(0);
}

pub fn new() {
    println!("Creating new note for today...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string());

    println!("✅ Note has been created successfully!");
    std::process::exit(0);
}

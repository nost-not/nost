use crate::{
    dates::parse::parse_iso_date,
    events::{find::find_last_work_event, models::EventName},
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};
pub fn new_legacy(args: Vec<String>) {
    // Optional date argument in YYYY-MM-DD format
    let date = match args.get(2) {
        Some(arg) => match parse_iso_date(arg) {
            Ok(d) => Some(d),
            Err(msg) => {
                eprintln!("{}", msg);
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
}

pub fn new() {
    // Warn if a work session is still open
    if let Some(last) = find_last_work_event() {
        if last.event == EventName::StartWork.to_string() {
            eprintln!("⚠️  Warning: you should first end the last work session before creating a new note.");
        }
    }

    println!("Creating new note for today...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string());

    println!("✅ Note has been created successfully!");
}

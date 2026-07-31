use crate::{
    dates::parse::parse_iso_date,
    events::{find::find_last_work_event, models::EventName},
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};

pub fn new_legacy(args: Vec<String>) {
    if args.len() > 2 {
        println!("Creating not with title: {}", args[1]);
        create_file(Some(args[2].clone())).unwrap();
    } else {
        create_file(None).unwrap();
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

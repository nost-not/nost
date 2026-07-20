use crate::{
    events::{find::find_last_work_event, models::EventName},
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};

pub fn new_legacy(args: Vec<String>) {
    // Warn if a work session is still open
    if let Some(last) = find_last_work_event() {
        if last.event == EventName::StartWork.to_string() {
            eprintln!("⚠️  Warning: you should first end the last work session before creating a new note.");
        }
    }

    // Parse command line arguments
    let mut title: Option<String> = None;
    let mut date: Option<String> = None;

    let mut i = 2;
    while i < args.len() {
        if args[i] == "--date" && i + 1 < args.len() {
            date = Some(args[i + 1].clone());
            i += 2;
        } else {
            title = Some(args[i].clone());
            i += 1;
        }
    }

    if title.is_some() {
        println!("Creating not with title: {}", title.as_ref().unwrap());
    }
    if date.is_some() {
        println!("Creating note for date: {}", date.as_ref().unwrap());
    }

    create_file(title, date).unwrap();

    std::process::exit(0);
}

pub fn new() {
    println!("Creating new note for today...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string());

    println!("✅ Note has been created successfully!");
    std::process::exit(0);
}

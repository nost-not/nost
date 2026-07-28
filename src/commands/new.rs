use crate::{
    files::create::{create_file, create_note_file_with_folders},
    projects::initialize::initialize_project,
};

pub fn new_legacy(args: Vec<String>) {
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

    if let Some(ref t) = title {
        println!("Creating not with title: {}", t);
    }
    if let Some(ref d) = date {
        println!("Creating note for date: {}", d);
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

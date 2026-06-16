use crate::{files::create::create_file, projects::initialize::initialize_project};

pub fn new_legacy(args: Vec<String>) {
    if args.len() > 2 {
        println!("Creating not with title: {}", args[1]);
        create_file(Some(args[2].clone())).unwrap();
    } else {
        create_file(None).unwrap();
    }

    std::process::exit(0);
}

pub fn new() {
    println!("Creating new note for today...");
    let _ = initialize_project();
    // create_note_file_with_folder();
    // add_event_to_configuration(EventName::CreateNot);

    println!("✅ Note has been created successfully!");
    std::process::exit(0);
}

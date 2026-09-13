use crate::{
    files::create::create_note_file_with_folders, projects::initialize::initialize_project,
};

pub fn new(date_in_string: Option<String>) {
    println!("Creating new note...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string(), date_in_string);

    println!("✅ Note has been created successfully!");
}

use crate::{
    dates::parse::parse_iso_date,
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
    println!("Creating new note for today...");
    let _ = initialize_project();
    let _ = create_note_file_with_folders("default".to_string());

    println!("✅ Note has been created successfully!");
}

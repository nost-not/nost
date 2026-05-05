use crate::files::create::create_file;

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
    // compute path of the configuration
    // check if the configuration file exists, if not create it with default values

    // compute path of the day folder
    // check if the folder exists, if not create it

    // compute path of the day note
    // create the note if it doesn't exist

    // add minimal content to the note: the date and add an info in config file
    println!("Creating new note for today...");
    std::process::exit(0);
}

use crate::files::create::create_note;

pub fn new(args: Vec<String>) {
    if args.len() > 2 {
        println!("Creating not with title: {}", args[1]);
        create_note(Some(args[2].clone())).unwrap();
    } else {
        create_note(None).unwrap();
    }

    std::process::exit(0);
}

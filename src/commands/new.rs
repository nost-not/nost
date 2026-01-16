use crate::files::create::create_file;

pub fn new(args: Vec<String>) {
    if args.len() > 2 {
        println!("Creating not with title: {}", args[1]);
        create_file(Some(args[2].clone())).unwrap();
    } else {
        create_file(None).unwrap();
    }

    std::process::exit(0);
}

use crate::files::create::create_not;

pub fn new(args: Vec<String>) {
    if args.len() > 2 {
        println!("Creating not with title: {}", args[1]);
        create_not(Some(args[2].clone())).unwrap();
    } else {
        create_not(None).unwrap();
    }

    std::process::exit(0);
}

use dotenv::dotenv;
use std::env;

mod annotation;
mod not;
mod work;
use crate::not::get_or_create_not;
use crate::not::NotEvent;

// todo: give the possibility to create a note for a specific day, eg: 2025-12-31
// todo: add the templates content to the file
// todo: add a version number for nost and for not
fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("You need to pass at least one argument.");
        std::process::exit(1);
    }

    if args[1] == "not" {
        if args.len() > 2 {
            println!("Creating not with title: {}", args[1]);
            not::create_not(Some(args[2].clone())).unwrap();
        } else {
            not::create_not(None).unwrap();
        }
        std::process::exit(0);
    } else if args[1] == "start-work" {
        let not_path = get_or_create_not(None).unwrap();
        annotation::annotate(None, None, NotEvent::StartWork, None, &not_path);
        std::process::exit(0);
    } else if args[1] == "stop-work" {
        let not_path = get_or_create_not(None).unwrap();
        annotation::annotate(None, None, NotEvent::StopWork, None, &not_path);
        std::process::exit(0);
    } else if args[1] == "work-stats" {
        let stats = work::compute_work_stats();
        work::display_work_stats(stats);
        std::process::exit(0);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
}

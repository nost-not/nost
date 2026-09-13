mod commands;
mod configurations;
mod dates;
mod events;
mod files;
mod projects;
mod statistics;
use crate::commands::new::new;
use crate::commands::stats::stats;
use crate::commands::work::work;
use dotenv::dotenv;
use std::env;

// todo: add a version number for nost and for not
fn main() {
    dotenv().ok();
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("You need to pass at least one argument.");
        std::process::exit(1);
    } else if args[1] == "new" || args[1] == "n" {
        if args.len() > 2 {
            let date_arg = args[2].clone();
            new(Some(date_arg));
        } else {
            new(None);
        }
    } else if args[1] == "work" || args[1] == "w" {
        if args.len() > 2 {
            let date_arg = args[2].clone();
            work(Some(date_arg));
        } else {
            work(None);
        }
    } else if args[1] == "stats" || args[1] == "s" {
        stats(args);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
}

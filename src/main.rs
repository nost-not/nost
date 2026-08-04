mod annotations;
mod commands;
mod configurations;
mod dates;
mod events;
mod files;
mod plugins;
mod projects;
use crate::commands::new::{new, new_legacy};
use crate::commands::new_work_stats::new_work_stats;
use crate::commands::work::work;
use crate::plugins::gdarquie_work::commands::end_work::end_work;
use crate::plugins::gdarquie_work::commands::start_work::start_work;
use crate::plugins::gdarquie_work::commands::work_stats::work_stats;
use dotenv::dotenv;
use std::env;

// todo: give the possibility to create a note for a specific day, eg: 2025-12-31
// todo: add a version number for nost and for not
fn main() {
    dotenv().ok();
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("You need to pass at least one argument.");
        std::process::exit(1);
    }
    if args[1] == "new-legacy" || args[1] == "nl" {
        // deprecated
        new_legacy(args);
    } else if args[1] == "new" || args[1] == "n" {
        new();
    } else if args[1] == "work" || args[1] == "w" {
        work();
    } else if args[1] == "stats" || args[1] == "s" {
        new_work_stats(args);
    } else if args[1] == "start-work" || args[1] == "sw" {
        // deprecated
        start_work(args);
    } else if args[1] == "end-work" || args[1] == "ew" {
        // deprecated
        end_work();
    } else if args[1] == "work-stats" || args[1] == "ws" {
        // deprecated
        work_stats(args);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
}

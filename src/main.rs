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
        print_help();
        std::process::exit(1);
    } else if args[1] == "--version" || args[1] == "-V" {
        println!("nost {}", env!("CARGO_PKG_VERSION"));
    } else if args[1] == "--help" || args[1] == "-h" || args[1] == "help" {
        print_help();
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
        print_help();
        std::process::exit(1);
    }
}

fn print_help() {
    println!(
        "nost {} - a markdown-based NOT note generator\n\
\n\
Usage: nost <COMMAND> [ARGS]\n\
\n\
Commands:\n\
  new, n [YYYY-MM-DD]   Create (or open) a note for today or the given date\n\
  work, w [YYYY-MM-DD]  Record a work session\n\
  stats, s             Show statistics\n\
\n\
Options:\n\
  -h, --help           Print this help\n\
  -V, --version        Print version\n\
\n\
Configuration:\n\
  Reads `not_path` and `language` from config.toml (see config.toml.dist),\n\
  or the NOT_PATH environment variable.",
        env!("CARGO_PKG_VERSION")
    );
}

mod annotations;
mod commands;
mod configurations;
mod dates;
mod events;
mod files;
mod plugins;
mod projects;
use crate::commands::new::{new, new_legacy};
use crate::commands::work::work;
use crate::plugins::gdarquie_work::plugin::WorkPlugin;
use crate::plugins::plugin::PluginRegistry;
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

    // Plugins are registered here — the only place the core names them. To drop
    // or extract a feature area, add/remove one line; routing below is generic.
    let registry = PluginRegistry::new().register(Box::new(WorkPlugin));

    let command = args[1].as_str();

    // Give plugins first chance to claim the command.
    if let Some(result) = registry.dispatch(command, &args) {
        if let Err(e) = result {
            eprintln!("{}", e);
            std::process::exit(1);
        }
        return;
    }
    // Core built-in commands.
    if command == "new" || command == "n" {
        new_legacy(args);
    } else if command == "new-default" || command == "nn" {
        // wip
        new();
        println!("Creating new default note...");
    } else if command == "work" || command == "w" {
        work();
    } else if command == "new-start-work" {
        // implement new start work
    } else {
        eprintln!("Unknown command: \"{}\"", command);
        std::process::exit(1);
    }
}

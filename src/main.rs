use dotenv::dotenv;
use std::env;
use std::path::PathBuf;
mod annotation;
mod not;
mod work;
use crate::not::append;
use crate::not::get_or_create_not;
use crate::not::NotEvent;

// Validate a string as year-month in format YYYY-MM (01..12)
fn is_valid_year_month(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[4] != b'-' {
        return false;
    }
    let year = &s[0..4];
    let month = &s[5..7];
    if !year.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !month.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(month.parse::<u32>(), Ok(m) if (1..=12).contains(&m))
}

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

    if args[1] == "new" || args[1] == "n" {
        if args.len() > 2 {
            println!("Creating not with title: {}", args[1]);
            not::create_not(Some(args[2].clone())).unwrap();
        } else {
            not::create_not(None).unwrap();
        }
        std::process::exit(0);
    } else if args[1] == "start-work" || args[1] == "sw" {
        let not_path = get_or_create_not(None).unwrap();
        let default_workday;
        let workday = if args.len() > 2 {
            if chrono::NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").is_err() {
                eprintln!("Invalid date format. Please use YYYY-MM-DD.");
                std::process::exit(1);
            }
            Some(args[2].as_str())
        } else {
            println!("No date provided, using today's date.");
            default_workday = chrono::Local::now().format("%Y-%m-%d").to_string();
            Some(default_workday.as_str())
        };
        annotation::annotate(None, NotEvent::StartWork, None, &not_path, workday);
        std::process::exit(0);
    } else if args[1] == "end-work" || args[1] == "ew" {
        let not_path = get_or_create_not(None).unwrap();
        let default_workday;
        let workday = if args.len() > 2 {
            if chrono::NaiveDate::parse_from_str(&args[2], "%Y-%m-%d").is_err() {
                eprintln!("Invalid date format. Please use YYYY-MM-DD.");
                std::process::exit(1);
            }
            Some(args[2].as_str())
        } else {
            println!("No date provided, using today's date.");
            default_workday = chrono::Local::now().format("%Y-%m-%d").to_string();
            Some(default_workday.as_str())
        };
        annotation::annotate(None, NotEvent::StopWork, None, &not_path, workday);
        std::process::exit(0);
    } else if args[1] == "work-stats" || args[1] == "ws" {
        // Optional first arg is month in format YYYY-MM
        let month = if args.len() > 2 {
            let m = args[2].as_str();
            if !is_valid_year_month(m) {
                eprintln!("Invalid month format. Please use YYYY-MM.");
                std::process::exit(1);
            }
            Some(m)
        } else {
            None
        };
        let stats = match work::compute_work_stats(month) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("💥 Cannot compute stats for the current month:\"{}\".", e);
                eprintln!("Is there an existing note for this month?");
                std::process::exit(1);
            }
        };

        // Optional second flag to append stats in current note: true/1/yes/y
        let in_not = if args.len() > 3 {
            matches!(args[3].to_lowercase().as_str(), "true" | "1" | "yes" | "y")
        } else {
            false
        };

        let stats_content = work::compose_work_stats(stats);

        if in_not {
            let file_path = get_or_create_not(None).unwrap();
            let _ = append(PathBuf::from(file_path), &stats_content);
            println!("Stats appended to the current not.");
        } else {
            println!("{}", stats_content);
        }
        std::process::exit(0);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
}

use dotenv::dotenv;
use std::env;

mod not;
mod work;
use crate::not::get_now_as_string;
use crate::not::get_or_create_not;

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
        let annotation = format!(
            "not: {{start-work: '{}', salary: '{}', salary-currency: '{}'}}",
            get_now_as_string(),
            work::get_salary(),
            work::get_salary_currency()
        );
        not::annotate(&annotation, &not_path);
        std::process::exit(0);
    } else if args[1] == "stop-work" {
        let not_path = get_or_create_not(None).unwrap();
        let annotation = format!("not: {{stop-work: '{}'}}", get_now_as_string());
        not::annotate(&annotation, &not_path);
        std::process::exit(0);
    } else if args[1] == "work-stats" {
        // todo: add logic to compute work stats
        // return stats for the current month
        // consult all annotations for the month
        // return an array of stats
        // and append it to the current not
        let stats = work::compute_work_stats();
        // not::annotate(&stats.unwrap(), &not_path);
        std::process::exit(0);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
}

use crate::{
    annotations::annotate::annotate, events::models::EventName, files::create::create_file,
};

pub fn start_work(args: Vec<String>) {
    let not_path = create_file(None).unwrap();
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
    annotate(None, EventName::StartWork, None, &not_path, workday);
    std::process::exit(0);
}

use crate::{
    annotations::annotate::annotate, dates::parse::parse_iso_date, events::models::EventName,
    files::create::create_file,
};

pub fn start_work(args: Vec<String>) {
    let not_path = create_file(None).unwrap();
    let default_workday;
    let workday = if args.len() > 2 {
        if let Err(msg) = parse_iso_date(&args[2]) {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Some(args[2].as_str())
    } else {
        println!("No date provided, using today's date.");
        default_workday = chrono::Local::now().format("%Y-%m-%d").to_string();
        Some(default_workday.as_str())
    };
    annotate(None, EventName::StartWork, None, &not_path, workday);
}

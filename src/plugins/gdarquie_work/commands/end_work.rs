use crate::{
    annotations::{annotate::annotate, extract::extract_annotations_from_path},
    events::models::NotEvent,
    files::find::{find_last_not, get_or_create_not},
};

pub fn end_work(args: Vec<String>) {
    // todo: move that in dedicated function
    // we check if there is an active work session in the last not
    let last_not_path = find_last_not();
    if !last_not_path.is_none() {
        let annotations_from_last_not =
            extract_annotations_from_path(last_not_path.clone().unwrap());

        if let Ok(annotations) = annotations_from_last_not {
            match annotations.last() {
                Some(last) => {
                    // check if last is a start work annotation
                    if last.event == NotEvent::StartWork {
                        println!("There is an active work session in the last not.");
                        let workday = last.workday.as_deref();
                        // todo: end the work session
                        annotate(
                            None,
                            NotEvent::StopWork,
                            None,
                            last_not_path.clone().unwrap().to_str().unwrap(),
                            workday,
                        );
                    }
                }
                None => {}
            }
        }
    }

    // check if there is an active work session in the last not
    // open last not path, get the file and get annotations

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
    annotate(None, NotEvent::StopWork, None, &not_path, workday);
    std::process::exit(0);
}

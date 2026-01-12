use crate::{
    annotations::annotate::annotate, events::models::NotEvent, files::find::get_or_create_not,
    plugins::gdarquie_work::work_annotations::find::find_last_work_annotation,
};

pub fn end_work(args: Vec<String>) {
    // we check if there is an active work session in the last not
    let last_work_annotation = find_last_work_annotation();
    let not_path = get_or_create_not(None).unwrap();

    // we first check if there is a previous work annotation
    let workday_string = if let Some((annotation, path)) = last_work_annotation {
        // there is a START_WORK annotation : it is an active work session
        if annotation.event == NotEvent::StartWork {
            println!("There is an active work session in the last not.");

            let workday = annotation.workday.clone();
            let workday_str = workday.clone().unwrap_or_default();

            let previous_datetime_string = format!(
                "{}T23:59:59.999999999{}",
                workday_str,
                chrono::Local::now().format("%:z")
            );

            // STOP_WORK for the previous workday
            annotate(
                Some(&previous_datetime_string),
                NotEvent::StopWork,
                None,
                path.to_str().unwrap(),
                workday.as_deref(),
            );

            // START_WORK for today at 00:00
            let today_workday_str = chrono::Local::now().format("%Y-%m-%d").to_string();
            let today_datetime_string = format!(
                "{}T00:00:00.000000000{}",
                today_workday_str,
                chrono::Local::now().format("%:z")
            );

            annotate(
                Some(&today_datetime_string),
                NotEvent::StartWork,
                None,
                &not_path,
                Some(&today_workday_str),
            );

            workday
        } else {
            // Annotation exists but not START_WORK: no active session
            if args.len() > 2 {
                Some(args[2].clone())
            } else {
                Some(chrono::Local::now().format("%Y-%m-%d").to_string())
            }
        }
    } else {
        // No annotation found: no active session
        if args.len() > 2 {
            Some(args[2].clone())
        } else {
            println!("No date provided, using today's date.");
            Some(chrono::Local::now().format("%Y-%m-%d").to_string())
        }
    };

    let workday = workday_string.as_deref();
    annotate(None, NotEvent::StopWork, None, &not_path, workday);
    std::process::exit(0);
}

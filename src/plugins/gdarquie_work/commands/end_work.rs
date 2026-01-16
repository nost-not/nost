// use std::path::Path;

use crate::{
    annotations::annotate::annotate, events::models::NotEvent, files::create::create_note,
    plugins::gdarquie_work::work_annotations::find::find_last_work_annotation,
};

pub fn end_work() {
    // we check if there is an active work session in the last not
    let last_work_annotation = find_last_work_annotation();
    let not_path = create_note(None).unwrap();

    // we first check if there is a previous work annotation
    let workday_string = if let Some(work_annotation_with_path) = last_work_annotation {
        let annotation = work_annotation_with_path.annotation;
        let path = work_annotation_with_path.path;
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
            println!("No active session, using today's date.");
            Some(chrono::Local::now().format("%Y-%m-%d").to_string())
        }
    } else {
        println!("No active session, using today's date.");
        Some(chrono::Local::now().format("%Y-%m-%d").to_string())
    };

    let workday = workday_string.as_deref();
    annotate(None, NotEvent::StopWork, None, &not_path, workday);
    std::process::exit(0);
}

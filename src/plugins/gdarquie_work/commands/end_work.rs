use std::path::Path;

use crate::{
    annotations::annotate::annotate,
    events::models::EventName,
    files::create::create_file,
    plugins::gdarquie_work::work_annotations::{
        find::find_last_work_annotation, models::WorkAnnotationWithPath,
    },
};

fn add_stop_work_annotations(last_annotation: &WorkAnnotationWithPath, path: &Path) {
    // find the workday of the last annotation
    let today = chrono::Local::now()
        .date_naive()
        .format("%Y-%m-%d")
        .to_string();

    // if the last annoation is not in correct format
    if last_annotation.annotation.workday.is_none() {
        println!("The last annotation does not have a workday. Repair the annotation before adding a STOP_WORK annotation. The annotation is: {:?}.", last_annotation.annotation);
        return;
    }

    // if this is not today
    if last_annotation.annotation.workday.as_deref() != Some(&today) {
        let yesterday = (chrono::Local::now() - chrono::Duration::days(1))
            .date_naive()
            .format("%Y-%m-%d")
            .to_string();

        // if it is more than one day, we display a warning message with the last start work annotation and the date and path, etc.: not supposed to work more than one full day without stopping work, we skip it
        if last_annotation.annotation.workday.as_deref() < Some(&yesterday) {
            println!("The last annotation is from a previous day ({}), more than one day ago. No STOP_WORK annotation has been added. The annotation is: {:?}.", yesterday, last_annotation.annotation);
            return;
        }

        // if it is yesterday, we add a STOP_WORK annotation for yesterday at 23:59:59 and a START_WORK annotation for today at 00:00:00
        if last_annotation.annotation.workday.as_deref() == Some(&yesterday) {
            let yesterday_datetime_string = format!(
                "{}T23:59:59.999999999{}",
                yesterday,
                chrono::Local::now().format("%:z")
            );

            // STOP_WORK for yesterday
            annotate(
                Some(&yesterday_datetime_string),
                EventName::StopWork,
                None,
                last_annotation.path.to_str().unwrap(),
                last_annotation.annotation.workday.as_deref(),
            );

            let today_datetime_string = format!(
                "{}T00:00:00.000000000{}",
                today,
                chrono::Local::now().format("%:z")
            );

            // START_WORK for today
            annotate(
                Some(&today_datetime_string),
                EventName::StartWork,
                None,
                path.to_str().unwrap(),
                last_annotation.annotation.workday.as_deref(),
            );
        }
    }

    // we add a STOP_WORK annotation for today
    annotate(
        None,
        EventName::StopWork,
        None,
        path.to_str().unwrap(),
        last_annotation.annotation.workday.as_deref(),
    );
}

pub fn has_active_session(last_work_annotation: &WorkAnnotationWithPath) -> bool {
    if last_work_annotation.annotation.event == EventName::StartWork {
        return true;
    }

    false
}

pub fn end_work() {
    let last_work_annotation = find_last_work_annotation().unwrap();
    let path = Path::new(&create_file(None).unwrap()).to_path_buf();

    // find last active session
    if !has_active_session(&last_work_annotation) {
        // return a message, we have nothin to do, there is no active session
        println!("No working active session has been found. No annotation has been added.");
        std::process::exit(0);
    }

    add_stop_work_annotations(&last_work_annotation, &path);

    std::process::exit(0);
}

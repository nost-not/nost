use std::path::PathBuf;

use crate::annotations::extract::extract_annotations_from_path;
use crate::annotations::filter::filter_annotation_by_events;
use crate::annotations::models::Annotation;
use crate::configurations::get::get_value_from_config;
use crate::events::models::NotEvent;
use crate::files::find::find_all_not_files;

pub fn find_last_work_annotation() -> Option<(Annotation, PathBuf)> {
    let not_path = match get_value_from_config("not_path") {
        Ok(path) => path,
        Err(_) => return None,
    };

    // Get all not files sorted
    let mut files = match find_all_not_files(not_path.into()) {
        Ok(file) => file,
        Err(_) => return None,
    };

    // Process files in reverse (last first) to minimize iterations
    while let Some(path) = files.pop() {
        // Extract annotations from this file
        if let Ok(annotations) = extract_annotations_from_path(path.clone()) {
            // Check if there are any work-related annotations
            let mut work_annotations = filter_annotation_by_events(
                annotations,
                vec![NotEvent::StartWork, NotEvent::StopWork],
            );

            if !work_annotations.is_empty() {
                // Sort by datetime to get the most recent annotation
                work_annotations.sort_by_key(|a| a.datetime);
                return Some((work_annotations.last().cloned().unwrap(), path));
            }
        }
    }

    None
}

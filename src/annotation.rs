use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::NotEvent;
use chrono::DateTime;
use chrono::Local;
use regex::Regex;
use std::path::PathBuf;
use uuid::Uuid;

pub struct Annotation {
    pub uid: Uuid,
    pub event: NotEvent,
    pub datetime: DateTime<Local>,
}

pub fn convert_into_annotation(annotation_in_text: &String) -> Annotation {
    // todo: change by extracting data from string

    // extract uid
    // let re = Regex::new(r#"\[//\]: # "not:(\{.*\})""#).unwrap();
    // let re_uid = Regex::new(r#"uuid"#).unwrap();
    // re.captures(&annotation)
    //     .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()));

    let extract_uid_regex = regex::Regex::new(
        r#"\[//\]: # "\{date: '.*?', event: '.*?', uuid: '(?P<uuid>[a-f0-9\-]+)'\}""#,
    )
    .unwrap();

    let mut uid = "";

    if let Some(caps) = extract_uid_regex.captures(annotation_in_text) {
        if let Some(uid_str) = caps.name("uuid") {
            if let Ok(parsed_uid) = Uuid::parse_str(uid_str.as_str()) {
                uid = parsed_uid;
            }
        }
    }

    // extract event
    let event = NotEvent::CreateNot;

    // extract datetime
    let datetime = Local::now();

    Annotation {
        uid,
        event,
        datetime,
    }
}

pub fn extract_annotations_from_path(path: PathBuf) -> Vec<Annotation> {
    // get the pathes of all the notes of the currents monthes
    let current_month_pathes = get_not_pathes(path).unwrap();

    // get all annotations of the month
    let mut raw_annotations = Vec::new();
    for path in current_month_pathes {
        let annotations_for_current_file = extract_annotations_from_one_file(&path).unwrap();
        raw_annotations.extend(annotations_for_current_file);
    }

    // refine the annotation removing the not prefix
    let re = Regex::new(r#"\[//\]: # "not:(\{.*\})""#).unwrap();

    let filtered_annotations: Vec<String> = raw_annotations
        .into_iter()
        .filter_map(|annotation| {
            re.captures(&annotation)
                .and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
        })
        .collect();

    let mut annotations = Vec::new();

    filtered_annotations.into_iter().for_each(|annotation| {
        println!("{}", annotation);
        annotations.push(Annotation {
            uid: uuid::Uuid::new_v4(),  // todo: change with value from annotation
            event: NotEvent::CreateNot, // todo: change with value from annotation
            datetime: Local::now(),     // todo: change with value from annotation
        })
    });

    annotations
}

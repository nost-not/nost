use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::NotEvent;
use chrono::DateTime;
use chrono::Local;
use regex::Regex;
use std::path;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Annotation {
    pub uid: Uuid,
    pub event: NotEvent,
    pub datetime: DateTime<Local>,
}

pub fn filter_annotation_by_events(
    annotations: Vec<Annotation>,
    event: Vec<NotEvent>,
) -> Vec<Annotation> {
    annotations
        .into_iter()
        .filter(|annotation| event.contains(&annotation.event))
        .collect()
}

pub fn extract_field_from_annotation(annotation: &String, field: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"{}: '(?P<value>[^']+)'"#, field)).unwrap();
    if let Some(caps) = re.captures(annotation) {
        if let Some(value) = caps.name("value") {
            return Some(value.as_str().to_string());
        }
    }
    None
}

pub fn convert_into_annotation(annotation_in_text: &String) -> Result<Annotation, &str> {
    // extract uid
    let mut uid = extract_field_from_annotation(annotation_in_text, "uid")
        .and_then(|uid_str| Uuid::parse_str(&uid_str).ok())
        .ok_or("Missing or invalid uid")?;

    // extract event
    let event = extract_field_from_annotation(annotation_in_text, "event")
        .and_then(|event_str| NotEvent::from_str(&event_str).ok())
        .ok_or("Missing or invalid event")?;

    // extract datetime
    let datetime = extract_field_from_annotation(annotation_in_text, "date")
        .and_then(|datetime_str| DateTime::parse_from_rfc3339(&datetime_str).ok())
        .map(|dt| dt.with_timezone(&Local))
        .ok_or("Missing or invalid date")?;

    Ok(Annotation {
        uid,
        event,
        datetime,
    })
}

pub fn extract_annotations_from_path(path: PathBuf) -> Vec<Annotation> {
    // get all the pathes of the notes of parent path
    let pathes = get_not_pathes(path).unwrap();

    // get all annotations of the month
    let mut raw_annotations = Vec::new();
    for path in pathes {
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

    // convert text annotations into Annotation structs
    let mut annotations = Vec::new();

    filtered_annotations.into_iter().for_each(|annotation| {
        println!("{}", annotation);
        match convert_into_annotation(&annotation) {
            Ok(annotation_struct) => annotations.push(annotation_struct),
            Err(e) => eprintln!("Failed to parse annotation: {}", e),
        }
    });

    println!("Extracted {:?} annotations", &annotations.len());

    annotations
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_uid_from_annotation() {
        let annotation = "not:{uid: 'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff', created_at: '2025-08-10 00:51:45 +09:00',  event: 'START_WORK'}".to_string();
        let uid = super::extract_field_from_annotation(&annotation, "uid");
        assert_eq!(
            uid.unwrap().to_string(),
            "b86bc6ed-50a5-4ef2-bdd3-e17baef11eff"
        );
    }

    #[test]
    fn extract_datetime_from_annotation() {
        let annotation = "not:{uid: 'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff', created_at: '2025-08-10 00:51:45 +09:00',  event: 'START_WORK'}".to_string();
        let datetime = super::extract_field_from_annotation(&annotation, "created_at");
        assert_eq!(datetime.unwrap().to_string(), "2025-08-10 00:51:45 +09:00");
    }

    #[test]
    fn extract_event_from_annotation() {
        let annotation = "not:{uid: 'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff', created_at: '2025-08-10 00:51:45 +09:00',  event: 'START_WORK'}".to_string();
        let event = super::extract_field_from_annotation(&annotation, "event");
        assert_eq!(event.unwrap().to_string(), "START_WORK");
    }
}

use crate::not::append;
use crate::not::extract_annotations_from_one_file;
use crate::not::get_not_pathes;
use crate::not::get_now_as_string;
use crate::NotEvent;
use chrono::DateTime;
use chrono::Local;
use regex::Regex;
use std::path::PathBuf;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct Annotation {
    pub _uid: Uuid,
    pub event: NotEvent,
    pub datetime: DateTime<Local>,
}

pub fn annotate(
    option_date: Option<&str>,
    event: NotEvent,
    input_uid: Option<&Uuid>,
    not_path: &str,
) {
    let now = get_now_as_string();
    let date = match option_date {
        Some(d) => d,
        None => &now,
    };

    let new_uid = Uuid::new_v4().to_string();
    let uid = match input_uid {
        Some(u) => u.to_string(),
        None => new_uid,
    };

    let content = format!(
        "\"not:{{date:'{}',event:'{}',uid:'{}'}}\"",
        date, event, uid
    );
    let annotation = format!("[//]: # {}\n", content);
    append(not_path.into(), &annotation).expect("🛑 Failed to annotate.");
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

pub fn extract_field_from_annotation(annotation: &str, field: &str) -> Option<String> {
    let re = Regex::new(&format!(r#"{}:'(?P<value>[^']+)'"#, field)).unwrap();
    if let Some(caps) = re.captures(annotation) {
        if let Some(value) = caps.name("value") {
            return Some(value.as_str().to_string());
        }
    }
    None
}

pub fn convert_into_annotation(annotation_in_text: &str) -> Result<Annotation, &str> {
    // extract datetime
    let datetime = extract_field_from_annotation(annotation_in_text, "date")
        .and_then(|datetime_str| DateTime::parse_from_rfc3339(&datetime_str).ok())
        .map(|dt| dt.with_timezone(&Local))
        .ok_or("Missing or invalid date")?;

    // extract event
    let event = extract_field_from_annotation(annotation_in_text, "event")
        .and_then(|event_str| NotEvent::from_str(&event_str).ok())
        .ok_or("Missing or invalid event")?;

    // extract uid
    let uid = extract_field_from_annotation(annotation_in_text, "uid")
        .and_then(|uid_str| Uuid::parse_str(&uid_str).ok())
        .ok_or("Missing or invalid uid")?;

    Ok(Annotation {
        _uid: uid,
        event,
        datetime,
    })
}

pub fn extract_annotations_from_path(path: PathBuf) -> Result<Vec<Annotation>, std::io::Error> {
    // get all the pathes of the notes of parent path
    // let pathes = get_not_pathes(path).unwrap();

    let pathes = match get_not_pathes(path) {
        Ok(p) => p,
        Err(e) => {
            return Err(e);
        }
    };

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
        match convert_into_annotation(&annotation) {
            Ok(annotation_struct) => annotations.push(annotation_struct),
            Err(e) => eprintln!(
                "Failed to parse annotation: {}\n Annotation: {}\n",
                e, annotation
            ),
        }
    });

    Ok(annotations)
}

#[cfg(test)]
mod tests {

    #[test]
    #[serial_test::serial]
    fn test_append() {
        use std::fs;
        use std::io::Read;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_append.txt");

        // Create the file first
        fs::File::create(&file_path).unwrap();

        // Call append
        let content = "Test content generated from test_append test!";
        super::append(file_path.clone(), content).expect("Failed to append");

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        assert!(file_content.contains(content));
    }

    #[test]
    #[serial_test::serial]
    fn test_annotate() {
        use std::fs;
        use std::io::Read;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_annotate.txt");

        // Create the file first
        fs::File::create(&file_path).unwrap();

        // Call annotate
        super::annotate(
            None,
            crate::not::NotEvent::CreateNot,
            None,
            file_path.to_str().unwrap(),
        );

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        // The annotation should be wrapped as [//]: # "..."
        let annotation_regex =
            regex::Regex::new(r#"\[//\]: # "not:\{date:'.*',event:'CREATE_NOT',uid:'.*'\}""#)
                .unwrap();
        assert!(
            file_content
                .lines()
                .any(|line| annotation_regex.is_match(line)),
            "Annotation with expected format not found in file content"
        );
    }

    #[test]
    #[serial_test::serial]
    fn extract_uid_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let uid = super::extract_field_from_annotation(&annotation, "uid");
        assert_eq!(
            uid.unwrap().to_string(),
            "b86bc6ed-50a5-4ef2-bdd3-e17baef11eff"
        );
    }

    #[test]
    #[serial_test::serial]
    fn extract_datetime_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let datetime = super::extract_field_from_annotation(&annotation, "created_at");
        assert_eq!(
            datetime.unwrap().to_string(),
            "2025-09-29T00:00:43.245684903+02:00"
        );
    }

    #[test]
    #[serial_test::serial]
    fn extract_event_from_annotation() {
        let annotation = "not:{uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff',created_at:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK'}".to_string();
        let event = super::extract_field_from_annotation(&annotation, "event");
        assert_eq!(event.unwrap().to_string(), "START_WORK");
    }
}

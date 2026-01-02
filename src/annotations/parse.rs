use chrono::DateTime;
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    annotations::{extract::extract_field_from_annotation, models::Annotation},
    events::models::NotEvent,
};

pub fn parse_annotation(annotation_in_text: &str) -> Result<Annotation, &str> {
    // extract datetime
    let datetime = extract_field_from_annotation(annotation_in_text, "date")
        .and_then(|datetime_str| DateTime::parse_from_rfc3339(&datetime_str).ok())
        .ok_or("Missing or invalid date")?;

    // extract event
    let event = extract_field_from_annotation(annotation_in_text, "event")
        .and_then(|event_str| NotEvent::from_str(&event_str).ok())
        .ok_or("Missing or invalid event")?;

    // extract uid
    let uid = extract_field_from_annotation(annotation_in_text, "uid")
        .and_then(|uid_str| Uuid::parse_str(&uid_str).ok())
        .ok_or("Missing or invalid uid")?;

    let workday = extract_field_from_annotation(annotation_in_text, "workday");

    Ok(Annotation {
        _uid: uid,
        event,
        datetime,
        workday,
    })
}

#[cfg(test)]
mod tests {
    use crate::annotations::parse::parse_annotation;

    // todo: move this test where append function is defined
    #[test]
    #[serial_test::serial]
    fn test_append() {
        use crate::append;
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
        append(file_path.clone(), content).expect("Failed to append");

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        assert!(file_content.contains(content));
    }

    #[test]
    #[serial_test::serial]
    fn test_parse_annotation() {
        let raw_annotation = "not:{date:'2025-09-29T00:00:43.245684903+02:00',event:'START_WORK',uid:'b86bc6ed-50a5-4ef2-bdd3-e17baef11eff'}";
        let annotation = parse_annotation(raw_annotation).unwrap();
        assert_eq!(
            annotation.datetime.to_rfc3339(),
            "2025-09-29T00:00:43.245684903+02:00"
        );
        assert_eq!(annotation.event, crate::events::models::NotEvent::StartWork);
        assert_eq!(
            annotation._uid.to_string(),
            "b86bc6ed-50a5-4ef2-bdd3-e17baef11eff"
        );
    }
}

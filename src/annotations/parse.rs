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

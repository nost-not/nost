use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventName {
    StartWork,
    StopWork,
    CreateNot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub datetime: String,
    pub event: String,
    pub day: String,
    pub not_type: String,
    pub uid: String,
}

impl Event {
    pub fn new(event_name: EventName, not_type: String, string_date: String) -> Self {
        let now = Local::now();
        Self {
            datetime: now.to_rfc3339(),
            event: format!("{}", event_name),
            day: string_date,
            not_type,
            uid: Uuid::new_v4().to_string(),
        }
    }
}

impl fmt::Display for EventName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventName::StartWork => write!(f, "START_WORK"),
            EventName::StopWork => write!(f, "STOP_WORK"),
            EventName::CreateNot => write!(f, "CREATE_NOT"),
        }
    }
}

impl std::str::FromStr for EventName {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "START_WORK" => Ok(EventName::StartWork),
            "STOP_WORK" => Ok(EventName::StopWork),
            "CREATE_NOT" => Ok(EventName::CreateNot),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_event_name_display_and_from_str_roundtrip() {
        let names = [
            EventName::StartWork,
            EventName::StopWork,
            EventName::CreateNot,
        ];

        for name in names {
            let as_string = name.to_string();
            let parsed: EventName = as_string.parse().expect("EventName should parse");
            assert_eq!(parsed, name);
        }
    }

    #[test]
    fn test_event_name_from_str_invalid_value() {
        let parsed: Result<EventName, _> = "UNKNOWN_EVENT".parse();
        assert!(parsed.is_err());
    }

    #[test]
    fn test_event_new_sets_expected_fields() {
        let event = Event::new(
            EventName::StartWork,
            "work".to_string(),
            "2026-08-13".to_string(),
        );

        assert_eq!(event.event, EventName::StartWork.to_string());
        assert_eq!(event.not_type, "work");
        assert_eq!(event.day, "2026-08-13");
        assert!(DateTime::parse_from_rfc3339(&event.datetime).is_ok());
        assert!(Uuid::parse_str(&event.uid).is_ok());
    }
}

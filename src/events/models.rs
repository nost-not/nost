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
    pub fn now(event_name: EventName, not_type: String) -> Self {
        let now = Local::now();
        Self {
            datetime: now.to_rfc3339(),
            event: format!("{}", event_name),
            day: now.format("%Y-%m-%d").to_string(),
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

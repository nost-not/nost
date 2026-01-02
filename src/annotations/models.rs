use crate::events::models::NotEvent;
use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

#[derive(Debug)]
pub struct Annotation {
    pub _uid: Uuid,
    pub event: NotEvent,
    pub datetime: DateTime<FixedOffset>,
    pub workday: Option<String>,
}

use crate::events::models::EventName;
use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Annotation {
    pub _uid: Uuid,
    pub event: EventName,
    pub datetime: DateTime<FixedOffset>,
    // todo: remove from core, should be plugin specific
    pub workday: Option<String>,
}

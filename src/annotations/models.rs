use crate::events::models::NotEvent;
use chrono::{DateTime, FixedOffset};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Annotation {
    pub _uid: Uuid,
    pub event: NotEvent,
    pub datetime: DateTime<FixedOffset>,
    // todo: remove from core, should be plugin specific
    pub workday: Option<String>,
}

use chrono::{DateTime, FixedOffset};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Stats {
    pub day: String, // in format "YYYY-MM-DD"
    pub length_in_minutes: i32,
}

#[derive(Debug, Clone)]
pub struct WeekStats {
    pub total_duration_in_minutes: i32,
    pub work_stats: Vec<Stats>,
}

#[derive(Debug, Clone)]
pub struct MonthStats {
    pub total_duration_in_minutes: i32,
    pub total_work_days: i32,
    pub work_stats_by_week: HashMap<WeekId, WeekStats>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeekId {
    pub year: i32,
    pub week: u32,
}

/// A work event that has already been parsed and validated.
/// Can only be constructed via `WorkEvent::try_from_raw`, which ensures
/// the datetime is valid RFC3339 and the event kind is StartWork or StopWork.
/// Downstream code never needs to handle parse errors.
#[derive(Debug, Clone)]
pub struct WorkEvent {
    pub datetime: DateTime<FixedOffset>,
    pub kind: WorkEventKind,
    pub day: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkEventKind {
    Start,
    Stop,
}

impl WorkEvent {
    /// Parse and validate a raw event. Returns `None` if the datetime is
    /// invalid or the event is not a work event (START_WORK / STOP_WORK).
    /// All validation happens here — callers receive a guaranteed-valid value.
    pub fn try_from_raw(datetime: &str, event: &str, day: &str) -> Option<Self> {
        let datetime = DateTime::parse_from_rfc3339(datetime).ok()?;
        let kind = match event {
            "START_WORK" => WorkEventKind::Start,
            "STOP_WORK" => WorkEventKind::Stop,
            _ => return None,
        };
        Some(WorkEvent {
            datetime,
            kind,
            day: day.to_string(),
        })
    }
}

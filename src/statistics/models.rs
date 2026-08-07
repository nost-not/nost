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

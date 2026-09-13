use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub day: String,
    pub start: String,
    pub stop: Option<String>,
}

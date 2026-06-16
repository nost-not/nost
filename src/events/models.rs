use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventName {
    StartWork,
    StopWork,
    CreateNot,
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

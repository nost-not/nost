use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotEvent {
    // todo: move in work event enum
    StartWork,
    StopWork,
    // keep in core
    CreateNot,
}

impl fmt::Display for NotEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotEvent::StartWork => write!(f, "START_WORK"),
            NotEvent::StopWork => write!(f, "STOP_WORK"),
            NotEvent::CreateNot => write!(f, "CREATE_NOT"),
        }
    }
}

impl std::str::FromStr for NotEvent {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "START_WORK" => Ok(NotEvent::StartWork),
            "STOP_WORK" => Ok(NotEvent::StopWork),
            "CREATE_NOT" => Ok(NotEvent::CreateNot),
            _ => Err(()),
        }
    }
}

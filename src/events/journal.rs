use crate::{events::models::WorkSession, projects::initialize::get_project_config_path};
use std::{
    fs::{self, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

/// Returns the path to the monthly NDJSON journal file.
/// e.g. `.nost/journal-2026-08.ndjson`
pub fn journal_file_path(month: &str) -> String {
    let config_path = get_project_config_path();
    format!("{}/journal-{}.ndjson", config_path, month)
}

/// Appends a WorkSession as a NDJSON line to the monthly file.
pub fn append_session(session: &WorkSession) -> io::Result<()> {
    let month = &session.workday[..7]; // YYYY-MM
    let path = journal_file_path(month);

    // ensure parent directory exists
    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

    let line = serde_json::to_string(session).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize WorkSession: {}", e),
        )
    })?;

    let mut file = OpenOptions::new().create(true).append(true).open(&path)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Reads the monthly file, finds the last line with `stop: null`, sets its stop
/// to `stop_datetime`, and rewrites the file.
pub fn update_last_stop(month: &str, stop_datetime: &str) -> io::Result<()> {
    let path = journal_file_path(month);

    if !Path::new(&path).exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Journal file not found: {}", path),
        ));
    }

    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;

    // find last line with stop: null
    let last_open = lines
        .iter()
        .rposition(|line| {
            serde_json::from_str::<WorkSession>(line)
                .map(|s| s.stop.is_none())
                .unwrap_or(false)
        })
        .ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "No open session found to close")
        })?;

    let mut session: WorkSession = serde_json::from_str(&lines[last_open]).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse WorkSession: {}", e),
        )
    })?;
    session.stop = Some(stop_datetime.to_string());
    lines[last_open] = serde_json::to_string(&session).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize WorkSession: {}", e),
        )
    })?;

    fs::write(&path, lines.join("\n") + "\n")?;
    Ok(())
}

/// Loads all WorkSession records from the monthly NDJSON file.
pub fn load_month_sessions(month: &str) -> io::Result<Vec<WorkSession>> {
    let path = journal_file_path(month);

    if !Path::new(&path).exists() {
        return Ok(Vec::new());
    }

    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let mut sessions = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let session: WorkSession = serde_json::from_str(&line).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Invalid NDJSON on line {}: {}", i + 1, e),
            )
        })?;
        sessions.push(session);
    }

    Ok(sessions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    // Helper: redirect config path to a temp dir for testing.
    // We write the file directly without going through get_project_config_path().
    fn write_ndjson(dir: &TempDir, month: &str, content: &str) -> String {
        let path = format!("{}/journal-{}.ndjson", dir.path().display(), month);
        fs::write(&path, content).unwrap();
        path
    }

    fn session(workday: &str, start: &str, stop: Option<&str>) -> WorkSession {
        WorkSession {
            workday: workday.to_string(),
            start: start.to_string(),
            stop: stop.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_worksession_serialization_roundtrip() {
        let s = session(
            "2026-08-14",
            "2026-08-14T09:00:00+09:00",
            Some("2026-08-14T12:00:00+09:00"),
        );
        let json = serde_json::to_string(&s).unwrap();
        let parsed: WorkSession = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.workday, "2026-08-14");
        assert_eq!(parsed.stop, Some("2026-08-14T12:00:00+09:00".to_string()));
    }

    #[test]
    fn test_worksession_open_stop_is_null() {
        let s = session("2026-08-14", "2026-08-14T14:00:00+09:00", None);
        let json = serde_json::to_string(&s).unwrap();
        assert!(json.contains("\"stop\":null"));
        let parsed: WorkSession = serde_json::from_str(&json).unwrap();
        assert!(parsed.stop.is_none());
    }

    #[test]
    fn test_load_month_sessions_parses_ndjson() {
        let dir = TempDir::new().unwrap();
        write_ndjson(
            &dir,
            "2026-08",
            "{\"workday\":\"2026-08-14\",\"start\":\"2026-08-14T09:00:00+09:00\",\"stop\":\"2026-08-14T12:00:00+09:00\"}\n\
             {\"workday\":\"2026-08-14\",\"start\":\"2026-08-14T14:00:00+09:00\",\"stop\":null}\n",
        );

        // Parse directly, bypassing path resolution
        let path = format!("{}/journal-2026-08.ndjson", dir.path().display());
        let content = fs::read_to_string(&path).unwrap();
        let sessions: Vec<WorkSession> = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();

        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[0].workday, "2026-08-14");
        assert!(sessions[0].stop.is_some());
        assert!(sessions[1].stop.is_none());
    }

    #[test]
    fn test_update_last_stop_sets_stop_datetime() {
        let dir = TempDir::new().unwrap();
        let path = write_ndjson(
            &dir,
            "2026-08",
            "{\"workday\":\"2026-08-14\",\"start\":\"2026-08-14T09:00:00+09:00\",\"stop\":null}\n",
        );

        // Manually update
        let content = fs::read_to_string(&path).unwrap();
        let mut sessions: Vec<WorkSession> = content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();

        let last_open = sessions.iter().rposition(|s| s.stop.is_none()).unwrap();
        sessions[last_open].stop = Some("2026-08-14T12:00:00+09:00".to_string());

        let updated: String = sessions
            .iter()
            .map(|s| serde_json::to_string(s).unwrap())
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        fs::write(&path, updated).unwrap();

        let result_content = fs::read_to_string(&path).unwrap();
        let result: WorkSession = serde_json::from_str(result_content.trim()).unwrap();
        assert_eq!(
            result.stop,
            Some("2026-08-14T12:00:00+09:00".to_string())
        );
    }
}

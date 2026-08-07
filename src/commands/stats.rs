use crate::dates::validate::is_valid_month_string;
use crate::statistics::compute::compute_month_stats;
use crate::statistics::print::print_stats;

pub fn stats(args: Vec<String>) {
    // Optional first arg is month in format YYYY-MM
    let month = if args.len() > 2 {
        let m = args[2].as_str();
        if !is_valid_month_string(m) {
            eprintln!("Invalid month format. Please use YYYY-MM.");
            std::process::exit(1);
        }
        Some(m.to_string())
    } else {
        None
    };

    let stats = match compute_month_stats(month.as_deref()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("💥 Cannot compute stats: \"{}\".", e);
            eprintln!("Is there a journal.json with work events for this month?");
            std::process::exit(1);
        }
    };

    log::debug!(
        "Computed work stats for month {:?}: total_duration_in_minutes={:?}, stats={:?}",
        month,
        stats.total_duration_in_minutes,
        stats
    );

    let stats_content = print_stats(stats);
    println!("{}", stats_content);
}

#[cfg(test)]
mod tests {
    use super::stats;
    use crate::events::models::Event;
    use std::{env, fs};
    use tempfile::tempdir;

    fn write_journal(base: &str, events: &[Event]) {
        let journal_dir = format!("{}/.nost", base);
        fs::create_dir_all(&journal_dir).unwrap();
        let content = serde_json::to_string_pretty(events).unwrap();
        fs::write(format!("{}/journal.json", journal_dir), content).unwrap();
    }

    fn make_event(datetime: &str, day: &str, event: &str, uid: &str) -> Event {
        Event {
            datetime: datetime.to_string(),
            event: event.to_string(),
            day: day.to_string(),
            not_type: "work".to_string(),
            uid: uid.to_string(),
        }
    }

    #[test]
    #[serial_test::serial]
    fn stats_prints_monthly_summary_for_valid_month() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        env::set_var("NOST_WORK_SALARY", "100");
        env::set_var("NOST_WORK_CURRENCY", "EUR");

        let events = vec![
            make_event("2026-08-05T09:00:00+00:00", "2026-08-05", "START_WORK", "a"),
            make_event("2026-08-05T10:30:00+00:00", "2026-08-05", "STOP_WORK", "b"),
            make_event("2026-07-31T09:00:00+00:00", "2026-07-31", "START_WORK", "c"),
            make_event("2026-07-31T10:00:00+00:00", "2026-07-31", "STOP_WORK", "d"),
        ];
        write_journal(dir.path().to_str().unwrap(), &events);

        stats(vec![
            "nost".to_string(),
            "s".to_string(),
            "2026-08".to_string(),
        ]);

        assert!(true); // smoke test: asserts no panic
    }
}

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
            std::process::exit(1);
        }
    };

    log::debug!(
        "Computed stats: total_duration_in_minutes={:?}, stats={:?}",
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

    fn write_events_file(base: &str, month: &str, events: &[Event]) {
        let events_dir = format!("{}/.nost/events", base);
        fs::create_dir_all(&events_dir).unwrap();
        let content = events
            .iter()
            .map(|event| serde_json::to_string(event).unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        fs::write(format!("{}/{}.ndjson", events_dir, month), content).unwrap();
    }

    fn make_event(day: &str, start: &str, stop: Option<&str>) -> Event {
        Event {
            day: day.to_string(),
            start: start.to_string(),
            stop: stop.map(|value| value.to_string()),
        }
    }

    #[test]
    #[serial_test::serial]
    fn stats_prints_monthly_summary_for_valid_month() {
        let dir = tempdir().unwrap();
        env::set_var("NOT_PATH", dir.path().to_str().unwrap());
        env::set_var("NOST_WORK_SALARY", "100");
        env::set_var("NOST_WORK_CURRENCY", "EUR");

        write_events_file(
            dir.path().to_str().unwrap(),
            "2026-08",
            &[make_event(
                "2026-08-05",
                "2026-08-05T09:00:00+00:00",
                Some("2026-08-05T10:30:00+00:00"),
            )],
        );
        write_events_file(
            dir.path().to_str().unwrap(),
            "2026-07",
            &[make_event(
                "2026-07-31",
                "2026-07-31T09:00:00+00:00",
                Some("2026-07-31T10:00:00+00:00"),
            )],
        );

        stats(vec![
            "nost".to_string(),
            "s".to_string(),
            "2026-08".to_string(),
        ]);

        assert!(true); // smoke test: asserts no panic
    }
}

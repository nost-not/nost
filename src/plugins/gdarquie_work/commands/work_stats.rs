use crate::files::append::append;
use crate::files::find::get_or_create_not;
use crate::plugins::gdarquie_work::work;
use std::path::PathBuf;

pub fn work_stats(args: Vec<String>) {
    // Optional first arg is month in format YYYY-MM
    let month = if args.len() > 2 {
        let m = args[2].as_str();
        if !is_valid_year_month(m) {
            eprintln!("Invalid month format. Please use YYYY-MM.");
            std::process::exit(1);
        }
        Some(m)
    } else {
        None
    };
    let stats = match work::compute_monthly_work_stats(month) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("💥 Cannot compute stats for the current month:\"{}\".", e);
            eprintln!("Is there an existing note for this month?");
            std::process::exit(1);
        }
    };

    // Optional second flag to append stats in current note: true/1/yes/y
    let in_not = if args.len() > 3 {
        matches!(args[3].to_lowercase().as_str(), "true" | "1" | "yes" | "y")
    } else {
        false
    };

    let stats_content = work::compose_monthly_work_stats(stats);

    if in_not {
        let file_path = get_or_create_not(None).unwrap();
        let _ = append(PathBuf::from(file_path), &stats_content);
        println!("Stats appended to the current not.");
    } else {
        println!("{}", stats_content);
    }
    std::process::exit(0);
}

// Validate a string as year-month in format YYYY-MM (01..12)
fn is_valid_year_month(s: &str) -> bool {
    if s.len() != 7 {
        return false;
    }
    let bytes = s.as_bytes();
    if bytes[4] != b'-' {
        return false;
    }
    let year = &s[0..4];
    let month = &s[5..7];
    if !year.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    if !month.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    matches!(month.parse::<u32>(), Ok(m) if (1..=12).contains(&m))
}

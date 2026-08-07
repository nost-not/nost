use crate::statistics::models::{MonthStats, WeekId, WeekStats};
use chrono::Datelike;
use chrono::NaiveDate;
use std::env;

pub fn print_stats(stats: MonthStats) -> String {
    // would be better to return a Result<String, Error> but for now we just return a String
    let header =
        "\n| Day | Date       | Hours | Acc |\n|-----|------------|-------|-----|\n".to_string();
    let mut stats_content: String = String::new();

    // collect and sort weeks by date (year, then week)
    let mut sorted_weeks: Vec<(&WeekId, &WeekStats)> = stats.work_stats_by_week.iter().collect();
    sorted_weeks
        .sort_by(|(a_id, _), (b_id, _)| a_id.year.cmp(&b_id.year).then(a_id.week.cmp(&b_id.week)));

    // for each week in work_stats_by_week add an header and then the stats
    for (_week_id, week_stats) in sorted_weeks {
        // Add week header
        stats_content.push_str(&header);

        // sort the days by day (ascending)
        let mut sorted_work_stats = week_stats.work_stats.clone();
        sorted_work_stats.sort_by(|a, b| a.day.cmp(&b.day));

        let mut cumulative_week_hours = 0.0;
        for work_stat in sorted_work_stats.iter() {
            let date = NaiveDate::parse_from_str(&work_stat.day, "%Y-%m-%d").unwrap();
            let weekday = date.weekday();
            let hours = work_stat.length_in_minutes as f32 / 60.0;
            cumulative_week_hours += hours;

            stats_content.push_str(&format!(
                "| {} | {} | {:.2} | {:.2} |\n",
                weekday, work_stat.day, hours, cumulative_week_hours
            ));
        }
    }

    stats_content.push_str(&format!(
        "\n| Work Days | {}     |\n",
        stats.total_work_days
    ));
    stats_content.push_str(&format!(
        "| Total     | {:.2} |\n",
        stats.total_duration_in_minutes as f32 / 60.0
    ));

    // replace env, use config
    let daily_rate: f32 = env::var("NOST_WORK_SALARY")
        .unwrap_or_else(|_| {
            eprintln!("NOST_WORK_SALARY environment variable not set.");
            "0".to_string()
        })
        .parse()
        .unwrap_or(0.0);

    // replace env, use config
    let currency = env::var("NOST_WORK_CURRENCY").unwrap_or_else(|_| {
        eprintln!("NOST_WORK_CURRENCY environment variable not set.");
        "EUR".to_string()
    });

    let salary = if stats.total_work_days > 0 {
        daily_rate * stats.total_work_days as f32
    } else {
        0.0
    };

    stats_content.push_str(&format!("| Salary    | {:.2} {} |\n", salary, currency));

    stats_content
}

#[cfg(test)]
mod tests {
    use super::print_stats;
    use crate::statistics::models::{MonthStats, Stats, WeekId, WeekStats};
    use std::{collections::HashMap, env};

    #[test]
    #[serial_test::serial]
    fn print_stats_renders_totals_and_salary() {
        env::set_var("NOST_WORK_SALARY", "120");
        env::set_var("NOST_WORK_CURRENCY", "EUR");

        let mut work_stats_by_week = HashMap::new();
        work_stats_by_week.insert(
            WeekId {
                year: 2026,
                week: 32,
            },
            WeekStats {
                total_duration_in_minutes: 180,
                work_stats: vec![
                    Stats {
                        day: "2026-08-05".to_string(),
                        length_in_minutes: 60,
                    },
                    Stats {
                        day: "2026-08-06".to_string(),
                        length_in_minutes: 120,
                    },
                ],
            },
        );

        let stats = MonthStats {
            total_duration_in_minutes: 180,
            total_work_days: 2,
            work_stats_by_week,
        };

        let rendered = print_stats(stats);

        assert!(rendered.contains("| Work Days | 2"));
        assert!(rendered.contains("| Total     | 3.00 |"));
        assert!(rendered.contains("| Salary    | 240.00 EUR |"));
    }

    #[test]
    #[serial_test::serial]
    fn print_stats_sorts_weeks_and_days() {
        env::set_var("NOST_WORK_SALARY", "100");
        env::set_var("NOST_WORK_CURRENCY", "EUR");

        let mut work_stats_by_week = HashMap::new();
        // Insert week 33 first on purpose (to verify sorting by week id)
        work_stats_by_week.insert(
            WeekId {
                year: 2026,
                week: 33,
            },
            WeekStats {
                total_duration_in_minutes: 60,
                work_stats: vec![Stats {
                    day: "2026-08-12".to_string(),
                    length_in_minutes: 60,
                }],
            },
        );

        // Insert week 32 with unsorted days (to verify day sorting)
        work_stats_by_week.insert(
            WeekId {
                year: 2026,
                week: 32,
            },
            WeekStats {
                total_duration_in_minutes: 120,
                work_stats: vec![
                    Stats {
                        day: "2026-08-06".to_string(),
                        length_in_minutes: 60,
                    },
                    Stats {
                        day: "2026-08-05".to_string(),
                        length_in_minutes: 60,
                    },
                ],
            },
        );

        let stats = MonthStats {
            total_duration_in_minutes: 180,
            total_work_days: 3,
            work_stats_by_week,
        };

        let rendered = print_stats(stats);

        let idx_0805 = rendered.find("2026-08-05").unwrap();
        let idx_0806 = rendered.find("2026-08-06").unwrap();
        let idx_0812 = rendered.find("2026-08-12").unwrap();

        assert!(idx_0805 < idx_0806);
        assert!(idx_0806 < idx_0812);
    }

    #[test]
    #[serial_test::serial]
    fn print_stats_zero_work_days_has_zero_salary() {
        env::set_var("NOST_WORK_SALARY", "500");
        env::set_var("NOST_WORK_CURRENCY", "USD");

        let stats = MonthStats {
            total_duration_in_minutes: 0,
            total_work_days: 0,
            work_stats_by_week: HashMap::new(),
        };

        let rendered = print_stats(stats);

        assert!(rendered.contains("| Work Days | 0"));
        assert!(rendered.contains("| Total     | 0.00 |"));
        assert!(rendered.contains("| Salary    | 0.00 USD |"));
    }
}

use chrono::Datelike;
use chrono::Local;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn name_file() -> String {
    let today = Local::now().date_naive();
    let day_of_month = today.day();
    let file_name = format!("{:02}.md", day_of_month);
    file_name.to_string()
}

pub fn compose_file_path(base_path: &str) -> String {
    let today = Local::now().date_naive();
    let year = today.year();
    let month = format!("{:02}", today.month());

    format!("{}/temp/{}/{}/{}/", base_path, year, month, week_of_month())
}

pub fn get_now_as_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

fn week_of_month() -> u32 {
    let today = chrono::Local::now().date_naive();
    // Get the first day of the month
    let first_day = today.with_day(1).unwrap();
    // Calculate the week number for today and the first day of the month
    let week_today = today.iso_week().week();
    let week_first = first_day.iso_week().week();
    // Week of month is difference + 1
    week_today - week_first + 1
}

fn get_day_suffix(day: u32) -> &'static str {
    match day {
        11..=13 => "th",
        _ => match day % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        },
    }
}

pub fn get_date_as_text_en() -> String {
    let now = Local::now();

    let weekday = now.format("%A").to_string(); // e.g., "Thursday"
    let day = now.day(); // e.g., 7
    let month = now.format("%B").to_string(); // e.g., "August"
    let year = now.year(); // e.g., 2025

    let suffix = get_day_suffix(day); // e.g., "th"
    let formatted_date = format!("{}, {} {}{}, {}", weekday, month, day, suffix, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_date_as_text_fr() -> String {
    let now = Local::now();

    let weekdays = [
        "Dimanche", "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi",
    ];
    let months = [
        "janvier",
        "février",
        "mars",
        "avril",
        "mai",
        "juin",
        "juillet",
        "août",
        "septembre",
        "octobre",
        "novembre",
        "décembre",
    ];

    let weekday = weekdays[now.weekday().num_days_from_sunday() as usize];
    let day = now.day();
    let month = months[(now.month() - 1) as usize];
    let year = now.year();

    let formatted_date = format!("{} {} {} {}", weekday, day, month, year);
    let date_line = format!("# {}\n", formatted_date);

    date_line
}

pub fn get_or_create_not(title: Option<String>) -> std::io::Result<String> {
    // get all existing notes

    match title {
        Some(title) => {
            // todo: check if not title is correct
            create_not(Some(title.clone())).unwrap();

            let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
                eprintln!("NOST_NOT_PATH environment variable not set.");
                panic!("NOST_NOT_PATH not set");
            });
            let not_file_path = compose_file_path(&not_path);
            let not_file_name = name_file();
            let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

            println!(
                "Using NOST_NOT_PATH in get_or_create_not: {}",
                full_not_file_path
            );

            Ok(full_not_file_path)
        }
        None => {
            let new_not_path = create_not(None);
            Ok(new_not_path.unwrap())
        }
    }
}

pub fn create_not(title: Option<String>) -> std::io::Result<String> {
    // handle pathes
    let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOST_NOT_PATH environment variable not set.");
        panic!("NOST_NOT_PATH not set");
    });

    println!("Using NOST_NOT_PATH: {}", not_path);

    let not_file_path = compose_file_path(&not_path);

    let not_file_name = match &title {
        Some(t) => t.clone(), // todo: validate t here
        None => name_file(),
    };

    let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

    // create folders if needed
    if let Err(e) = create_dir_all(&not_file_path) {
        return Err(Error::other(format!(
            "🛑 Failed to create directory: {}",
            e
        )));
    }

    // only create the file if it does not exist
    if Path::new(&full_not_file_path).exists() {
        println!("Not already existed.");
        return Ok(full_not_file_path);
    }

    // create the file
    match File::create(&full_not_file_path) {
        Ok(_file) => {
            println!("✅ File created: {}", full_not_file_path);
        }
        Err(e) => {
            eprintln!("Error creating file: {}", e);
        }
    };

    // add the not header metadata
    // eg: [//]: # "not:{uid: '5ef05459-9af2-4760-8f46-3262b49803fc', created_at: 2025-06-11 01:50:56 +02:00, version: '0.1.0'}"
    let not_info = format!(
        "\"not:{{uid: '{}', created_at: {}, version: '0.0.0'}}\"",
        uuid::Uuid::new_v4(),
        get_now_as_string()
    );
    let header = format!("[//]: # {}\n", not_info);
    append(full_not_file_path.clone().into(), &header)
        .expect("🛑 Failed to append not metatadata.");

    // append the current date as text
    // let date = Local::now().format("%Y-%m-%d");
    // let date_line = format!("# {}\n", date);
    let date_line = match env::var("NOST_LANGUAGE")
        .unwrap_or_else(|_| "en".to_string())
        .as_str()
    {
        "fr" => get_date_as_text_fr(),
        _ => get_date_as_text_en(), // default to French
    };
    append(full_not_file_path.clone().into(), &date_line)
        .expect("🛑 Failed to append date as text.");

    println!("✅ New \"not\" has successfully being initiated.");

    Ok(full_not_file_path)
}

pub fn annotate(content: &str, not_path: &str) {
    let annotation = format!("[//]: # {}\n", content);
    append(not_path.into(), &annotation).expect("🛑 Failed to annotate.");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_append() {
        use std::fs;
        use std::io::Read;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_append.txt");

        // Create the file first
        fs::File::create(&file_path).unwrap();

        // Call append
        let content = "Test content generated from test_append test!";
        super::append(file_path.clone(), content).expect("Failed to append");

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        assert!(file_content.contains(content));
    }

    #[test]
    fn test_annotate() {
        use std::fs;
        use std::io::Read;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_annotate.txt");

        // Create the file first
        fs::File::create(&file_path).unwrap();

        // Call annotate
        let annotation_content = "annotate test content";
        super::annotate(annotation_content, file_path.to_str().unwrap());

        // Read back the content
        let mut file = fs::File::open(&file_path).unwrap();
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        // The annotation should be wrapped as [//]: # "..."
        let expected = format!("[//]: # {}\n", annotation_content);
        assert!(file_content.contains(&expected));
    }
}

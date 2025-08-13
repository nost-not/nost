use chrono::Local;
use dotenv::dotenv;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;
use std::path::Path;

mod not;
mod work;

// todo: give the possibility to create a note for a specific day, eg: 2025-12-31
// todo: add the templates content to the file
// todo: add a version number for nost and for not
// todo: create a repo for not in md
// todo: correct the tests
fn main() {
    dotenv().ok();

    work::test_module_injection();

    // handle pathes
    let not_path = env::var("NOST_NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOST_NOT_PATH environment variable not set.");
        Err("NOST_NOT_PATH not set").unwrap()
    });

    println!("Using NOST_NOT_PATH: {}", not_path);

    let not_file_path = not::compose_file_path(&not_path);
    let not_file_name = not::name_file();
    let full_not_file_path = format!("{}{}", &not_file_path, not_file_name);

    // create folders if needed
    if let Err(e) = create_dir_all(&not_file_path) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    // only create the file if it does not exist
    if Path::new(&full_not_file_path).exists() {
        eprintln!(
            "This note already exists. Nothing has been done.: {}",
            full_not_file_path
        );
        return;
    }

    // create the file
    let mut _not_file = match File::create(&full_not_file_path) {
        Ok(_file) => {
            println!("File created at: {}", full_not_file_path);
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
        Local::now().format("%Y-%m-%d %H:%M:%S %Z")
    );
    let header = format!("[//]: # {}\n", not_info);
    not::append(full_not_file_path.clone().into(), &header)
        .expect("Failed to append not metatadata.");

    // append the current date as text
    // let date = Local::now().format("%Y-%m-%d");
    // let date_line = format!("# {}\n", date);
    let date_line = match env::var("NOST_LANGUAGE")
        .unwrap_or_else(|_| "en".to_string())
        .as_str()
    {
        "fr" => not::get_date_as_text_fr(),
        _ => not::get_date_as_text_en(), // default to French
    };
    not::append(full_not_file_path.clone().into(), &date_line)
        .expect("Failed to append date as text.");

    println!("File created at: {}", full_not_file_path);
}

// todo: this tests are not correct, they need to be moved
// in corresponding modules and adapted
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_compose_file_path() {
        let expected_path = "./temp/not";
        assert_eq!(compose_file_path(), expected_path);
    }

    #[test]
    fn test_name_file() {
        let expected_name = "example.txt";
        assert_eq!(name_file(), expected_name);
    }

    #[test]
    fn test_file_creation() {
        let not_file_path = compose_file_path();
        let not_file_name = name_file();
        let full_not_file_path = format!("{}{}", not_file_path, not_file_name);

        // Ensure the directory exists
        assert!(create_dir_all(&not_file_path).is_ok());

        // Create the file
        let file_result = File::create(&full_not_file_path);
        assert!(file_result.is_ok());

        // Check if the file exists
        assert!(fs::metadata(&full_not_file_path).is_ok());

        // Cleanup
        fs::remove_file(&full_not_file_path).unwrap();
        fs::remove_dir_all(&not_file_path).unwrap();
    }
}

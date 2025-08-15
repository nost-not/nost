use dotenv::dotenv;
use std::env;

mod not;
mod work;

// todo: give the possibility to create a note for a specific day, eg: 2025-12-31
// todo: add the templates content to the file
// todo: add a version number for nost and for not
// todo: correct the tests
fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        println!("You need to pass at least one argument.");
        std::process::exit(1);
    }

    if args[1] == "not" {
        if args.len() > 2 {
            println!("Creating not with title: {}", args[1]);
            not::create_not(Some(args[2].clone())).unwrap();
        } else {
            not::create_not(None).unwrap();
        }
        std::process::exit(0);
    } else if args[1] == "start-work" {
        work::start_work();
        std::process::exit(0);
    } else if args[1] == "stop-work" {
        work::stop_work();
        std::process::exit(0);
    } else {
        eprintln!("Unknown command: \"{}\"", args[1]);
        std::process::exit(1);
    }
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

use dotenv::dotenv;
use std::env;
use std::fs::create_dir_all;
use std::fs::File;

fn main() {
    dotenv().ok();

    let not_path = env::var("NOT_PATH").unwrap_or_else(|_| {
        eprintln!("NOT_PATH environment variable not set.");
        Err("NOT_PATH not set").unwrap()
    });

    let not_file_path = compose_file_path();
    let not_file_name = name_file();
    let not_file_path_without_title = format!("{}{}", not_path, not_file_path);
    let full_not_file_path = format!("{}{}", not_file_path_without_title, not_file_name);

    if let Err(e) = create_dir_all(&not_file_path_without_title) {
        eprintln!("Failed to create directory: {}", e);
        return;
    }

    match File::create(&full_not_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };

    println!("File created at: {}", full_not_file_path);
}

fn name_file() -> String {
    // todo: complete here
    let file_name = "example.txt";
    file_name.to_string()
}

fn compose_file_path() -> String {
    // todo: complete here
    let file_path = "/temp/";
    file_path.to_string()
}

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

use std::fs::{create_dir_all, File};

use crate::{
    configurations::get::get_value_from_config, files::build_paths::build_folder_path_for_now,
};

pub fn new_folder() -> std::io::Result<String> {
    // compose path of the day
    let not_path = get_value_from_config("not_path").unwrap();
    let new_folder_path = build_folder_path_for_now(&not_path);

    // create folder of the composed path (and recursive parents if needed)
    match create_dir_all(&new_folder_path) {
        Ok(_result) => {
            println!("✅ Folder has been created successfully!");

            // create a the config { "created_at": "2025-12-31T23:59:59Z", "type": "day" }
            let config_file_path = format!("{}{}", &new_folder_path, ".not-config.json");

            match File::create(&config_file_path) {
                Ok(_file) => {
                    println!("✅ Config file created: {}", config_file_path);
                    // add minimal content to the config file
                }
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                }
            };

            // create the default file in the folder
            let default_file_path = format!("{}{}", &new_folder_path, "not.md");

            match File::create(&default_file_path) {
                Ok(_file) => {
                    println!("✅ Default file created: {}", default_file_path);
                    // add minimal content to the default file: the date and add an info in config file
                }
                Err(e) => {
                    eprintln!("Error creating file: {}", e);
                }
            };
        }
        Err(e) => {
            eprintln!("Error creating folder: {}", e);
            return Err(e);
        }
    }

    Ok("Nost has created the folder successfully!".to_string())
}

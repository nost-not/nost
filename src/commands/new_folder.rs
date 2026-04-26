use std::fs::create_dir_all;

use crate::{
    configurations::get::get_value_from_config, files::build_paths::build_folder_path_for_now,
};

pub fn new_folder() -> std::io::Result<String> {
    // compose path of the day
    let not_path = get_value_from_config("not_path").unwrap();
    let new_folder_path = build_folder_path_for_now(&not_path);

    // create folder of the composed path (and recursive parents if needed)
    match create_dir_all(&new_folder_path) {
        Ok(_result) => Ok("✅ Folder has been created successfully!".to_string()),
        Err(e) => {
            eprintln!("Error creating folder: {}", e);
            Err(e)
        }
    }
}

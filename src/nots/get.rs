use crate::files::build_paths::build_file_path_for_now;
use crate::files::name::name;
use crate::nots::create::create_not;
use std::env;

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
            let not_file_path = build_file_path_for_now(&not_path);
            let not_file_name = name();
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

use std::{env, fs::read_dir, io::Result as IoResult, path::PathBuf};

use regex::Regex;

pub fn find_all_not_files(path: PathBuf) -> IoResult<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut paths: Vec<PathBuf> = vec![path];

    let folder_regex = Regex::new(r"^\d+$").unwrap();
    let file_regex = Regex::new(r".*\d+\.md$").unwrap();

    while let Some(current) = paths.pop() {
        match read_dir(&current) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let current_path = entry.path();
                    if let Some(name) = current_path.file_name().and_then(|name| name.to_str()) {
                        if current_path.is_dir() {
                            if folder_regex.is_match(name) {
                                paths.push(current_path);
                            }
                        } else if file_regex.is_match(name) {
                            files.push(current_path);
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        }
    }

    files.sort();

    Ok(files)
}

pub fn get_current_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let execution_path = env::current_exe()?;
    let execution_dir = execution_path
        .parent()
        .ok_or("Could not determine executable directory")?;
    println!("Execution directory: {:?}\n", execution_dir);

    Ok(execution_dir.to_path_buf())
}

pub fn get_project_root() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let execution_dir = get_current_directory()?;
    let target_dir = execution_dir
        .parent()
        .ok_or("Could not determine target directory")?;
    let project_root = target_dir
        .parent()
        .ok_or("Could not determine project root")?;
    println!("Project root: {:?}\n", project_root);

    Ok(project_root.to_path_buf())
}

#[cfg(test)]
mod tests {
    use crate::files::find::find_all_not_files;

    #[test]
    #[serial_test::serial]
    fn test_get_not_paths() {
        use std::fs::{self, File};
        use std::io::Write;
        use tempfile::tempdir;

        // Create a temporary directory
        let dir = tempdir().unwrap();
        let base = dir.path();

        // Create subfolders and files
        let week_folder = base.join("1");
        fs::create_dir(&week_folder).unwrap();

        let file1 = week_folder.join("01.md");
        let file2 = week_folder.join("02.md");
        let file3 = week_folder.join("not_a_note.txt");

        File::create(&file1).unwrap().write_all(b"note 1").unwrap();
        File::create(&file2).unwrap().write_all(b"note 2").unwrap();
        File::create(&file3)
            .unwrap()
            .write_all(b"not a note")
            .unwrap();

        // Should find only .md files in numeric folders
        let found = find_all_not_files(base.to_path_buf()).unwrap();

        let found_files: Vec<_> = found
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap().to_string())
            .collect();

        assert!(found_files.contains(&"01.md".to_string()));
        assert!(found_files.contains(&"02.md".to_string()));
        assert!(!found_files.contains(&"not_a_note.txt".to_string()));
        assert_eq!(found_files.len(), 2);
    }
}

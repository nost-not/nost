use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

#[test]
#[serial_test::serial]
fn test_append() {
    use crate::files::append::append;
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
    append(file_path.clone(), content).expect("Failed to append");

    // Read back the content
    let mut file = fs::File::open(&file_path).unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();

    assert!(file_content.contains(content));
}

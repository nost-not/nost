use std::io::Result;
use std::path::PathBuf;

pub fn ends_with_line_break(file_path: PathBuf) -> Result<bool> {
    let content = std::fs::read_to_string(file_path)?;
    Ok(content.ends_with('\n'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_ends_with_line_break() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_line_break.txt");

        // Write content with a line break
        fs::write(&file_path, "Hello world!\n").unwrap();
        assert_eq!(ends_with_line_break(file_path.clone()).unwrap(), true);

        // Write content without a line break
        fs::write(&file_path, "Hello world!").unwrap();
        assert_eq!(ends_with_line_break(file_path.clone()).unwrap(), false);
    }
}

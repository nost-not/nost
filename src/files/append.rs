use std::fs::OpenOptions;
use std::io::Result;
use std::io::Write;
use std::path::PathBuf;

pub fn append(file_path: PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_path)?;
    writeln!(file, "{}", content)?;
    Ok(())
}

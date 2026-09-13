use std::{env, path::PathBuf};

pub fn get_current_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let execution_path = env::current_exe()?;
    let execution_dir = execution_path
        .parent()
        .ok_or("Could not determine executable directory")?;

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
    log::debug!("Project root: {:?}\n", project_root);

    Ok(project_root.to_path_buf())
}

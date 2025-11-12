use anyhow::Result;
use std::{fs, path::PathBuf};
const SESSION_FILE: &str = "/tmp/encnotes.session";

pub fn save_session(password: &str) -> Result<()> {
    let path = PathBuf::from(SESSION_FILE);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, password)?;
    Ok(())
}

pub fn clear_session() -> Result<()> {
    if std::path::Path::new(SESSION_FILE).exists() {
        fs::remove_file(SESSION_FILE)?;
    }
    Ok(())
}

pub fn load_session() -> Result<Option<String>> {
    let path = PathBuf::from(SESSION_FILE);
    if !path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(path)?;
    Ok(Some(data.trim().to_string()))
}

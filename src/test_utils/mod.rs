use std::env;
use std::fs;
use std::path::PathBuf;

/// Removes the directory `OCTMM/tmp/`,
/// including the contents. Returns
/// Some(PathBuf) on success, and None
/// on failure
pub fn get_test_dir() -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("tmp");

    let _ = fs::remove_dir_all(&current_dir);
    fs::create_dir(&current_dir).ok()?;

    if current_dir.exists() {
        Some(current_dir)
    } else {
        None
    }
}

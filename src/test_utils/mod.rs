use std::env;
use std::fs;
use std::path::PathBuf;

/// Gets the test directory for a given sub_folder
/// If the sub_folder tries to change its parent, it will
/// return None. Otherwise, returns Some(PathBuf)
#[allow(dead_code)]
fn get_test_dir(sub_folder: &str) -> Option<PathBuf> {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("tmp");

    // Get tmp directory
    let tmp_dir = current_dir.clone();

    // Make sure our tmp_dir actually exists
    if tmp_dir.to_str().is_none() {
        return None;
    }

    // Push sub folder
    current_dir.push(sub_folder);

    // Get parent to sanity check against tmp_dir
    let parent_dir = current_dir.parent();
    // Make sure we didn't somehow accidentally navigate to root
    if parent_dir.is_none() {
        return None;
    }

    let parent_dir = parent_dir.unwrap();

    // Make sure the parent dir & tmp dir are the same
    if tmp_dir.to_str() != parent_dir.to_str() {
        return None;
    }

    Some(current_dir)
}

/// Removes the directory `OCTMM/tmp/`,
/// including the contents. Returns
/// Some(PathBuf) on success, and None
/// on failure
#[allow(dead_code)]
pub fn make_test_dir(sub_folder: &str) -> Option<PathBuf> {
    let current_dir = get_test_dir(sub_folder)?;

    let _ = fs::remove_dir_all(&current_dir);
    fs::create_dir_all(&current_dir).ok()?;

    if current_dir.exists() {
        Some(current_dir)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::get_test_dir;
    use std::env;
    use std::path::PathBuf;

    #[test]
    fn test_get_test_dir() {
        // Valid
        assert!(get_test_dir("abc123").is_some());
        assert!(get_test_dir("abc_123").is_some());
        assert!(get_test_dir("abc-123").is_some());
        // Invalid
        assert!(get_test_dir("../abc").is_none());
        assert!(get_test_dir("/tmp").is_none());
        assert!(get_test_dir("/abc").is_none());
    }
}

use std::path::PathBuf;

/// Recursively goes up the file tree starting at `dir_path`,
/// looking for file named `target_file_name`.
///
pub fn find_file_starting_from(dir_path: &PathBuf, target_file_name: &str) -> Option<PathBuf> {

    if !dir_path.is_dir() {
        return None;
    }

    // Add filename to the directory path. Does it exist?
    let mut test_path = dir_path.clone();
    test_path.push(target_file_name);

    if test_path.exists() {
        return Some(test_path)
    }

    // Go up one directory and try again
    let mut dir_path = dir_path.clone();
    if !dir_path.pop() {
        return None;
    }
    return find_file_starting_from(&dir_path, &target_file_name);
}

/// Useful for finding file in rust project root dir due uncertainty of cwd during development
/// (based on what executable is being launched or by what means)
///
pub fn find_file_starting_from_cwd(target_file_name: &str) -> Option<String> {
    let cwd = &std::env::current_dir().ok()?;
    let path = find_file_starting_from(cwd, target_file_name)?;
    path.into_os_string().into_string().ok()
}

use std::fs;

pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

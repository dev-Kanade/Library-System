use std::path::PathBuf;

pub fn validate_path(path: &str) -> bool {
    if path.is_empty() {
        return false;
    }

    !path.contains("\"") && !path.contains("'")
}

pub fn expand_path(path: &str) -> Option<PathBuf> {
    if path.starts_with("~") {
        dirs::home_dir().map(|home| {
            PathBuf::from(path.replace("~", home.to_string_lossy().as_ref()))
        })
    } else {
        Some(PathBuf::from(path))
    }
}
use std::path::Path;

pub fn default_session_title(directory: &str) -> String {
    let tail = Path::new(directory)
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("workspace");

    format!("Dark Chat // {tail}")
}

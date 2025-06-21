use anyhow::Context;

pub fn read_dailies_dir(dir_path: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut paths = Vec::new();
    let dir_entries = std::fs::read_dir(dir_path)
        .with_context(|| format!("Failed to read the directory: {}", dir_path))?;
    for entry in dir_entries {
        let entry = entry.with_context(|| format!("failed to read directory {}", dir_path))?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.starts_with("202") {
                    paths.push(path.to_string_lossy().to_string());
                }
            }
        }
    }
    Ok(paths)
}

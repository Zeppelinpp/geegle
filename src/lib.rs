use std::path::PathBuf;
use walkdir::WalkDir;

pub fn get_corpus<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            result.push(entry.path().canonicalize()?);
        }
    }
    Ok(result)
}

use algo::Doc;
use futures::stream::{self, StreamExt};
use ignore::WalkBuilder;
use std::path::PathBuf;

pub struct DocScore {
    pub path: String,
    pub score: f64,
}

pub fn get_corpus<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();

    for entry in WalkBuilder::new(path).standard_filters(true).build() {
        let entry = entry?;
        if entry.file_type().map_or(false, |ft| ft.is_file()) {
            result.push(entry.path().canonicalize()?);
        }
    }
    Ok(result)
}

pub async fn load_docs(paths: &[PathBuf]) -> Vec<Doc> {
    stream::iter(paths)
        .map(|p| async move {
            let content = tokio::fs::read_to_string(p).await.unwrap_or_default();
            Doc {
                path: p.to_string_lossy().to_string(),
                content,
            }
        })
        .buffer_unordered(128)
        .collect()
        .await
}

use algo::types::Doc;
use colored::Colorize;
use core::fmt;
use futures::stream::{self, StreamExt};
use ignore::WalkBuilder;
use std::path::PathBuf;

pub struct DocScore {
    pub path: String,
    pub score: f64,
}

impl fmt::Display for DocScore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}\t{}",
            self.path.cyan(),
            format!("{:.2}", self.score).green()
        )
    }
}

pub fn tokenize(text: &str) -> Vec<String> {
    text.to_lowercase()
        .split_whitespace()
        .map(|s| s.trim_matches(|c: char| !c.is_alphanumeric()).to_string())
        .filter(|s| !s.is_empty())
        .collect()
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

pub fn get_score(terms: &str, docs: &[Doc], algo: &str) -> Vec<DocScore> {
    let terms = tokenize(terms);
    let stats = algo::compute_stats(&terms, docs, algo);
    docs
        .iter()
        .map(|d| {
            let total: f64 = terms
                .iter()
                .map(|t| algo::score_with_stats(t, &d.content, &stats, algo))
                .sum();
            DocScore {
                path: d.path.clone(),
                score: total / terms.len() as f64,
            }
        })
        .collect()
}

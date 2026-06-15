use crate::{tf, types::Doc};

pub struct Bm25Params {
    pub k: f64,
    pub b: f64,
}

impl Default for Bm25Params {
    fn default() -> Self {
        Self { k: 1.5, b: 0.75 }
    }
}

/// BM25 IDF
pub fn _idf(query: &str, docs: &[Doc]) -> f64 {
    let n = docs.len() as f64;
    let df = docs.iter().filter(|d| tf(query, &d.content) > 0.0).count() as f64;
    ((n - df + 0.5) / (df + 0.5)).ln()
}

pub fn _avg_dl(docs: &[Doc]) -> f64 {
    let total: f64 = docs
        .iter()
        .map(|d| d.content.trim().split_whitespace().count() as f64)
        .sum();
    total as f64 / docs.len().max(1) as f64
}

/// BM25
/// `avg_dl` should be pre-computed once per corpus (e.g. via [`_avg_dl`])
/// rather than recomputed for every query/document pair.
pub fn bm25(query: &str, content: &str, idf: f64, avg_dl: f64, params: Option<Bm25Params>) -> f64 {
    let tf = tf(query, content);
    let dl = content.trim().split_whitespace().count().max(1) as f64;
    let params = params.unwrap_or_default();

    idf * tf * (params.k + 1.0) / (tf + params.k * (1.0 - params.b + params.b * avg_dl / dl))
}

pub mod bm25;
pub mod tf_idf;
pub mod types;

use std::collections::HashMap;
use types::Doc;

/// Pre-computed corpus-wide statistics used to score terms without
/// re-scanning the whole document set.
///
/// - `idf`: cached inverse document frequency per query term.
/// - `avg_dl`: average document length, only meaningful for BM25.
pub struct AlgoStats {
    pub idf: HashMap<String, f64>,
    pub avg_dl: f64,
}

/// Score `query` against `content` using full corpus statistics.
///
/// Note: this recomputes global stats on every call. For multi-term or
/// multi-document scoring, prefer [`compute_stats`] + [`score_with_stats`].
pub fn score(query: &str, content: &str, docs: &[Doc], algo: &str) -> f64 {
    let idf = idf(query, docs, algo);
    match algo {
        "bm25" => bm25::bm25(query, content, idf, bm25::_avg_dl(docs), None),
        _ => tf_idf::tf_idf(query, content, idf),
    }
}

/// Pre-compute global statistics (IDF per term, average doc length) once.
/// Use with [`score_with_stats`] to avoid repeated full-corpus scans.
pub fn compute_stats(terms: &[String], docs: &[Doc], algo: &str) -> AlgoStats {
    let mut idf_map = HashMap::with_capacity(terms.len());
    for term in terms {
        idf_map.insert(term.clone(), idf(term, docs, algo));
    }
    let avg_dl = if algo == "bm25" {
        bm25::_avg_dl(docs)
    } else {
        0.0
    };
    AlgoStats {
        idf: idf_map,
        avg_dl,
    }
}

/// Score a single term using pre-computed statistics.
pub fn score_with_stats(query: &str, content: &str, stats: &AlgoStats, algo: &str) -> f64 {
    let idf = *stats.idf.get(query).unwrap_or(&0.0);
    match algo {
        "bm25" => bm25::bm25(query, content, idf, stats.avg_dl, None),
        _ => tf_idf::tf_idf(query, content, idf),
    }
}

/// Count case-insensitive exact occurrences of `query` in `content`.
///
/// Tokenization mirrors `split_whitespace()`: punctuation attached to a
/// word is considered part of that word. Normalize via `tokenize()` first
/// when exact matching is required.
pub fn tf(query: &str, content: &str) -> f64 {
    let query = query.to_lowercase();
    content
        .trim()
        .split_whitespace()
        .filter(|w| w.to_lowercase() == query)
        .count() as f64
}

/// Compute inverse document frequency for `query` using the chosen algorithm.
///
/// - `"bm25"`: Robertson BM25 IDF.
/// - anything else: smoothed TF-IDF IDF (`log((N+1)/(df+1)) + 1`).
pub fn idf(query: &str, docs: &[Doc], algo: &str) -> f64 {
    match algo {
        "bm25" => crate::bm25::_idf(query, docs),
        _ => crate::tf_idf::_idf(query, docs),
    }
}

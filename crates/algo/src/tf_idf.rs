use crate::{tf, types::Doc};

pub fn _idf(query: &str, docs: &[Doc]) -> f64 {
    let n = docs.len() as f64;
    let df = docs.iter().filter(|d| tf(query, &d.content) > 0.0).count() as f64;
    ((n + 1.0) / (df + 1.0)).ln() + 1.0
}

pub fn tf_idf(query: &str, content: &str, idf: f64) -> f64 {
    tf(query, content) as f64 / idf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tf() {
        assert_eq!(tf("the", "the quick brown fox jumps over the lazy dog"), 2.0);
        assert_eq!(tf("a", " She is a beautiful girl, a mother. "), 2.0);
        assert_eq!(tf("a", " taaadfaa "), 0.0);
    }
    fn load_test_corpus() -> Vec<Doc> {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        [
            root.join("test_files/test_file.txt"),
            root.join("test_files/test_file_2.txt"),
            root.join("test_files/test_files_3.txt"),
        ]
        .into_iter()
        .map(|p| Doc {
            path: p.to_string_lossy().to_string(),
            content: std::fs::read_to_string(&p).unwrap(),
        })
        .collect()
    }

    #[test]
    fn test_idf() {
        let corpus = load_test_corpus();

        let idf_the = _idf("the", &corpus);
        assert!((idf_the - ((3.0_f64 + 1.0) / (3.0 + 1.0)).ln() - 1.0).abs() < 1e-9);

        let idf_cat = _idf("cat", &corpus);
        assert!((idf_cat - ((3.0_f64 + 1.0) / (1.0 + 1.0)).ln() - 1.0).abs() < 1e-9);

        let idf_none = _idf("nonexistent", &corpus);
        assert!((idf_none - ((3.0_f64 + 1.0) / (0.0 + 1.0)).ln() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_tf_idf() {
        let corpus = load_test_corpus();
        let idf_the = _idf("the", &corpus);

        let score = tf_idf("the", &corpus[0].content, idf_the);
        assert!((score - 1.0 / idf_the).abs() < 1e-9);
    }
}

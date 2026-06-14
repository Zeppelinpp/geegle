use std::{collections::HashMap, fs};

fn _tf(query: &str, content: &str) -> i32 {
    let words = content.trim().split_whitespace();
    let mut count = 0;
    for word in words {
        if query.to_lowercase() == word.to_lowercase() {
            count += 1;
        }
    }
    count
}

fn _idf(query: &str, corpus: Vec<String>) -> f64 {
    let map: HashMap<String, i32> = corpus
        .into_iter()
        .map(|path| {
            let content = fs::read_to_string(&path).unwrap_or(String::new());
            let tf = _tf(query, &content);
            (path, tf)
        })
        .collect();

    let n = map.len() as f64;
    let df = map.values().filter(|&&v| v > 0).count() as f64;
    ((n + 1.0) / (df + 1.0)).ln() + 1.0
}

pub fn tf_idf(query: &str, content: &str, corpus: Vec<String>) -> f64 {
    _tf(query, content) as f64 / _idf(query, corpus)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tf() {
        assert_eq!(_tf("the", "the quick brown fox jumps over the lazy dog"), 2);
        assert_eq!(_tf("a", " She is a beautiful girl, a mother. "), 2);
        assert_eq!(_tf("a", " taaadfaa "), 0);
    }
    #[test]
    fn test_idf() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let corpus = vec![
            root.join("test_files/test_file.txt")
                .to_string_lossy()
                .to_string(),
            root.join("test_files/test_file_2.txt")
                .to_string_lossy()
                .to_string(),
            root.join("test_files/test_files_3.txt")
                .to_string_lossy()
                .to_string(),
        ];
        let idf_the = _idf("the", corpus.clone());
        assert!((idf_the - ((3.0_f64 + 1.0) / (2.0 + 1.0)).ln() - 1.0).abs() < 1e-9);
        let idf_cat = _idf("cat", corpus.clone());
        assert!((idf_cat - ((3.0_f64 + 1.0) / (1.0 + 1.0)).ln() - 1.0).abs() < 1e-9);
        let idf_none = _idf("nonexistent", corpus);
        assert!((idf_none - ((3.0_f64 + 1.0) / (0.0 + 1.0)).ln() - 1.0).abs() < 1e-9);
    }
}

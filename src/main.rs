use algo::tf_idf;
use clap::Parser;
use geegle::get_corpus;
use std::collections::HashMap;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dir: String,
    #[arg(short, long)]
    query: String,
}

fn main() {
    let args = Args::parse();
    let corpus = get_corpus(&args.dir).unwrap_or_default();

    if corpus.is_empty() {
        eprintln!("no files found in {}", args.dir);
        return;
    }

    let corpus_strings: Vec<String> = corpus
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect();

    let mut scores: HashMap<String, f64> = HashMap::new();
    for path in &corpus {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        let score = tf_idf(&args.query, &content, corpus_strings.clone());
        scores.insert(path.to_string_lossy().to_string(), score);
    }

    for (path, score) in &scores {
        println!("{}\t{}", path, score);
    }
}

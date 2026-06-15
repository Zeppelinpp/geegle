use clap::Parser;
use geegle::{get_corpus, get_score, load_docs};

#[derive(Parser, Debug)]
#[command(name = "geegle", about = "Search files with keyword-based ranking")]
struct Args {
    /// Directory to search
    #[arg(short, long, default_value = ".")]
    dir: String,
    /// Keyword based query
    #[arg(short, long)]
    query: String,
    /// Algorithm: bm25, tfidf ...
    #[arg(short = 'a', long, default_value = "bm25")]
    algo: String,
    /// Top-N results
    #[arg(short = 'n', long, default_value = "5")]
    top_n: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let corpus = get_corpus(&args.dir).unwrap_or_default();

    if corpus.is_empty() {
        eprintln!("Given directory is empty");
        return;
    }
    let docs = load_docs(&corpus).await;

    let mut scores = get_score(&args.query, &docs, &args.algo);
    scores.retain(|d| d.score > 0.0);
    scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    for score in scores.iter().take(args.top_n) {
        println!("{}", score);
    }
}

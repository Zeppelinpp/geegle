use algo::{_idf, tf_idf};
use clap::Parser;
use geegle::{DocScore, get_corpus, load_docs};

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    dir: String,
    #[arg(short, long)]
    query: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let corpus = get_corpus(&args.dir).unwrap_or_default();

    if corpus.is_empty() {
        eprintln!("no files found in {}", args.dir);
        return;
    }
    let docs = load_docs(&corpus).await;
    let idf_value = _idf(&args.query, &docs);

    let mut scores: Vec<DocScore> = docs
        .into_iter()
        .map(|d| {
            let score = tf_idf(&args.query, &d.content, idf_value);
            DocScore {
                path: d.path,
                score,
            }
        })
        .collect();

    scores.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    for score in scores {
        if score.score > 0.0 {
            println!("{}\t{}", score.path, score.score);
        }
    }
}

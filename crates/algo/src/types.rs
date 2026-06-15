/// A single document in the corpus.
///
/// `path` is used for display / ranking output, `content` is the raw text
/// that algorithms tokenize and score.
pub struct Doc {
    pub path: String,
    pub content: String,
}

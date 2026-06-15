# geegle

> A blazing-fast local search engine for your files. Like Google, but for your codebase.

`geegle` ranks files by relevance instead of dumping every grep match.

- For humans: it surfaces the files you actually need.
- For AI agents: it returns a compact ranked list of paths, saving tokens by avoiding huge raw grep output.

## Why not grep?

- `grep` returns every line containing a keyword — noisy and token-heavy.
- `geegle` scores files with `TF-IDF` / `BM25` and returns only the top-N relevant files.
- Agents can read the returned paths progressively instead of ingesting a wall of text.

## Install

```bash
cargo build --release
```

Binary: `./target/release/geegle`

## Usage

```bash
# BM25 (default)
geegle -q "config parser" -d ./src

# TF-IDF
geegle -q "config parser" -d ./src -a tfidf

# Top 3 results
geegle -q "config parser" -d ./src -n 3
```

Output:

```text
../geegle/src/lib.rs      1.85
../geegle/src/main.rs     0.92
```

```bash
geegle -q "<query>" -d <dir> [-a <algo>] [-n <top_n>] [-e <ext1,ext2>]
```

Examples:

```bash
# Exclude .json and .lock files
geegle -q "auth middleware" -d ./src -e json,lock
```

Run `geegle -h` for the full list of flags.

## Algorithms

- **TF-IDF**: simple term frequency weighted by inverse document frequency.
- **BM25**: probabilistic ranking with length normalization.

## License

MIT

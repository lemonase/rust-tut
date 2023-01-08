// Basic Rust CLI Application
// https://rust-cli.github.io/book/tutorial/index.html

use clap::Parser;

#[derive(Parser)]
struct Cli {
    // The pattern to look for
    pattern: String,
    // Path to file to read
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() {
    // First way to get arguments
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no pattern given");

    // Parse arguments passed from CLI
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).unwrap();
    find_matches(&content, &args.pattern);
}

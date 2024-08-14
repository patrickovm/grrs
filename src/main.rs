use std::io::BufRead;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::File::open(&args.path).expect("could not open file");

    let reader = std::io::BufReader::new(content);
    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}

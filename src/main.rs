use clap::Parser;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let file = File::open(&args.path).expect("could not read file");
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        let result = reader.read_line(&mut line);
        let len = match result {
            Ok(len) => len,
            Err(error) => {
                return Err(error.into());
            }
        };
        if len <= 0 {
            break;
        }
        if line.contains(&args.pattern) {
            print!("{}", line);
        }
    }
    Ok(())
}

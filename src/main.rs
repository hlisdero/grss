use anyhow::{Context, Result};
use clap::Parser;
use log::{info, warn};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let stdout = io::stdout(); // get the global stdout entity
    let mut stdout_handle = stdout.lock(); // acquire a lock on it
    info!("Parsing arguments");
    let args = Cli::parse();
    info!("Opening file");
    let file = File::open(&args.path)
        .with_context(|| format!("could not open file `{}`", &args.path.to_string_lossy()))?;
    let mut reader = BufReader::new(file);

    loop {
        let mut line = String::new();
        info!("Reading line from file");
        let len = reader
            .read_line(&mut line)
            .with_context(|| format!("could not read file `{}`", &args.path.to_string_lossy()))?;
        if len <= 0 {
            break;
        }
        info!("Checking if line contains the pattern");
        if line.contains(&args.pattern) {
            writeln!(stdout_handle, "{}", line).with_context(|| "could not write to stdout")?;
        }
    }
    stdout_handle
        .flush()
        .with_context(|| "could not flush output to stdout")?;
    Ok(())
}

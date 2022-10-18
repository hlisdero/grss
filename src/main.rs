use anyhow::{Context, Result};
use clap::Parser;
use grss::find_matches;
use log::info;
use std::fs::File;
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
        .with_context(|| format!("Could not open file `{}`", args.path.display()))?;
    let mut reader = BufReader::new(file);

    find_matches(&mut reader, &args.pattern, &mut stdout_handle)?;

    stdout_handle
        .flush()
        .with_context(|| "Could not flush output to stdout")?;
    Ok(())
}

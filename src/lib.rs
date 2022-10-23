use anyhow::{Context, Result};
use log::info;
use std::io::BufRead;
use std::io::BufReader;

pub fn check_match(content: &str, pattern: &str, writer: &mut impl std::io::Write) -> Result<()> {
    if content.contains(pattern) {
        write!(writer, "{}", content).with_context(|| "Could not write to stdout")?;
    }
    Ok(())
}

pub fn find_matches<R>(
    reader: &mut BufReader<R>,
    pattern: &str,
    writer: &mut impl std::io::Write,
) -> Result<()>
where
    R: std::io::Read,
{
    loop {
        let mut line = String::new();
        info!("Reading line from file");
        let len = reader
            .read_line(&mut line)
            .with_context(|| "Could not read line from file")?;
        if len <= 0 {
            break;
        }
        info!("Checking if line contains the pattern");
        check_match(&line, pattern, writer)?;
    }
    Ok(())
}

#[test]
fn check_match_detects_match() {
    let mut result = Vec::new();
    check_match("lorem ipsum", "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum");
}

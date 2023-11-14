use regex::Regex;
use std::io::{self, BufRead};
use std::path::PathBuf;

use clap::Parser;

/// `col` is a tool to print a specific column from tabular output,
/// e.g. `ls -l | awk '{ print $2 }' -> `ls -l | col 2`
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The column to print. 0 will print the entire line with the provided separator
    column: usize,
    /// Optional input file to read from, if not provided `stdin` will be used
    input: Option<PathBuf>,
    /// Delimiter used to split lines.
    /// Will consider all consecutive characters as a single delimiter,
    /// E.g. "test1 test2" will be split the same as "test1    test2"
    #[clap(short, long, default_value = " ")]
    delimiter: String,
    /// Output separator to add between printed column values
    #[clap(short, long, default_value = "\n")]
    separator: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if let Err(err) = print_columns(&args) {
        eprintln!("ERROR: {err:#}");
    }
    Ok(())
}

fn print_columns(args: &Args) -> anyhow::Result<()> {
    let input: Box<dyn io::BufRead + 'static> = if let Some(path) = &args.input {
        match std::fs::File::open(path) {
            Ok(file) => Box::new(io::BufReader::new(file)),
            Err(err) => {
                anyhow::bail!("{}: {}", path.display(), err);
            }
        }
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    let delimiter_re = Regex::new(&format!("{}+", args.delimiter))?;
    for line in io::BufReader::new(input).lines() {
        let line = line?;
        match args.column {
            0 => print!("{line}{}", args.separator),
            c => {
                let col = delimiter_re
                    .split(&line)
                    .nth(c - 1)
                    .ok_or_else(|| anyhow::anyhow!("Column overflow"))?;
                print!("{col}{}", &args.separator);
            }
        }
    }

    Ok(())
}

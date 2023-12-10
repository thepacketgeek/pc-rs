use std::io;

use clap::Parser;
use clap_stdin::FileOrStdin;
use pc_rs::parse_columns;

/// `pc` is a tool to print a specific column from tabular output,
/// e.g. `ls -l | awk '{ print $2 }' -> `ls -l | pc 2`
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The column to print. 0 will print the entire line with the provided separator
    column: usize,
    /// Optional input file to read from, if not provided `stdin` will be used
    #[clap(default_value = "-")]
    input: FileOrStdin,
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
    let source = io::BufReader::new(args.input.clone().into_reader()?);
    parse_columns(source, args.column, &args.delimiter, |col| {
        print!("{col}{}", args.separator)
    })?;
    Ok(())
}

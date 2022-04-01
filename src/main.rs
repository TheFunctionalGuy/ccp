use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::Context;
use clap::Parser;

struct CrashContext {
    count: usize,
    pc: usize,
    lr: usize,
    paths: Vec<String>,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// File containing crash contexts in the format defined by https://github.com/fuzzware-fuzzer/fuzzware and its
    /// `fuzzware genstats crashcontexts` command
    input_file: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let cc_file = File::open(&args.input_file)
        .with_context(|| format!("Could not read file {:?}", &args.input_file))?;
    let cc_file_reader = BufReader::new(cc_file);

    for line in cc_file_reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

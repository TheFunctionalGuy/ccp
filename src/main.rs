mod app;
mod crossterm;
mod ui;

use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
    time::Duration,
};

use anyhow::Context;
use clap::Parser;

use crate::crossterm::run;

#[derive(Debug)]
pub struct CrashContext {
    count: usize,
    pc: usize,
    lr: usize,
    paths: Vec<String>,
}

impl FromStr for CrashContext {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(4, ' ').collect();

        let count = parts[0].parse::<usize>()?;
        let pc = usize::from_str_radix(parts[1].strip_prefix("0x").unwrap(), 16)?;
        let lr = usize::from_str_radix(parts[2].strip_prefix("0x").unwrap(), 16)?;
        let paths = parts[3].split(' ').map(str::to_string).collect();

        Ok(CrashContext {
            count,
            pc,
            lr,
            paths,
        })
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// File containing crash contexts in the format defined by https://github.com/fuzzware-fuzzer/fuzzware and its
    /// `fuzzware genstats crashcontexts` command
    input_file: std::path::PathBuf,
    /// Milliseconds between each TUI refresh
    #[clap(default_value_t = 250)]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    // Create tick rate
    let tick_rate = Duration::from_millis(args.tick_rate);

    // Read and parse file
    let cc_file = File::open(&args.input_file)
        .with_context(|| format!("Could not read file {:?}", &args.input_file))?;
    let cc_file_reader = BufReader::new(cc_file);

    // Create crash context vector and start GUI
    let mut crash_contexts: Vec<CrashContext> = Vec::new();

    for line in cc_file_reader.lines().skip(1) {
        crash_contexts.push(CrashContext::from_str(&line?)?);
    }
    crash_contexts.sort_by_key(|k| k.pc);

    run(tick_rate, crash_contexts)?;

    Ok(())
}

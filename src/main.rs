mod app;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    num::ParseIntError,
    str::FromStr,
    time::{Duration, Instant},
};

use crate::app::App;

use anyhow::Context;
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Frame, Terminal,
};

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
    #[clap(default_value_t = 250)]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Setup
    enable_raw_mode().expect("Cannot enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Read and parse file
    let cc_file = File::open(&args.input_file)
        .with_context(|| format!("Could not read file {:?}", &args.input_file))?;
    let cc_file_reader = BufReader::new(cc_file);

    let mut crash_contexts: Vec<CrashContext> = Vec::new();

    for line in cc_file_reader.lines().skip(1) {
        crash_contexts.push(CrashContext::from_str(&line?)?);
    }

    // Create tick rate
    let tick_rate = Duration::from_millis(args.tick_rate);

    // Show CUI
    let app = App::new("[C]rash [C]ontext [P]arser", crash_contexts);
    let res = run_app(&mut terminal, app, tick_rate);

    // Restore
    disable_raw_mode().expect("Cannot disable raw mode");
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    KeyCode::Left => app.on_left(),
                    KeyCode::Up => app.on_up(),
                    KeyCode::Right => app.on_right(),
                    KeyCode::Down => app.on_down(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {}

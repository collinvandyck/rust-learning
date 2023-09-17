use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use rql::{
    app::{App, Tick},
    dao::DbType,
};
use std::{
    io::{self, Stdout},
    process,
};

#[derive(clap::Parser)]
struct Args {
    #[arg(env)]
    db_path: String,
}

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn main() {
    if let Err(err) = setup_and_run() {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

fn setup_and_run() -> Result<()> {
    let args = Args::parse();
    let mut term = setup_terminal().context("term setup failed")?;
    let res = run(&args, &mut term);
    restore_terminal(&mut term).context("term restore failed")?;
    res
}

fn run(args: &Args, term: &mut Term) -> Result<()> {
    let db: DbType = DbType::Path(args.db_path.as_str());
    let mut app = App::new(db)?;
    loop {
        app.draw(term)?;
        match app.tick()? {
            Tick::Quit => {
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn setup_terminal() -> Result<Term> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen).context("failed to enter alt screen")?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend).context("create terminal")
}

fn restore_terminal(term: &mut Term) -> Result<()> {
    disable_raw_mode().context("disable raw mode")?;
    execute!(term.backend_mut(), LeaveAlternateScreen).context("leave alt screen")?;
    term.show_cursor().context("show cursor")
}

use anyhow::{Context, Result};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph, Frame, Terminal};
use std::{
    io::{self, Stdout},
    process,
    time::Duration,
};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn main() -> Result<()> {
    let mut term = setup_terminal().context("setup term")?;
    let res = run(&mut term).context("run");
    restore_terminal(&mut term).context("restore term")?;
    if let Err(err) = res {
        eprintln!("{err}");
        process::exit(1);
    }
    Ok(())
}

fn run(term: &mut Term) -> Result<()> {
    loop {
        term.draw(render)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render(frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll")? {
        if let Event::Key(key) = event::read().context("event read")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
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

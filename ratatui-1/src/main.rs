#![allow(unused)]

use anyhow::{bail, Context, Result};
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{Stdout, Write};
use std::{io::stdout, thread, time::Duration};

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    if let Err(err) = enable_raw_mode() {
        leave_alt()?;
        return Err(err.into());
    }

    let term = match init_term() {
        Ok(term) => term,
        Err(err) => {
            leave_alt_and_raw();
            bail!(err);
        }
    };

    leave_alt_and_raw()?;
    Ok(())
}

fn leave_alt_and_raw() -> Result<()> {
    leave_alt()?;
    disable_raw_mode()?;
    Ok(())
}

fn leave_alt() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn init_term() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    Terminal::new(CrosstermBackend::new(stdout()))
        .context("new terminal")
        .and_then(|mut term| {
            term.clear().context("clear term")?;
            Ok(term)
        })
}

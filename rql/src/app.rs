use std::{io::Stdout, time::Duration};

use crate::dao::BlockingDao;
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Nothing,
}

pub struct App {
    dao: BlockingDao,
}

impl App {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let dao = BlockingDao::new(path)?;
        Ok(Self { dao })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        let tables = self.dao.tables()?.join("\n");
        term.draw(move |frame| {
            let greeting = Paragraph::new(format!("{tables}"));
            frame.render_widget(greeting, frame.size());
        })?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Tick> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if KeyCode::Char('c') == key.code && key.modifiers.contains(KeyModifiers::CONTROL) {
                    return Ok(Tick::Quit);
                }
                if KeyCode::Char('q') == key.code {
                    return Ok(Tick::Quit);
                }
            }
        }
        Ok(Tick::Nothing)
    }
}

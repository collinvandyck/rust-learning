use std::{io::Stdout, time::Duration};

use crate::{dao::BlockingDao, widgets::Tables};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{
    prelude::CrosstermBackend,
    widgets::{List, ListItem},
};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
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
        let mut tables = Tables::new(self.dao.tables()?);
        term.draw(move |frame| {
            let items: Vec<ListItem> = tables
                .names
                .iter()
                .map(|n| ListItem::new(n.as_str()))
                .collect();
            let list = List::new(items);
            let state = &mut tables.state;
            frame.render_stateful_widget(list, frame.size(), state);
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
        Ok(Tick::Continue)
    }
}

use std::{io::Stdout, time::Duration};

use crate::{dao::BlockingDao, tables::Tables};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::CrosstermBackend,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
}

pub struct App {
    dao: BlockingDao,
    tables: Tables,
}

enum State {
    Start,
    Table(String),
}

impl App {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let dao = BlockingDao::new(path)?;
        let tables = Tables::new(dao.tables()?);
        Ok(Self { dao, tables })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        term.draw(move |frame| {
            let items: Vec<ListItem> = self
                .tables
                .names
                .iter()
                .map(|n| n.clone())
                .map(|n| ListItem::new(n).style(Style::default().fg(Color::Cyan)))
                .collect();
            let list = List::new(items)
                .block(Block::default().borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                );
            let state = &mut self.tables.state;
            frame.render_stateful_widget(list, frame.size(), state);
        })?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Tick> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if Self::should_quit(key) {
                    return Ok(Tick::Quit);
                }
                match key.code {
                    KeyCode::Char('j') => {
                        self.tables.next();
                    }
                    KeyCode::Char('k') => {
                        self.tables.previous();
                    }
                    _ => {}
                }
            }
        }
        Ok(Tick::Continue)
    }

    fn should_quit(key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            KeyCode::Char('q') => return true,
            _ => false,
        }
    }
}

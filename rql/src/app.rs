use std::{io::Stdout, time::Duration};

use crate::{
    dao::{BlockingDao, Field, Record, DB},
    table::Table,
    tables::Tables,
};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
}

pub struct App {
    dao: BlockingDao,     // db handle
    tables: Tables,       // the list of tables
    table: Option<Table>, // the selected table
}

impl App {
    pub fn new(db: DB) -> Result<Self> {
        let dao = BlockingDao::new(db)?;
        let tables = Tables::new(dao.tables()?);
        let mut table = None;
        if let Some(name) = tables.selected() {
            table.replace(Table::new(dao.clone(), name)?);
        }
        Ok(Self { dao, tables, table })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        term.draw(move |frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(frame.size());
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
            frame.render_stateful_widget(list, chunks[0], state);
            if let Some(table) = &self.table {
                let max = frame.size().height as usize;
                let items: Vec<ListItem> = table
                    .records
                    .iter()
                    .take(max)
                    .map(|record| {
                        record
                            .fields
                            .iter()
                            .map(|field| format!("{:?}", field.val))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .map(|s| ListItem::new(s))
                    .collect();
                let list = List::new(items);
                frame.render_widget(list, chunks[1]);
            }
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
                    KeyCode::Char('j') | KeyCode::Char('k') => {
                        if key.code == KeyCode::Char('j') {
                            self.tables.next();
                        } else {
                            self.tables.previous();
                        }
                        if let Some(name) = self.tables.selected() {
                            self.table.replace(Table::new(self.dao.clone(), name)?);
                        }
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

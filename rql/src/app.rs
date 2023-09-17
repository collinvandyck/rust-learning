use std::{io::Stdout, time::Duration};

use crate::{
    dao::{BlockingDao, DbType},
    table::DbTable,
    tables::DbTables,
};
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    prelude::{Constraint, CrosstermBackend, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, List, ListItem, Row, Table},
};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
}

pub struct App {
    dao: BlockingDao,       // db handle
    tables: DbTables,       // the list of tables
    table: Option<DbTable>, // the selected table
}

impl App {
    pub fn new(db: DbType) -> Result<Self> {
        let dao = BlockingDao::new(db)?;
        let tables = DbTables::new(dao.tables()?);
        let mut table = None;
        if let Some(name) = tables.selected() {
            table.replace(DbTable::new(dao.clone(), name)?);
        }
        Ok(Self { dao, tables, table })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        let size = term.size()?;
        term.draw(move |frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(self.tables.max_len() + 3), // padding
                        Constraint::Max(size.width),
                    ]
                    .as_ref(),
                )
                .split(frame.size());
            let items: Vec<ListItem> = self
                .tables
                .names
                .iter()
                .map(|n| n.clone())
                .map(|n| ListItem::new(n).style(Style::default().fg(Color::Cyan)))
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("[ tables ]")
                        .title_style(Style::default().fg(Color::LightGreen))
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                );
            let state = &mut self.tables.state;
            frame.render_stateful_widget(list, chunks[0], state);
            if let Some(selected_table) = &self.table {
                let header_cells = selected_table
                    .schema
                    .cols
                    .iter()
                    .map(|col| Cell::from(col.name.clone()).style(Style::default()));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1)
                    .bottom_margin(1);
                let max_records = frame.size().height as usize;
                let rows = selected_table
                    .records
                    .iter()
                    .take(max_records)
                    .map(|record| {
                        let cells = record
                            .fields
                            .iter()
                            .map(|field| format!("{}", field.val))
                            .map(|s| Cell::from(s).style(Style::default()));
                        Row::new(cells)
                    });
                let num_cols = selected_table.schema.cols.len();
                let pct = (100.0 / num_cols as f64) as u16;
                let widths = selected_table
                    .schema
                    .cols
                    .iter()
                    .map(|_| Constraint::Percentage(pct))
                    .collect::<Vec<_>>();
                let table: Table = Table::new(rows)
                    .header(header)
                    .block(
                        Block::default()
                            .title(format!("[ Table: {} ]", selected_table.name))
                            .title_style(Style::default().fg(Color::LightGreen))
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().fg(Color::LightGreen))
                    .highlight_symbol(">>")
                    .widths(&widths);
                frame.render_widget(table, chunks[1]);
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
                            self.table.replace(DbTable::new(self.dao.clone(), name)?);
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

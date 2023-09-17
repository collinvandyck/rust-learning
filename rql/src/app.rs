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
use std::{
    io::Stdout,
    time::{Duration, Instant},
};
use tracing::{debug, instrument, trace};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
}

#[derive(Default, PartialEq)]
enum Focus {
    #[default]
    Tables,
    Table,
}

pub struct App {
    dao: BlockingDao,       // db handle
    tables: DbTables,       // the list of tables
    table: Option<DbTable>, // the selected table
    focus: Focus,           // what ui element has focus
}

impl App {
    pub fn new(db: DbType) -> Result<Self> {
        let dao = BlockingDao::new(db)?;
        let tables = DbTables::new(dao.tables()?);
        let mut table = None;
        if let Some(name) = tables.selected() {
            table.replace(DbTable::new(dao.clone(), name)?);
        }
        let focus = Focus::default();
        Ok(Self {
            dao,
            tables,
            table,
            focus,
        })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        let start = Instant::now();
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
            let mut title_style = Style::default();
            if self.focus == Focus::Tables {
                title_style = title_style.fg(Color::LightGreen);
            }
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("[ tables ]")
                        .title_style(title_style)
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                );
            let state = &mut self.tables.state;
            frame.render_stateful_widget(list, chunks[0], state);
            if let Some(selected_table) = &mut self.table {
                let header_cells = selected_table
                    .schema
                    .cols
                    .iter()
                    .map(|col| Cell::from(col.name.clone()).style(Style::default()));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1)
                    .bottom_margin(1);
                let rows = selected_table.records.iter().map(|record| {
                    let cells = record
                        .fields
                        .iter()
                        .map(|field| format!("{}", field.val))
                        .map(|s| Cell::from(s).style(Style::default()));
                    Row::new(cells).height(1)
                });
                let num_cols = selected_table.schema.cols.len();
                let pct = (100.0 / num_cols as f64) as u16;
                let widths = selected_table
                    .schema
                    .cols
                    .iter()
                    .map(|_| Constraint::Percentage(pct))
                    .collect::<Vec<_>>();
                let mut title_style = Style::default();
                if self.focus == Focus::Table {
                    title_style = title_style.fg(Color::LightGreen);
                }
                let table: Table = Table::new(rows)
                    .header(header)
                    .block(
                        Block::default()
                            .title(format!(
                                "[ Table: {} ({} records) ]",
                                selected_table.name, selected_table.count
                            ))
                            .title_style(title_style)
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().fg(Color::LightGreen))
                    .highlight_symbol("")
                    .widths(&widths);
                let state = &mut selected_table.state;
                frame.render_stateful_widget(table, chunks[1], state);
            }
        })?;
        let elapsed = start.elapsed();
        trace!(?elapsed, "Draw");
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Tick> {
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                let start = Instant::now();
                if Self::should_quit(key) {
                    return Ok(Tick::Quit);
                }
                match self.focus {
                    Focus::Tables => match key.code {
                        KeyCode::Char('j')
                        | KeyCode::Char('k')
                        | KeyCode::Char('J')
                        | KeyCode::Char('K') => {
                            if key.code == KeyCode::Char('j') || key.code == KeyCode::Char('J') {
                                self.tables.next();
                            } else {
                                self.tables.previous();
                            }
                            if let Some(name) = self.tables.selected() {
                                self.table.replace(DbTable::new(self.dao.clone(), name)?);
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Enter | KeyCode::Char('o') => {
                            if let Some(table) = &mut self.table {
                                if table.count > 0 {
                                    self.focus = Focus::Table;
                                    table.select_first();
                                }
                            }
                        }
                        KeyCode::Char('q') | KeyCode::Esc => {
                            return Ok(Tick::Quit);
                        }
                        _ => {}
                    },
                    Focus::Table => match key.code {
                        KeyCode::Char('J') | KeyCode::Char('K') => {
                            if key.code == KeyCode::Char('J') {
                                self.tables.next();
                            } else {
                                self.tables.previous();
                            }
                            if let Some(name) = self.tables.selected() {
                                let mut table = DbTable::new(self.dao.clone(), name)?;
                                table.select_first();
                                self.table.replace(table);
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Char('k') => {
                            if let Some(table) = &mut self.table {
                                if key.code == KeyCode::Char('j') {
                                    table.next();
                                } else {
                                    table.previous();
                                }
                            }
                        }
                        KeyCode::Char('h') | KeyCode::Char('q') | KeyCode::Esc => {
                            self.focus = Focus::Tables;
                            if let Some(table) = &mut self.table {
                                table.unselect();
                            }
                        }
                        _ => {}
                    },
                }
                let elapsed = start.elapsed();
                trace!(?elapsed, "Tick");
            }
        }
        Ok(Tick::Continue)
    }

    fn should_quit(key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            _ => false,
        }
    }
}

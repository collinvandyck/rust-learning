use std::collections::{HashMap, HashSet};

use crate::prelude::*;

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Continue,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
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
    dims: Rect,             // how large the frame is
    bindings: KeyBindSet,
}

struct KeyBindSet {
    bindings: HashMap<Focus, HashMap<KeyEvent, Action>>,
}

impl KeyBindSet {
    fn matches(&self, focus: Focus, event: KeyEvent) -> Option<Action> {
        self.bindings
            .get(&focus)
            .map(|b| b.get(&event))
            .flatten()
            .cloned()
    }
}

impl Default for KeyBindSet {
    fn default() -> Self {
        use Action::*;
        let kev = |code: KeyCode| -> KeyEvent { KeyEvent::new(code, KeyModifiers::NONE) };
        let mut bindings = HashMap::default();
        bindings.insert(Focus::Tables, {
            HashMap::from([
                // tablesnext
                (kev(KeyCode::Down), TablesNext),
                (kev(KeyCode::Char('J')), TablesNext),
                (kev(KeyCode::Char('j')), TablesNext),
                /// tablesprev
                (kev(KeyCode::Up), TablesPrev),
                (kev(KeyCode::Char('K')), TablesPrev),
                (kev(KeyCode::Char('k')), TablesPrev),
                // focustable
                (kev(KeyCode::Right), ChangeFocus(Focus::Table)),
                (kev(KeyCode::Char('l')), ChangeFocus(Focus::Table)),
                (kev(KeyCode::Char('o')), ChangeFocus(Focus::Table)),
                (kev(KeyCode::Enter), ChangeFocus(Focus::Table)),
                // quit
                (kev(KeyCode::Char('q')), Quit),
                (kev(KeyCode::Esc), Quit),
            ])
        });
        bindings.insert(Focus::Table, {
            HashMap::from([
                // tablesnext
                (kev(KeyCode::Char('J')), TablesNext),
                // tablesprev
                (kev(KeyCode::Char('K')), TablesPrev),
                // tablenext
                (kev(KeyCode::Down), TableNext),
                (kev(KeyCode::Char('j')), TableNext),
                // tableprev
                (kev(KeyCode::Up), TablePrev),
                (kev(KeyCode::Char('k')), TablePrev),
                // focustables
                (kev(KeyCode::Left), ChangeFocus(Focus::Tables)),
                (kev(KeyCode::Char('h')), ChangeFocus(Focus::Tables)),
                (kev(KeyCode::Char('q')), ChangeFocus(Focus::Tables)),
                (kev(KeyCode::Esc), ChangeFocus(Focus::Tables)),
            ])
        });
        Self { bindings }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    TablesNext,
    TablesPrev,
    TableNext,
    TablePrev,
    ChangeFocus(Focus),
    Quit,
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
        let dims = Rect::default();
        let bindings = KeyBindSet::default();
        Ok(Self {
            dao,
            tables,
            table,
            focus,
            dims,
            bindings,
        })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        let start = Instant::now();
        let size = term.size()?;
        self.dims = size;
        term.draw(move |frame| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(self.tables.max_len() + 1), // 2 border, 1 padding
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
            let table_rows = self.table_rows();
            if let Some(selected_table) = &mut self.table {
                selected_table.set_viewport_rows(table_rows);
                let header_names = selected_table
                    .schema
                    .cols
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect::<Vec<_>>();
                let header_style = Style::default().fg(Color::LightBlue).bold();
                let header_cells = header_names
                    .iter()
                    .map(|name| Cell::from(name.clone()).style(header_style));
                let header = Row::new(header_cells)
                    .style(Style::default())
                    .height(1)
                    .bottom_margin(0);
                let (records, mut state) = selected_table.records();
                let rows = records.iter().enumerate().map(|(row_idx, record)| {
                    let mut row_style = Style::default();
                    if row_idx % 2 == 0 {
                        row_style = row_style.bg(Color::Indexed(234));
                    }
                    let cells = record
                        .fields
                        .iter()
                        .map(|field| format!("{}", field.val))
                        .map(|s| Cell::from(s).style(row_style));
                    Row::new(cells).height(1)
                });
                let widths = selected_table
                    .schema
                    .cols
                    .iter()
                    .enumerate()
                    .map(|(idx, col)| {
                        let header_len = header_names[idx].len();
                        let col_len = selected_table.max_len(col, 4);
                        let len = std::cmp::max(col_len, header_len);
                        Constraint::Max(len.try_into().unwrap())
                    })
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
                                selected_table.name(),
                                selected_table.count
                            ))
                            .title_style(title_style)
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().fg(Color::LightGreen))
                    .highlight_symbol("")
                    .widths(&widths);
                frame.render_stateful_widget(table, chunks[1], &mut state);
            }
        })?;
        let elapsed = start.elapsed();
        trace!(?elapsed, "Draw");
        Ok(())
    }

    fn table_rows(&mut self) -> usize {
        (self.dims.height - 3) as usize // 2 border, 1 header
    }

    pub fn tick(&mut self) -> Result<Tick> {
        let poll_time = Duration::from_secs(24 * 3600);
        if event::poll(poll_time).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                let start = Instant::now();
                if Self::should_quit(key) {
                    return Ok(Tick::Quit);
                }
                if let Some(action) = self.bindings.matches(self.focus, key) {
                    match action {
                        Action::TablesNext => {
                            self.tables.next();
                            if let Some(name) = self.tables.selected() {
                                self.table.replace(DbTable::new(self.dao.clone(), name)?);
                            }
                            if self.focus == Focus::Table {
                                if let Some(table) = self.table.as_mut() {
                                    table.select_first();
                                }
                            }
                        }
                        Action::TablesPrev => {
                            self.tables.previous();
                            if let Some(name) = self.tables.selected() {
                                self.table.replace(DbTable::new(self.dao.clone(), name)?);
                            }
                            if self.focus == Focus::Table {
                                if let Some(table) = self.table.as_mut() {
                                    table.select_first();
                                }
                            }
                        }
                        Action::TableNext => {
                            let table_rows = self.table_rows();
                            if let Some(table) = &mut self.table {
                                table.next(table_rows);
                            }
                        }
                        Action::TablePrev => {
                            let table_rows = self.table_rows();
                            if let Some(table) = &mut self.table {
                                table.previous(table_rows);
                            }
                        }
                        Action::ChangeFocus(focus) => match focus {
                            Focus::Tables => {
                                self.focus = Focus::Tables;
                                if let Some(table) = &mut self.table {
                                    table.unselect();
                                }
                            }
                            Focus::Table => {
                                if let Some(table) = &mut self.table {
                                    if table.count > 0 {
                                        self.focus = Focus::Table;
                                        table.select_first();
                                    }
                                }
                            }
                        },
                        Action::Quit => return Ok(Tick::Quit),
                    }
                }
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

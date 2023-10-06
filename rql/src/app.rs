use crate::prelude::*;
use core::num;
use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
};

pub enum Tick {
    Quit,
    Continue,
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
enum Focus {
    #[default]
    Tables,
    Table,
    Search,
}

pub struct App {
    dao: BlockingDao,       // db handle
    tables: DbTables,       // the list of tables
    table: Option<DbTable>, // the selected table
    focus: Focus,           // what ui element has focus
    dims: Rect,             // how large the frame is
    bindings: KeyBindSet,   // keybindings
    search: Search,
}

#[derive(Clone, Default)]
pub struct Search {
    pub value: Option<String>,
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
        let kevent = |code: KeyCode, m: KeyModifiers| -> KeyEvent { KeyEvent::new(code, m) };
        let key = |code: KeyCode| -> KeyEvent { kevent(code, KeyModifiers::NONE) };
        let ctrl_key = |code: KeyCode| -> KeyEvent { kevent(code, KeyModifiers::CONTROL) };
        let mut bindings = HashMap::default();
        bindings.insert(Focus::Tables, {
            HashMap::from([
                // tablesnext
                (key(KeyCode::Down), TablesNext),
                (key(KeyCode::Char('J')), TablesNext),
                (key(KeyCode::Char('j')), TablesNext),
                /// tablesprev
                (key(KeyCode::Up), TablesPrev),
                (key(KeyCode::Char('K')), TablesPrev),
                (key(KeyCode::Char('k')), TablesPrev),
                // focustable
                (key(KeyCode::Right), ChangeFocus(Focus::Table)),
                (key(KeyCode::Char('l')), ChangeFocus(Focus::Table)),
                (key(KeyCode::Char('o')), ChangeFocus(Focus::Table)),
                (key(KeyCode::Enter), ChangeFocus(Focus::Table)),
                // quit
                (key(KeyCode::Char('q')), Quit),
                (key(KeyCode::Esc), Quit),
            ])
        });
        bindings.insert(Focus::Table, {
            HashMap::from([
                // tablesnext
                (key(KeyCode::Char('J')), TablesNext),
                // tablesprev
                (key(KeyCode::Char('K')), TablesPrev),
                // tablenext
                (key(KeyCode::Down), TableNext),
                (key(KeyCode::Char('j')), TableNext),
                // tableprev
                (key(KeyCode::Up), TablePrev),
                (key(KeyCode::Char('k')), TablePrev),
                // focustables
                (key(KeyCode::Left), ChangeFocus(Focus::Tables)),
                (key(KeyCode::Char('h')), ChangeFocus(Focus::Tables)),
                (key(KeyCode::Char('q')), ChangeFocus(Focus::Tables)),
                (key(KeyCode::Esc), ChangeFocus(Focus::Tables)),
                // pageup
                (key(KeyCode::PageUp), PageUp),
                (ctrl_key(KeyCode::Char('u')), PageUp),
                // pagedown
                (key(KeyCode::PageDown), PageDown),
                (ctrl_key(KeyCode::Char('d')), PageDown),
                (key(KeyCode::Char('/')), ChangeFocus(Focus::Search)),
                (key(KeyCode::Char('?')), ChangeFocus(Focus::Search)),
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
    PageUp,
    PageDown,
    ChangeFocus(Focus),
    Search,
    Quit,
}

impl App {
    pub fn new(db: DbType) -> Result<Self> {
        let dao = BlockingDao::new(db)?;
        let tables = DbTables::new(dao.tables()?);
        let mut table = None;
        let focus = Focus::default();
        let dims = Rect::default();
        let bindings = KeyBindSet::default();
        let search = Search::default();
        let mut app = Self {
            dao,
            tables,
            table,
            focus,
            dims,
            bindings,
            search,
        };
        Ok(app)
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        self.dims = term.size()?;
        if self.table.is_none() && self.tables.selected().is_some() {
            self.open_table()?;
        }
        let num_table_rows = self.num_table_rows();
        let table_records = if let Some(table) = self.table.as_mut() {
            table.set_viewport_rows(num_table_rows);
            Some(table.records()?)
        } else {
            None
        };
        term.draw(move |frame| {
            let mut chrome = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
                .split(frame.size());

            let mut tables_title_style = Style::default();
            let mut table_title_style = Style::default();
            let mut search_title_style = Style::default();
            match self.focus {
                Focus::Tables => {
                    tables_title_style = tables_title_style.fg(Color::LightGreen);
                }
                Focus::Table => {
                    table_title_style = table_title_style.fg(Color::LightGreen);
                }
                Focus::Search => {
                    search_title_style = table_title_style.fg(Color::LightGreen);
                }
            }

            let help = {
                let no_style = Style::default();
                let key_style = Style::default().fg(Color::LightCyan);

                if self.focus == Focus::Search {
                    let mut nav = vec![
                        Span::styled("Esc", key_style),
                        Span::raw(": exit search | "),
                        Span::styled("Enter", key_style),
                        Span::raw(": navigate results || current query: "),
                    ];
                    if let Some(q) = self.search.value.as_ref() {
                        nav.push(Span::styled(q, search_title_style));
                    }
                    nav
                } else {
                    let mut nav = ["j", "k", "h", "l", "↓", "↑", "←", "→"]
                        .iter()
                        .zip(std::iter::repeat(Span::styled(",", no_style)))
                        .map(|(s, sep)| [sep, Span::styled(*s, key_style)])
                        .flatten()
                        .skip(1)
                        .collect::<Vec<_>>();
                    nav.push(Span::raw(": navigate | "));
                    nav.append(
                        &mut ["q", "esc"]
                            .iter()
                            .zip(std::iter::repeat(Span::styled(",", no_style)))
                            .map(|(s, sep)| [sep, Span::styled(*s, key_style)])
                            .flatten()
                            .skip(1)
                            .collect::<Vec<_>>(),
                    );
                    nav.push(Span::raw(": back/quit | "));
                    nav.push(Span::styled("/", key_style));
                    nav.push(Span::raw(": search"));
                    nav
                }
            };
            let help = text::Line::from(help);
            let help = Paragraph::new(help);
            frame.render_widget(help, chrome[1]);
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Length(self.tables.max_len() + 1), // 2 border, 1 padding
                        Constraint::Max(self.dims.width),
                    ]
                    .as_ref(),
                )
                .split(chrome[0]);
            let items: Vec<ListItem> = self
                .tables
                .names
                .iter()
                .map(|n| n.clone())
                .map(|n| ListItem::new(n).style(Style::default()))
                .collect();
            let list = List::new(items)
                .block(
                    Block::default()
                        .title("[ tables ]")
                        .title_style(tables_title_style)
                        .borders(Borders::ALL),
                )
                .highlight_style(
                    Style::default()
                        .fg(Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                );
            let state = &mut self.tables.state;
            frame.render_stateful_widget(list, chunks[0], state);
            let num_table_rows = self.num_table_rows();
            if let Some(selected_table) = &mut self.table {
                let Some((records, mut state)) = table_records else {
                    warn!("no records");
                    return;
                };
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
                let table: Table = Table::new(rows)
                    .header(header)
                    .block(
                        Block::default()
                            .title(format!(
                                "[ Table: {} ({} records) ]",
                                selected_table.name(),
                                selected_table.count
                            ))
                            .title_style(table_title_style)
                            .borders(Borders::ALL),
                    )
                    .highlight_style(Style::default().fg(Color::LightGreen))
                    .highlight_symbol("")
                    .widths(&widths);
                frame.render_stateful_widget(table, chunks[1], &mut state);
            }
        })?;
        Ok(())
    }

    fn num_table_rows(&mut self) -> usize {
        (self.dims.height - 4) as usize // 2 border, 1 header
    }

    fn open_table(&mut self) -> Result<()> {
        if let Some(name) = self.tables.selected() {
            let mut table = DbTable::new(self.dao.clone(), name, self.search.clone())?;
            if self.focus == Focus::Table {
                table.select_first();
            }
            self.table.replace(table);
        }
        Ok(())
    }

    pub fn tick(&mut self) -> Result<Tick> {
        let poll_time = Duration::from_secs(24 * 3600);
        if event::poll(poll_time).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                let start = Instant::now();
                if Self::should_quit(key) {
                    return Ok(Tick::Quit);
                }
                if self.focus == Focus::Search {
                    if let Some(action) = self.search(key) {
                        return Ok(self.process_action(action));
                    }
                } else if let Some(action) = self.bindings.matches(self.focus, key) {
                    return Ok(self.process_action(action));
                }
            }
        }
        Ok(Tick::Continue)
    }

    fn process_action(&mut self, action: Action) -> Tick {
        match action {
            Action::TablesNext => {
                if self.tables.next() {
                    self.open_table();
                }
            }
            Action::TablesPrev => {
                if self.tables.previous() {
                    self.open_table();
                }
            }
            Action::TableNext => {
                let table_rows = self.num_table_rows();
                self.table.iter_mut().for_each(DbTable::next);
            }
            Action::TablePrev => {
                let table_rows = self.num_table_rows();
                self.table.iter_mut().for_each(DbTable::previous);
            }
            Action::ChangeFocus(focus) => {
                self.focus = focus;

                match focus {
                    Focus::Tables => {
                        self.table.iter_mut().for_each(DbTable::unselect);
                    }
                    Focus::Table => {
                        self.table.iter_mut().for_each(DbTable::select_first);
                    }
                    Focus::Search => {
                        self.search = Search::default();
                    }
                }
            }
            Action::PageUp => {}
            Action::PageDown => {}
            Action::Search => {
                // User is searching live, return to search prompt
                self.open_table();
            }
            Action::Quit => return Tick::Quit,
        }

        Tick::Continue
    }

    fn search(&mut self, k: KeyEvent) -> Option<Action> {
        let kevent = |code: KeyCode, m: KeyModifiers| -> KeyEvent { KeyEvent::new(code, m) };
        let key = |code: KeyCode| -> KeyEvent { kevent(code, KeyModifiers::NONE) };

        match k.code {
            KeyCode::Esc => {
                // flush any previous results
                self.search.value = None;
                Some(Action::ChangeFocus(Focus::Table))
            }
            KeyCode::Enter => Some(Action::ChangeFocus(Focus::Table)),
            KeyCode::Backspace => {
                if let Some(mut q) = self.search.value.as_mut() {
                    q.pop();
                }
                if self.search.value.as_ref().is_some_and(|q| q == "") {
                    self.search.value = None;
                }
                // display intermediate results as they type
                Some(Action::Search)
            }
            KeyCode::Char(c) => {
                let mut new_query = self.search.value.take().unwrap_or_default();
                new_query.push(c);
                self.search.value = Some(new_query);
                // display intermediate results as they type
                Some(Action::Search)
            }
            _ => None,
        }
    }

    fn should_quit(key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return true,
            _ => false,
        }
    }
}

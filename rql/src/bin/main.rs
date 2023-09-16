use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Cell, Row, Table};
use ratatui_1::app::{App, Tick};
use std::{
    io::{self, Stdout},
    process,
    time::Duration,
};

#[derive(clap::Parser)]
struct Args {
    #[arg(long)]
    table: bool,
    path: String,
}

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

fn main() {
    if let Err(err) = setup_and_run() {
        eprintln!("{err:?}");
        process::exit(1);
    }
}

fn setup_and_run() -> Result<()> {
    let args = Args::parse();
    let mut term = setup_terminal().context("term setup failed")?;
    let res = run(&args, &mut term);
    restore_terminal(&mut term).context("term restore failed")?;
    res
}

fn run(args: &Args, term: &mut Term) -> Result<()> {
    let mut app = App::new(&args.path)?;
    loop {
        app.draw(term)?;
        match app.tick()? {
            Tick::Quit => {
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn render_table(frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let table = Table::new(vec![
        // Row can be created from simple strings.
        Row::new(vec!["Row11", "Row12", "Row13"]),
        // You can style the entire row.
        Row::new(vec!["Row21", "Row22", "Row23"]).style(Style::default().fg(Color::Blue)),
        // If you need more control over the styling you may need to create Cells directly
        Row::new(vec![
            Cell::from("Row31"),
            Cell::from("Row32").style(Style::default().fg(Color::Yellow)),
            Cell::from(Line::from(vec![
                Span::raw("Row"),
                Span::styled("33", Style::default().fg(Color::Green)),
            ])),
        ]),
        // If a Row need to display some content over multiple lines, you just have to change
        // its height.
        Row::new(vec![
            Cell::from("Row\n41"),
            Cell::from("Row\n42"),
            Cell::from("Row\n43"),
        ])
        .height(2),
    ])
    // You can set the style of the entire Table.
    .style(Style::default().fg(Color::White))
    // It has an optional header, which is simply a Row always visible at the top.
    .header(
        Row::new(vec!["Col1", "Col2", "Col3"])
            .style(Style::default().fg(Color::Yellow))
            // If you want some space between the header and the rest of the rows, you can always
            // specify some margin at the bottom.
            .bottom_margin(1),
    )
    // As any other widget, a Table can be wrapped in a Block.
    .block(Block::default().title("Table"))
    // Columns widths are constrained in the same way as Layout...
    .widths(&[
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(10),
    ])
    // ...and they can be separated by a fixed spacing.
    .column_spacing(1)
    // If you wish to highlight a row in any specific way when it is selected...
    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
    // ...and potentially show a symbol in front of the selection.
    .highlight_symbol(">>");
    frame.render_widget(table, frame.size());
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

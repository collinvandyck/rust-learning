#![allow(dead_code, unused)]

use std::{io::Stdout, sync::mpsc, time::Duration};

use crate::dao::BlockingDao;
use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub enum Tick {
    Quit,
    Nothing,
}

pub struct App {
    dao: BlockingDao,
    kill_rx: mpsc::Receiver<()>,
}

impl App {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let (tx, rx) = mpsc::sync_channel(1024);
        ctrlc::set_handler(move || {
            // catch kills
            panic!("kill");
            tx.send(()).expect("could not send kill signal");
        })?;
        let dao = BlockingDao::new(path)?;
        Ok(Self { dao, kill_rx: rx })
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
        if let Ok(_) = self.kill_rx.try_recv() {
            return Ok(Tick::Quit);
        }
        if event::poll(Duration::from_millis(250)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if KeyCode::Char('q') == key.code {
                    return Ok(Tick::Quit);
                }
            }
        }
        Ok(Tick::Nothing)
    }
}

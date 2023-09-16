#![allow(dead_code, unused)]

use std::io::Stdout;

use crate::dao::BlockingDao;
use anyhow::Result;
use ratatui::{prelude::CrosstermBackend, widgets::Paragraph};

type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;

pub struct App {
    dao: BlockingDao,
}

impl App {
    pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
        let dao = BlockingDao::new(path)?;
        Ok(Self { dao })
    }

    pub fn draw(&mut self, term: &mut Term) -> Result<()> {
        term.draw(|frame| {
            let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
            frame.render_widget(greeting, frame.size());
        })?;
        Ok(())
    }
}

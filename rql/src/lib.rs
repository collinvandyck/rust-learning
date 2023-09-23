#![allow(dead_code, unused)]
pub mod app;
pub mod dao;
mod pager;
pub mod table;
pub mod tables;
pub mod prelude {
    pub use crate::app::*;
    pub use crate::dao::*;
    pub use crate::table::*;
    pub use crate::tables::*;
    pub use anyhow::{Context, Error, Result};
    pub use clap::Parser;
    pub use crossterm::{
        event,
        event::*,
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    pub use ratatui::prelude::*;
    pub use ratatui::widgets::*;
    pub use std::time::{Duration, Instant};
    pub use std::{
        io::{self, Stdout},
        process,
    };
    pub use tracing::{debug, error, info, trace, warn};
    pub use tracing::{instrument, trace_span, Instrument};
    pub use tracing_subscriber::filter::{Directive, LevelFilter};
    pub use tracing_subscriber::EnvFilter;

    pub type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;
}

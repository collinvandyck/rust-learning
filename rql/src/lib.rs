#![allow(dead_code, unused)]
pub mod app;
pub mod dao;
mod pager;
pub mod table;
pub mod tables;
pub mod prelude {
    pub use crate::{app::*, dao::*, pager::*, table::*, tables::*};
    pub use anyhow::{Context, Error, Result};
    pub use clap::Parser;
    pub use crossterm::{
        event,
        event::*,
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    };
    pub use ratatui::{prelude::*, widgets::*};
    pub use std::{
        io::{self, Stdout},
        process,
        time::{Duration, Instant},
    };
    pub use tracing::{debug, error, info, instrument, trace, trace_span, warn, Instrument};
    pub use tracing_subscriber::{
        filter::{Directive, LevelFilter},
        EnvFilter,
    };

    pub type Term = ratatui::Terminal<CrosstermBackend<Stdout>>;
}

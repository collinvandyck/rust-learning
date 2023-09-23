use rql::prelude::*;

#[derive(clap::Parser, Debug)]
struct Args {
    #[arg(long, env, default_value = "rql.log")]
    log: String,

    #[arg(env)]
    db_path: String,
}

fn main() {
    let args = Args::parse();
    if let Err(err) = setup_and_run(&args) {
        error!("{err:?}");
        process::exit(1);
    }
}

fn init_tracing(args: &Args) -> Result<()> {
    let log_file = args.log.clone();
    let log_file = std::fs::File::create(&log_file)?;
    let mut filter = EnvFilter::default();
    for directive in &["rql=debug", "main=debug", "error"] {
        let directive: Directive = directive.parse()?;
        filter = filter.add_directive(directive);
    }
    tracing_subscriber::fmt()
        .without_time()
        .with_writer(log_file)
        .with_env_filter(filter)
        .init();
    Ok(())
}

fn setup_and_run(args: &Args) -> Result<()> {
    init_tracing(args)?;
    let mut term = setup_terminal().context("term setup failed")?;
    let res = run(&args, &mut term);
    restore_terminal(&mut term).context("term restore failed")?;
    res
}

fn run(args: &Args, term: &mut Term) -> Result<()> {
    info!(?args, "Running");
    let db: DbType = DbType::Path(args.db_path.as_str());
    let mut app = App::new(db)?;
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

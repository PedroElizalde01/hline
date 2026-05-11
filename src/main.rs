mod app;
mod clipboard;
mod favorites;
mod history;
mod sort;
mod ui;
mod update;

use anyhow::{Context, Result};
use app::App;
use clap::Parser;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use favorites::FavoritesStore;
use history::{default_history_path, load_history, HistoryFormat};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::fs::{File, OpenOptions};
use std::io::{self, IsTerminal, Stdout, Write};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(
    name = "hline",
    author,
    version,
    about = "Browse shell history in a centered-cursor TUI",
    long_about = None
)]
struct Cli {
    #[arg(long, value_name = "PATH", help = "History file to load")]
    file: Option<PathBuf>,
    #[arg(long, value_enum, default_value_t = HistoryFormat::Auto, help = "History format")]
    format: HistoryFormat,
    #[arg(
        long,
        help = "Check GitHub Releases for a newer hline version and exit"
    )]
    check_updates: bool,
    #[arg(long, help = "Skip the automatic daily update check")]
    no_update_check: bool,
}

enum TerminalWriter {
    Stdout(Stdout),
    Tty(File),
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<TerminalWriter>>,
    restored: bool,
}

impl TerminalSession {
    fn start() -> Result<Self> {
        enable_raw_mode().context("failed to enable raw mode")?;

        let mut writer = terminal_writer()?;
        writer
            .execute(EnterAlternateScreen)
            .context("failed to enter alternate screen")?;

        let backend = CrosstermBackend::new(writer);
        let mut terminal =
            Terminal::new(backend).context("failed to initialize terminal backend")?;
        terminal.clear().context("failed to clear terminal")?;

        Ok(Self {
            terminal,
            restored: false,
        })
    }

    fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<TerminalWriter>> {
        &mut self.terminal
    }

    fn restore(&mut self) -> Result<()> {
        if self.restored {
            return Ok(());
        }

        disable_raw_mode().context("failed to disable raw mode")?;

        self.terminal
            .backend_mut()
            .execute(LeaveAlternateScreen)
            .context("failed to leave alternate screen")?;

        self.terminal
            .show_cursor()
            .context("failed to show cursor")?;

        self.restored = true;
        Ok(())
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        if self.restored {
            return;
        }

        let _ = disable_raw_mode();
        let _ = self.terminal.backend_mut().execute(LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
        self.restored = true;
    }
}

impl Write for TerminalWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(stdout) => stdout.write(buf),
            Self::Tty(tty) => tty.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(stdout) => stdout.flush(),
            Self::Tty(tty) => tty.flush(),
        }
    }
}

fn terminal_writer() -> Result<TerminalWriter> {
    if io::stdout().is_terminal() {
        Ok(TerminalWriter::Stdout(io::stdout()))
    } else {
        let tty = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .context("failed to open /dev/tty for interactive output")?;
        Ok(TerminalWriter::Tty(tty))
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.check_updates {
        update::print_update_check()?;
        return Ok(());
    }

    let path = cli.file.unwrap_or_else(|| default_history_path(cli.format));
    let entries = load_history(&path, cli.format)
        .with_context(|| format!("failed loading history from {}", path.display()))?;
    let favorites = FavoritesStore::load_default().context("failed loading favorites")?;

    if let Some(output) = run_tui(App::with_favorites(entries, favorites))? {
        println!("{output}");
    }

    if !cli.no_update_check {
        update::maybe_print_update_notice();
    }

    Ok(())
}

fn run_tui(mut app: App) -> Result<Option<String>> {
    let mut session = TerminalSession::start()?;

    let run_result = run_app(session.terminal_mut(), &mut app);
    let restore_result = session.restore();

    match (run_result, restore_result) {
        (Ok(_), Ok(_)) => Ok(app.take_accepted_output()),
        (Err(run_err), Ok(_)) => Err(run_err),
        (Ok(_), Err(restore_err)) => Err(restore_err),
        (Err(run_err), Err(restore_err)) => Err(anyhow::anyhow!(
            "app error: {run_err}; restore error: {restore_err}"
        )),
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<TerminalWriter>>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(100);

    loop {
        terminal.draw(|frame| ui::draw(frame, app))?;

        if app.should_quit {
            break;
        }

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    app.handle_key(key);
                }
            }
        }

        app.tick();
    }

    Ok(())
}

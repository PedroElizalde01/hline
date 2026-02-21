mod app;
mod clipboard;
mod history;
mod sort;
mod ui;

use anyhow::{Context, Result};
use app::App;
use clap::Parser;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use history::{default_history_path, load_history};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(
    name = "hline",
    author,
    version,
    about = "Browse bash history in a centered-cursor TUI",
    long_about = None
)]
struct Cli {
    #[arg(long, value_name = "PATH")]
    file: Option<PathBuf>,
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    restored: bool,
}

impl TerminalSession {
    fn start() -> Result<Self> {
        enable_raw_mode().context("failed to enable raw mode")?;

        let mut stdout = io::stdout();
        stdout
            .execute(EnterAlternateScreen)
            .context("failed to enter alternate screen")?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal =
            Terminal::new(backend).context("failed to initialize terminal backend")?;
        terminal.clear().context("failed to clear terminal")?;

        Ok(Self {
            terminal,
            restored: false,
        })
    }

    fn terminal_mut(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    let path = cli.file.unwrap_or_else(default_history_path);
    let entries = load_history(&path)
        .with_context(|| format!("failed loading history from {}", path.display()))?;

    run_tui(App::new(entries))
}

fn run_tui(mut app: App) -> Result<()> {
    let mut session = TerminalSession::start()?;

    let run_result = run_app(session.terminal_mut(), &mut app);
    let restore_result = session.restore();

    match (run_result, restore_result) {
        (Ok(_), Ok(_)) => Ok(()),
        (Err(run_err), Ok(_)) => Err(run_err),
        (Ok(_), Err(restore_err)) => Err(restore_err),
        (Err(run_err), Err(restore_err)) => Err(anyhow::anyhow!(
            "app error: {run_err}; restore error: {restore_err}"
        )),
    }
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> Result<()> {
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

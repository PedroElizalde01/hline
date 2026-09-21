mod app;
mod clipboard;
mod favorites;
mod history;
mod settings;
mod sort;
mod ui;
mod update;

use anyhow::{bail, Context, Result};
use app::App;
use clap::{Parser, Subcommand};
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
    long_about = None,
    after_help = "\
Setup:
  eval \"$(hline init bash)\"        add to ~/.bashrc
  eval \"$(hline init zsh)\"         add to ~/.zshrc
  hline init fish | source         add to ~/.config/fish/config.fish
  Then press Ctrl+R (change with shell_key in `hline --settings`).

Aliases:
  Favorites are titled favN by default, rename with `r` in the favorites view.
  hline fav1                       print favorite to stdout and copy it
  hr() { eval \"$(hline \"$1\")\"; }   run a favorite
  Press ? inside the TUI for keybindings."
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
    #[arg(long, help = "Print the settings file path and contents, creating defaults if missing")]
    settings: bool,
    #[arg(long, help = "List favorites and their commands")]
    list: bool,
    #[arg(long, hide = true)]
    clipboard_wait: bool,
    #[arg(
        long,
        visible_alias = "behavior",
        value_name = "MODE",
        num_args = 0..=1,
        help = "What an alias does this run: print, copy, or full. Without a value, show the current mode"
    )]
    behaviour: Option<Option<settings::Behaviour>>,
    #[arg(
        value_name = "ALIAS",
        help = "Print the favorite with this title to stdout and copy it to the clipboard"
    )]
    alias: Option<String>,
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Print the shell widget snippet for `eval "$(hline init <shell>)"`
    Init {
        #[arg(value_name = "SHELL", help = "bash, zsh or fish")]
        shell: String,
    },
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

    if cli.settings {
        return settings::print_settings();
    }

    #[cfg(target_os = "linux")]
    if cli.clipboard_wait {
        return clipboard::hold_clipboard_from_stdin();
    }

    if let Some(Command::Init { shell }) = cli.command {
        print!("{}", settings::init_snippet(&shell, &settings::load()?)?);
        return Ok(());
    }

    if cli.alias.is_none() && !cli.list {
        match cli.behaviour {
            // `--behaviour` with no value is a question, not an override.
            Some(None) => return settings::print_behaviour(),
            // `--behaviour MODE` with no alias sets the global default.
            Some(Some(behaviour)) => return settings::set_behaviour(behaviour),
            None => {}
        }
    }

    if cli.list || cli.alias.is_some() {
        return run_alias(cli.alias.as_deref(), cli.behaviour.flatten());
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

fn run_alias(name: Option<&str>, requested: Option<settings::Behaviour>) -> Result<()> {
    let mut favorites = FavoritesStore::load_default().context("failed loading favorites")?;
    let Some(name) = name else {
        for block in &favorites.blocks {
            match block.behaviour {
                Some(behaviour) => println!("{} [{}]", block.display_title(), behaviour.name()),
                None => println!("{}", block.display_title()),
            }
            for line in &block.lines {
                println!("  {line}");
            }
        }
        return Ok(());
    };

    match favorites.find_by_alias(name) {
        Ok(index) => {
            // A mode passed with an alias sticks to that favorite.
            if let Some(behaviour) = requested {
                if favorites.set_behaviour(index, behaviour)? {
                    eprintln!(
                        "{} now defaults to {}",
                        favorites.blocks[index].display_title(),
                        behaviour.name()
                    );
                }
            }

            let block = &favorites.blocks[index];
            let behaviour = match block.behaviour {
                Some(behaviour) => behaviour,
                None => settings::load()?.alias_behaviour,
            };
            let text = block.lines.join("\n");
            // Headers on stderr so `eval "$(hline name)"` only sees the commands.
            if behaviour.copies() {
                match clipboard::copy_and_detach(&text) {
                    Ok(()) => eprintln!("Copied to clipboard:"),
                    Err(err) => eprintln!("hline: clipboard copy failed: {err:#}"),
                }
            }
            if behaviour.runs() {
                // Preview on stderr so stdout carries only the commands' own output.
                eprintln!("{text}");
                return run_commands(&text);
            }
            println!("{text}");
            Ok(())
        }
        Err(candidates) if candidates.is_empty() => bail!("no favorite named {name:?}"),
        Err(candidates) => bail!(
            "ambiguous favorite {name:?}, matches: {}",
            candidates.join(", ")
        ),
    }
}

/// Run the block in a child shell. `cd` and exports do not outlive it, so a
/// shell function around `hline` stays the way to change the calling shell.
fn run_commands(text: &str) -> Result<()> {
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    eprintln!("Running:");
    
    let status = std::process::Command::new(&shell)
        .arg("-c")
        .arg(text)
        .status()
        .with_context(|| format!("failed to run commands with {shell}"))?;

    if status.success() {
        Ok(())
    } else {
        std::process::exit(status.code().unwrap_or(1));
    }
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

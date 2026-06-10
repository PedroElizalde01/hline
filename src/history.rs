use anyhow::{Context, Result};
use clap::ValueEnum;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum HistoryFormat {
    Auto,
    Bash,
    Zsh,
    Fish,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: u64,
    pub cmd: String,
    pub timestamp: Option<i64>,
    pub duration_secs: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedEntry {
    cmd: String,
    timestamp: Option<i64>,
    duration_secs: Option<i64>,
}

impl HistoryFormat {
    pub fn default_history_path(self) -> PathBuf {
        match self {
            Self::Auto => default_history_path_auto(),
            Self::Bash => bash_history_path(),
            Self::Zsh => zsh_history_path(),
            Self::Fish => fish_history_path(),
        }
    }
}

pub fn default_history_path(format: HistoryFormat) -> PathBuf {
    format.default_history_path()
}

pub fn load_history(path: &Path, format: HistoryFormat) -> Result<Vec<Entry>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read history file: {}", path.display()))?;

    let resolved_format = match format {
        HistoryFormat::Auto => detect_history_format(path, &content),
        explicit => explicit,
    };

    let parsed = match resolved_format {
        HistoryFormat::Auto => unreachable!("history format auto must be resolved"),
        HistoryFormat::Bash => parse_bash_history(&content),
        HistoryFormat::Zsh => parse_zsh_history(&content),
        HistoryFormat::Fish => parse_fish_history(&content),
    };

    Ok(parsed
        .into_iter()
        .rev()
        .enumerate()
        .map(|(id, entry)| Entry {
            id: id as u64,
            cmd: entry.cmd,
            timestamp: entry.timestamp,
            duration_secs: entry.duration_secs,
        })
        .collect())
}

fn default_history_path_auto() -> PathBuf {
    match std::env::var("SHELL")
        .ok()
        .as_deref()
        .and_then(shell_name_from_path)
    {
        Some("zsh") => zsh_history_path(),
        Some("fish") => fish_history_path(),
        Some("bash") => bash_history_path(),
        _ => first_existing_history_path().unwrap_or_else(bash_history_path),
    }
}

fn first_existing_history_path() -> Option<PathBuf> {
    [
        bash_history_path(),
        zsh_history_path(),
        fish_history_path(),
        legacy_fish_history_path(),
    ]
    .into_iter()
    .find(|path| path.exists())
}

fn bash_history_path() -> PathBuf {
    home_dir().join(".bash_history")
}

fn zsh_history_path() -> PathBuf {
    home_dir().join(".zsh_history")
}

fn fish_history_path() -> PathBuf {
    let primary = if let Ok(xdg_data_home) = std::env::var("XDG_DATA_HOME") {
        PathBuf::from(xdg_data_home)
            .join("fish")
            .join("fish_history")
    } else {
        home_dir()
            .join(".local")
            .join("share")
            .join("fish")
            .join("fish_history")
    };

    let legacy = legacy_fish_history_path();
    if primary.exists() || !legacy.exists() {
        primary
    } else {
        legacy
    }
}

fn legacy_fish_history_path() -> PathBuf {
    home_dir().join(".config").join("fish").join("fish_history")
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."))
}

fn shell_name_from_path(path: &str) -> Option<&str> {
    Path::new(path).file_name()?.to_str()
}

fn detect_history_format(path: &Path, content: &str) -> HistoryFormat {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    if file_name == ".zsh_history" {
        return HistoryFormat::Zsh;
    }
    if file_name == "fish_history" {
        return HistoryFormat::Fish;
    }
    if file_name == ".bash_history" {
        return HistoryFormat::Bash;
    }

    for line in content.lines().map(|line| line.trim_end_matches('\r')) {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("- cmd: ") {
            return HistoryFormat::Fish;
        }
        if parse_zsh_extended_history_line(line).is_some() {
            return HistoryFormat::Zsh;
        }
        if parse_bash_timestamp_line(line).is_some() {
            return HistoryFormat::Bash;
        }
        break;
    }

    HistoryFormat::Bash
}

fn parse_bash_history(content: &str) -> Vec<ParsedEntry> {
    let mut entries = Vec::new();
    let mut pending_timestamp = None;

    for line in content.lines() {
        let trimmed = line.trim_end_matches('\r');
        if let Some(timestamp) = parse_bash_timestamp_line(trimmed) {
            pending_timestamp = Some(timestamp);
            continue;
        }
        if !trimmed.is_empty() {
            entries.push(ParsedEntry {
                cmd: trimmed.to_owned(),
                timestamp: pending_timestamp.take(),
                duration_secs: None,
            });
        }
    }

    entries
}

fn parse_zsh_history(content: &str) -> Vec<ParsedEntry> {
    let mut entries = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_end_matches('\r');
        if trimmed.is_empty() {
            continue;
        }

        let (cmd, timestamp, duration_secs) = if let Some((timestamp, duration_secs, cmd)) =
            parse_zsh_extended_history_line(trimmed)
        {
            (cmd.to_owned(), Some(timestamp), Some(duration_secs))
        } else {
            (trimmed.to_owned(), None, None)
        };

        entries.push(ParsedEntry {
            cmd,
            timestamp,
            duration_secs,
        });
    }

    entries
}

fn parse_fish_history(content: &str) -> Vec<ParsedEntry> {
    let mut entries = Vec::new();
    let mut current_cmd: Option<String> = None;
    let mut current_timestamp = None;

    for line in content.lines() {
        let trimmed = line.trim_end_matches('\r');
        if let Some(cmd) = trimmed.strip_prefix("- cmd: ") {
            flush_fish_entry(&mut entries, &mut current_cmd, &mut current_timestamp);
            current_cmd = Some(cmd.to_owned());
            current_timestamp = None;
            continue;
        }

        if let Some(timestamp) = trimmed
            .strip_prefix("  when: ")
            .and_then(|value| value.parse::<i64>().ok())
        {
            current_timestamp = Some(timestamp);
        }
    }

    flush_fish_entry(&mut entries, &mut current_cmd, &mut current_timestamp);
    entries
}

fn flush_fish_entry(
    entries: &mut Vec<ParsedEntry>,
    current_cmd: &mut Option<String>,
    current_timestamp: &mut Option<i64>,
) {
    if let Some(cmd) = current_cmd.take() {
        entries.push(ParsedEntry {
            cmd,
            timestamp: current_timestamp.take(),
            duration_secs: None,
        });
    }
}

fn parse_bash_timestamp_line(line: &str) -> Option<i64> {
    let rest = line.strip_prefix('#')?;
    if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    rest.parse::<i64>().ok()
}

fn parse_zsh_extended_history_line(line: &str) -> Option<(i64, i64, &str)> {
    let rest = line.strip_prefix(": ")?;
    let (timestamp, rest) = rest.split_once(':')?;
    let (duration, cmd) = rest.split_once(';')?;

    Some((
        timestamp.parse::<i64>().ok()?,
        duration.parse::<i64>().ok()?,
        cmd,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn load_bash_history_ignores_empty_and_trims_crlf() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(file.path(), "echo one\r\n\n  \r\nls -la\r\n\npwd\n").expect("write history");

        let entries = load_history(file.path(), HistoryFormat::Bash).expect("load history");

        let cmds: Vec<&str> = entries.iter().map(|e| e.cmd.as_str()).collect();
        assert_eq!(cmds, vec!["pwd", "ls -la", "  ", "echo one"]);
        assert_eq!(entries[0].id, 0);
        assert_eq!(entries[1].id, 1);
        assert!(entries.iter().all(|entry| entry.timestamp.is_none()));
    }

    #[test]
    fn load_bash_history_preserves_timestamps() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(
            file.path(),
            "#1700000000\nls\n#1700000123\ngit status\npwd\n",
        )
        .expect("write history");

        let entries = load_history(file.path(), HistoryFormat::Bash).expect("load history");

        let cmds: Vec<&str> = entries.iter().map(|e| e.cmd.as_str()).collect();
        assert_eq!(cmds, vec!["pwd", "git status", "ls"]);
        assert_eq!(entries[0].timestamp, None);
        assert_eq!(entries[1].timestamp, Some(1_700_000_123));
        assert_eq!(entries[2].timestamp, Some(1_700_000_000));
    }

    #[test]
    fn load_zsh_history_parses_extended_metadata() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(
            file.path(),
            ": 1700000000:12;git status\nplain command\n: 1700000100:0;cargo test\n",
        )
        .expect("write history");

        let entries = load_history(file.path(), HistoryFormat::Zsh).expect("load history");

        assert_eq!(entries[0].cmd, "cargo test");
        assert_eq!(entries[0].timestamp, Some(1_700_000_100));
        assert_eq!(entries[0].duration_secs, Some(0));
        assert_eq!(entries[1].cmd, "plain command");
        assert_eq!(entries[1].timestamp, None);
        assert_eq!(entries[2].cmd, "git status");
        assert_eq!(entries[2].duration_secs, Some(12));
    }

    #[test]
    fn load_fish_history_parses_cmd_and_when() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(
            file.path(),
            "- cmd: git status\n  when: 1700000000\n- cmd: cargo test\n  when: 1700000500\n",
        )
        .expect("write history");

        let entries = load_history(file.path(), HistoryFormat::Fish).expect("load history");

        assert_eq!(entries[0].cmd, "cargo test");
        assert_eq!(entries[0].timestamp, Some(1_700_000_500));
        assert_eq!(entries[1].cmd, "git status");
        assert_eq!(entries[1].timestamp, Some(1_700_000_000));
    }

    #[test]
    fn auto_detection_uses_content_and_filename() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(file.path(), ": 1700000000:0;git status\n").expect("write history");

        let entries = load_history(file.path(), HistoryFormat::Auto).expect("load history");

        assert_eq!(entries[0].timestamp, Some(1_700_000_000));
        assert_eq!(
            detect_history_format(Path::new(".zsh_history"), ""),
            HistoryFormat::Zsh
        );
        assert_eq!(
            detect_history_format(Path::new("fish_history"), ""),
            HistoryFormat::Fish
        );
    }

    #[test]
    fn default_history_path_points_to_requested_shell_history() {
        let path = default_history_path(HistoryFormat::Bash);
        assert!(path.ends_with(".bash_history"));
        assert!(default_history_path(HistoryFormat::Zsh).ends_with(".zsh_history"));
        assert!(default_history_path(HistoryFormat::Fish).ends_with("fish_history"));
    }
}

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub id: u64,
    pub cmd: String,
}

pub fn default_history_path() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home).join(".bash_history");
    }
    PathBuf::from(".bash_history")
}

pub fn load_history(path: &Path) -> Result<Vec<Entry>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read history file: {}", path.display()))?;

    let mut lines: Vec<String> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim_end_matches('\r');
        if parse_bash_timestamp_line(trimmed).is_some() {
            continue;
        }
        if !trimmed.is_empty() {
            lines.push(trimmed.to_owned());
        }
    }

    let mut entries = Vec::with_capacity(lines.len());
    for (id, cmd) in lines.into_iter().rev().enumerate() {
        entries.push(Entry { id: id as u64, cmd });
    }

    Ok(entries)
}

fn parse_bash_timestamp_line(line: &str) -> Option<i64> {
    let rest = line.strip_prefix('#')?;
    if rest.is_empty() || !rest.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    rest.parse::<i64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn load_history_ignores_empty_and_trims_crlf() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(file.path(), "echo one\r\n\n  \r\nls -la\r\n\npwd\n").expect("write history");

        let entries = load_history(file.path()).expect("load history");

        let cmds: Vec<&str> = entries.iter().map(|e| e.cmd.as_str()).collect();
        assert_eq!(cmds, vec!["pwd", "ls -la", "  ", "echo one"]);
        assert_eq!(entries[0].id, 0);
        assert_eq!(entries[1].id, 1);
    }

    #[test]
    fn load_history_ignores_bash_timestamps() {
        let file = NamedTempFile::new().expect("temp file");
        fs::write(
            file.path(),
            "#1700000000\nls\n#1700000123\ngit status\npwd\n",
        )
        .expect("write history");

        let entries = load_history(file.path()).expect("load history");

        let cmds: Vec<&str> = entries.iter().map(|e| e.cmd.as_str()).collect();
        assert_eq!(cmds, vec!["pwd", "git status", "ls"]);
    }

    #[test]
    fn default_history_path_points_to_bash_history() {
        let path = default_history_path();
        assert!(path.ends_with(".bash_history"));
    }
}

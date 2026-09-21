use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::favorites::config_dir;

/// What `hline <alias>` does with the favorite block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
#[clap(rename_all = "lowercase")]
pub enum Behaviour {
    /// Print to stdout only. Safe for `eval "$(hline name)"`.
    Print,
    /// Copy to the clipboard and print.
    Copy,
    /// Copy, print, and run the commands.
    #[serde(alias = "run")]
    #[clap(alias = "run")]
    Full,
}

impl Behaviour {
    pub fn copies(self) -> bool {
        matches!(self, Self::Copy | Self::Full)
    }

    pub fn runs(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    /// Shell widget key: "ctrl-<letter>" or "alt-<letter>".
    pub shell_key: String,
    /// What `hline <alias>` does: print, copy, or full.
    #[serde(default = "default_behaviour", alias = "alias_behavior")]
    pub alias_behaviour: Behaviour,
}

fn default_behaviour() -> Behaviour {
    Behaviour::Copy
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            shell_key: "ctrl-r".to_string(),
            alias_behaviour: default_behaviour(),
        }
    }
}

pub fn settings_path() -> PathBuf {
    config_dir().join("settings.json")
}

pub fn load() -> Result<Settings> {
    let path = settings_path();
    if !path.exists() {
        return Ok(Settings::default());
    }
    let content = fs::read_to_string(&path)
        .with_context(|| format!("failed to read settings file: {}", path.display()))?;
    serde_json::from_str(&content)
        .with_context(|| format!("failed to parse settings file: {}", path.display()))
}

/// Writes defaults if missing, then prints path and contents.
pub fn print_settings() -> Result<()> {
    let path = settings_path();
    if !path.exists() {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, serde_json::to_string_pretty(&Settings::default())?)
            .with_context(|| format!("failed to write settings file: {}", path.display()))?;
    }
    let settings = load()?;
    println!("{}", path.display());
    println!("{}", serde_json::to_string_pretty(&settings)?);
    println!();
    println!("shell_key: ctrl-<letter> or alt-<letter>, used by `hline init <shell>`");
    println!("alias_behaviour: print (stdout only), copy (clipboard + stdout), full (also runs)");
    println!("                 override once with `hline <alias> --behaviour full`");
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Key {
    Ctrl(char),
    Alt(char),
}

// ponytail: ctrl/alt + single letter only, extend parser if chords ever needed
fn parse_key(spec: &str) -> Result<Key> {
    let spec = spec.trim().to_lowercase();
    let (modifier, rest) = spec
        .split_once(['-', '+'])
        .ok_or_else(|| anyhow::anyhow!("invalid shell_key {spec:?}, expected ctrl-r or alt-r"))?;
    let mut chars = rest.chars();
    let (Some(c), None) = (chars.next(), chars.next()) else {
        bail!("invalid shell_key {spec:?}, expected a single letter after the modifier");
    };
    if !c.is_ascii_alphabetic() {
        bail!("invalid shell_key {spec:?}, expected an ascii letter");
    }
    match modifier {
        "ctrl" | "c" | "control" => Ok(Key::Ctrl(c)),
        "alt" | "a" | "m" | "meta" => Ok(Key::Alt(c)),
        _ => bail!("invalid shell_key {spec:?}, modifier must be ctrl or alt"),
    }
}

pub fn init_snippet(shell: &str, settings: &Settings) -> Result<String> {
    let key = parse_key(&settings.shell_key)?;
    let snippet = match shell {
        "bash" => {
            let bind = match key {
                Key::Ctrl(c) => format!("\\C-{c}"),
                Key::Alt(c) => format!("\\e{c}"),
            };
            format!(
                r#"hline-widget() {{
  local cmd
  cmd="$(hline)" || return
  [[ -n "$cmd" ]] || return
  READLINE_LINE="$cmd"
  READLINE_POINT=${{#READLINE_LINE}}
}}
bind -x '"{bind}":hline-widget'
"#
            )
        }
        "zsh" => {
            let bind = match key {
                Key::Ctrl(c) => format!("^{}", c.to_ascii_uppercase()),
                Key::Alt(c) => format!("^[{c}"),
            };
            format!(
                r#"hline-widget() {{
  local cmd
  cmd="$(hline)" || return
  [[ -n "$cmd" ]] || return
  BUFFER="$cmd"
  CURSOR=${{#BUFFER}}
  zle reset-prompt
}}
zle -N hline-widget
bindkey '{bind}' hline-widget
"#
            )
        }
        "fish" => {
            let bind = match key {
                Key::Ctrl(c) => format!("\\c{c}"),
                Key::Alt(c) => format!("\\e{c}"),
            };
            format!(
                r#"function hline-widget
    set cmd (hline)
    or return
    test -n "$cmd"; or return
    commandline -r -- $cmd
end
bind {bind} hline-widget
"#
            )
        }
        other => bail!("unsupported shell {other:?}, expected bash, zsh or fish"),
    };
    Ok(snippet)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn behaviour_controls_copy_and_run() {
        assert!(!Behaviour::Print.copies() && !Behaviour::Print.runs());
        assert!(Behaviour::Copy.copies() && !Behaviour::Copy.runs());
        assert!(Behaviour::Full.copies() && Behaviour::Full.runs());
    }

    #[test]
    fn settings_default_behaviour_and_old_files_without_one() {
        assert_eq!(Settings::default().alias_behaviour, Behaviour::Copy);

        let old: Settings = serde_json::from_str(r#"{"shell_key":"alt-h"}"#).expect("old file");
        assert_eq!(old.alias_behaviour, Behaviour::Copy);

        let run: Settings = serde_json::from_str(r#"{"shell_key":"ctrl-r","alias_behaviour":"run"}"#)
            .expect("run alias");
        assert_eq!(run.alias_behaviour, Behaviour::Full);

        let us: Settings = serde_json::from_str(r#"{"shell_key":"ctrl-r","alias_behavior":"print"}"#)
            .expect("american spelling");
        assert_eq!(us.alias_behaviour, Behaviour::Print);
    }

    #[test]
    fn parses_key_specs() {
        assert_eq!(parse_key("ctrl-r").unwrap(), Key::Ctrl('r'));
        assert_eq!(parse_key("Alt+F").unwrap(), Key::Alt('f'));
        assert!(parse_key("r").is_err());
        assert!(parse_key("ctrl-rr").is_err());
        assert!(parse_key("shift-r").is_err());
    }

    #[test]
    fn snippets_use_configured_key() {
        let settings = Settings {
            shell_key: "alt-f".to_string(),
            ..Settings::default()
        };
        assert!(init_snippet("bash", &settings).unwrap().contains(r#"'"\ef":hline-widget'"#));
        assert!(init_snippet("zsh", &settings).unwrap().contains("bindkey '^[f'"));
        assert!(init_snippet("fish", &settings).unwrap().contains("bind \\ef hline-widget"));
        assert!(init_snippet("zsh", &Settings::default()).unwrap().contains("bindkey '^R'"));
        assert!(init_snippet("nu", &settings).is_err());
    }
}

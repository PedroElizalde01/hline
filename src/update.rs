use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const LATEST_RELEASE_URL: &str =
    "https://api.github.com/repos/PedroElizalde01/hline/releases/latest";
const INSTALL_COMMAND: &str =
    "curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | bash";
const CHECK_INTERVAL_SECONDS: u64 = 24 * 60 * 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateCache {
    checked_at: u64,
    latest_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ReleaseResponse {
    tag_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UpdateInfo {
    current_version: String,
    latest_tag: String,
}

pub fn print_update_check() -> Result<()> {
    match check_for_update(true).context("failed to check for updates")? {
        Some(update) => {
            eprintln!(
                "hline {} is available. You have {}.",
                update.latest_tag, update.current_version
            );
            eprintln!("Update: {INSTALL_COMMAND}");
        }
        None => {
            eprintln!("hline {} is up to date.", current_version());
        }
    }

    Ok(())
}

pub fn maybe_print_update_notice() {
    let Ok(Some(update)) = check_for_update(false) else {
        return;
    };

    eprintln!();
    eprintln!(
        "hline {} is available. You have {}.",
        update.latest_tag, update.current_version
    );
    eprintln!("Update: {INSTALL_COMMAND}");
}

fn check_for_update(force_fetch: bool) -> Result<Option<UpdateInfo>> {
    let cache_path = default_update_cache_path();
    let cache = if force_fetch {
        Some(fetch_and_cache_latest(&cache_path, false)?)
    } else {
        read_fresh_cache(&cache_path).or_else(|| fetch_and_cache_latest(&cache_path, true).ok())
    };

    let Some(latest_tag) = cache.and_then(|cache| cache.latest_tag) else {
        return Ok(None);
    };

    let current = current_version();
    if compare_versions(&latest_tag, &current) == Ordering::Greater {
        return Ok(Some(UpdateInfo {
            current_version: current,
            latest_tag,
        }));
    }

    Ok(None)
}

fn fetch_and_cache_latest(path: &Path, tolerate_failure: bool) -> Result<UpdateCache> {
    let output = Command::new("curl")
        .args([
            "-fsSL",
            "--max-time",
            "2",
            "-H",
            "User-Agent: hline-update-check",
            LATEST_RELEASE_URL,
        ])
        .output()
        .context("failed to run curl")?;

    let latest_tag = match output.status.success() {
        true => {
            let release: ReleaseResponse = serde_json::from_slice(&output.stdout)
                .context("failed to parse release response")?;
            Some(release.tag_name)
        }
        false if tolerate_failure => None,
        false => bail!("GitHub release request failed with {}", output.status),
    };

    let cache = UpdateCache {
        checked_at: unix_now(),
        latest_tag,
    };
    write_cache(path, &cache)?;
    Ok(cache)
}

fn read_fresh_cache(path: &Path) -> Option<UpdateCache> {
    let content = fs::read_to_string(path).ok()?;
    let cache: UpdateCache = serde_json::from_str(&content).ok()?;
    let age = unix_now().saturating_sub(cache.checked_at);

    if age <= CHECK_INTERVAL_SECONDS {
        Some(cache)
    } else {
        None
    }
}

fn write_cache(path: &Path, cache: &UpdateCache) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "failed to create update cache directory: {}",
                parent.display()
            )
        })?;
    }

    let json = serde_json::to_string_pretty(cache).context("failed to encode update cache")?;
    fs::write(path, json)
        .with_context(|| format!("failed to write update cache: {}", path.display()))
}

fn current_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

fn compare_versions(left: &str, right: &str) -> Ordering {
    let left_parts = version_parts(left);
    let right_parts = version_parts(right);
    left_parts.cmp(&right_parts)
}

fn version_parts(version: &str) -> Vec<u64> {
    version
        .trim_start_matches('v')
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<u64>().unwrap_or(0))
        .collect()
}

fn default_update_cache_path() -> PathBuf {
    if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
        return PathBuf::from(cache_home).join("hline").join("update.json");
    }

    if let Ok(home) = std::env::var("HOME") {
        return PathBuf::from(home)
            .join(".cache")
            .join("hline")
            .join("update.json");
    }

    Path::new(".").join(".hline-update.json")
}

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_versions_without_v_prefix() {
        assert_eq!(compare_versions("v0.1.6", "v0.1.5"), Ordering::Greater);
        assert_eq!(compare_versions("0.1.5", "v0.1.5"), Ordering::Equal);
        assert_eq!(compare_versions("v0.1.4", "v0.1.5"), Ordering::Less);
    }

    #[test]
    fn uses_xdg_cache_home_when_present() {
        std::env::set_var("XDG_CACHE_HOME", "/tmp/hline-cache-test");
        assert_eq!(
            default_update_cache_path(),
            PathBuf::from("/tmp/hline-cache-test/hline/update.json")
        );
        std::env::remove_var("XDG_CACHE_HOME");
    }
}

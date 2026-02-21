# hline

`hline` is a Rust terminal UI for browsing bash history with a centered cursor, live filtering, sorting, multi-select, and clipboard copy.

Linux is the current primary target (`~/.bash_history`).

## Features

- Reads bash history from `~/.bash_history` by default (`--file PATH` supported)
- Centered-cursor list navigation for high-volume history
- Live search (`/`) with immediate filter updates
- Sorting modes: recency, alphabetical, length (`s` / `S`)
- Multi-select + copy (`y`) via `arboard`
- Selection stability across filters/sorts (entry IDs, not row indexes)
- Built-in help view (`?`)

## Requirements

- Rust toolchain (for building from source)
- Linux terminal environment

## Build and run

```bash
cargo run
```

Custom history file:

```bash
cargo run -- --file /path/to/history
```

Help/version:

```bash
cargo run -- --help
cargo run -- --version
```

## Planned release install flow

After GitHub Releases are configured:

```bash
curl -fsSL https://raw.githubusercontent.com/<PedroElizalde01>/hline/main/install.sh | bash
```

Override defaults:

```bash
curl -fsSL https://raw.githubusercontent.com/<PedroElizalde01>/hline/main/install.sh | \
  bash -s -- --repo <PedroElizalde01>/hline --version v0.1.0 --bin-dir ~/.local/bin
```

## Local release packaging

Build artifact + checksum:

```bash
./scripts/release_local.sh
```

Example output artifact naming:

- `dist/hline-x86_64-unknown-linux-gnu.tar.gz`
- `dist/hline-x86_64-unknown-linux-gnu.tar.gz.sha256`

## Keybindings

- `j` / `Down`: move down
- `k` / `Up`: move up
- `Ctrl+d` / `PageDown`: half-page down
- `Ctrl+u` / `PageUp`: half-page up
- `g`: jump to top
- `G`: jump to bottom
- `Space`: toggle current selection
- `a`: select all shown entries
- `c`: clear selection
- `y`: copy selected (or current when none selected)
- `/`: search mode
- `Enter`: confirm search
- `Esc`: leave search/help
- `Ctrl+w` or `Ctrl+Backspace` (search): delete last word
- `Ctrl+u` (search): clear query
- `s`: cycle sort mode
- `S`: reverse sort direction
- `?`: toggle help
- `q`: quit

## Config conventions (reserved)

`hline` currently works without a config file. If configuration is added later, prefer:

- `$XDG_CONFIG_HOME/hline/config.toml` (or `~/.config/hline/config.toml`)
- `$XDG_STATE_HOME/hline/` for state/cache-like data

## License

MIT. See `LICENSE`.

## Next step after repository exists

GitHub Actions release automation is intentionally not included yet; it will be added after the repository is created.

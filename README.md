<div align="center">
<pre>
██╗  ██╗██╗     ██╗███╗   ██╗███████╗
██║  ██║██║     ██║████╗  ██║██╔════╝
███████║██║     ██║██╔██╗ ██║█████╗
██╔══██║██║     ██║██║╚██╗██║██╔══╝
██║  ██║███████╗██║██║ ╚████║███████╗
╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝
</pre>

Bash history TUI for Linux
</div>

# hline

`hline` is a Rust TUI to browse Bash history (`~/.bash_history`) with fast navigation, live filtering, sorting, multi-select, and clipboard copy.

## Features

- Centered-cursor scrolling for large history files
- Live search/filter mode (`/`)
- Sort modes: recency, alphabetical, length (`s` / `S`)
- Multi-select and copy (`Space`, `a`, `c`, `y`)
- Custom history path with `--file PATH`
- Linux-first target

## Installation

### From GitHub Releases (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | bash
```

### Install specific version

```bash
curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | \
  bash -s -- --repo PedroElizalde01/hline --version v0.1.0 --bin-dir ~/.local/bin
```

## Build from source

```bash
cargo build --release
./target/release/hline --help
./target/release/hline --version
```

Run directly:

```bash
cargo run
cargo run -- --file /path/to/history
```

## Keybindings

- `j` / `Down`: move down
- `k` / `Up`: move up
- `Ctrl+d` / `PageDown`: half-page down
- `Ctrl+u` / `PageUp`: half-page up
- `g` / `G`: jump top / bottom
- `Space`: toggle selection
- `a`: select all shown
- `c`: clear selection
- `y`: copy selected (or current if none selected)
- `/`: enter search mode
- `Enter`: confirm search
- `Esc`: exit search/help
- `s`: cycle sort mode
- `S`: reverse sort direction
- `?`: help
- `q`: quit

## Release artifacts

Naming format:

- `hline-<arch>-<target>.tar.gz`
- `hline-<arch>-<target>.tar.gz.sha256`

Example:

- `hline-x86_64-unknown-linux-gnu.tar.gz`

## Local release build

```bash
./scripts/release_local.sh
ls -lah dist
```

## Quality checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

## License

MIT (`LICENSE`).

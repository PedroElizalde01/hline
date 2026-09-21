<div align="center">
<pre>
  ██╗  ██╗██╗     ██╗███╗   ██╗███████╗
  ██║  ██║██║     ██║████╗  ██║██╔════╝
███████║██║     ██║██╔██╗ ██║█████╗
██╔══██║██║     ██║██║╚██╗██║██╔══╝
  ██║  ██║███████╗██║██║ ╚████║███████╗
  ╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝
</pre>

Shell history TUI
</div>

---

**Shell history TUI for Bash, Zsh, and Fish on Linux and macOS.** A Ctrl+R replacement written in Rust.

Website: https://hline.vercel.app · Docs: https://hline.vercel.app/docs

`hline` opens your shell history in a full-screen terminal UI with live filtering, date filters, timestamp-aware sorting, multi-select, clipboard copy, persisted favorites, and aliases. Press Enter and the command lands in your prompt.

## Features

- Bash, Zsh, and Fish history, auto-detected, with timestamps when the format has them
- Live text filter plus `after:`, `before:`, `on:` date filters
- Sort by recency, timestamp, length, or alphabetically
- Multi-select, copy to clipboard, or accept to stdout for shell widgets
- Favorites: save one line or a multi-line block, rename it, jump between blocks
- Aliases: `hline <favorite>` prints the block and copies it to the clipboard
- `hline init bash|zsh|fish` prints a widget that binds Ctrl+R (configurable)
- Single static binary, no daemon, no database, your history file stays the source of truth

## Install

Latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | bash
```

Then run:

```bash
hline
```

Or install from crates.io. The crate is `hline-tui` because `hline` was taken, the command it installs is `hline`:

```bash
cargo install hline-tui
```

Install a specific version:

```bash
curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | \
  bash -s -- --version v0.1.5
```

If `hline` is not found after install, add this to your shell profile:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

## Usage

```bash
hline
hline --file /path/to/history
hline --format bash
hline --format zsh --file ~/.zsh_history
hline --format fish --file ~/.local/share/fish/fish_history
hline --check-updates
hline --no-update-check
hline --settings
hline --list
hline fav1
hline init zsh
```

By default `hline` auto-detects the history format and chooses a default history file from your shell when possible.

`hline` checks GitHub Releases for updates at most once per day and caches the result in
`~/.cache/hline/update.json` unless `XDG_CACHE_HOME` is set. Update notices are written to stderr
so accepted commands remain clean on stdout for shell widgets.

## Keys

- `j` / `k` or arrows: move
- `Ctrl+d` / `Ctrl+u`: half page
- `g` / `G`: top / bottom
- `/`: search
- `s`: change sort
- `S`: reverse sort
- `Space`: select
- `a`: select shown
- `c`: clear selection
- `y`: copy selected/current item
- `Y`: copy current line
- `f`: save selected/current command block as favorite
- `F`: toggle favorites view
- `J` / `K` or `Shift+Up` / `Shift+Down`: jump by favorite block
- `Enter`: print selected/current command(s) to stdout and quit
- `?`: help
- `q`: quit

## Search Filters

The search box still does case-insensitive text matching, and now also supports time filters:

- `after:YYYY-MM-DD`
- `before:YYYY-MM-DD`
- `on:YYYY-MM-DD`
- `since:` and `until:` aliases

Examples:

```bash
/cargo after:2026-03-01
/git on:2026-03-14
```

Timestamped entries are shown in the list when the loaded history format provides them.

## Favorites

Press `f` in history view to save current line or current multi-selection as one favorite block. Favorites are persisted on disk in `~/.config/hline/favorites.json` unless `XDG_CONFIG_HOME` is set.

Press `F` to open favorites view:

- `y` copies whole favorite block
- `Y` copies current line inside favorite block
- `Enter` accepts whole favorite block to stdout
- `j` / `k` move line by line
- `J` / `K` or `Shift+Up` / `Shift+Down` jump favorite-to-favorite

Favorites search matches any line in each saved block and shows whole matching blocks.

Favorites without a custom name are titled `favN`, so they work as aliases right away.

## Aliases

Favorite titles double as aliases. `hline <name>` prints the block to stdout and copies it
to the clipboard, `hline --list` shows every favorite. Lookup is case-insensitive, exact title
first, then a unique prefix. A favorite titled `init` or `help` is shadowed by the subcommand.

```bash
hline fav1
hr() { eval "$(hline "$1")"; }   # run a favorite
```

## Shell Integration

Pressing `Enter` accepts the current selection and writes it to stdout after the TUI exits.
`hline init <shell>` prints a widget that binds a key to open `hline` and paste the accepted
command into your prompt. Add one of these to your shell profile:

```bash
eval "$(hline init bash)"   # ~/.bashrc
eval "$(hline init zsh)"    # ~/.zshrc
hline init fish | source    # ~/.config/fish/config.fish
```

The key defaults to `Ctrl+R`. Change it in the settings file:

```bash
hline --settings   # prints the path and current values
```

```json
{
  "shell_key": "alt-h"
}
```

`shell_key` accepts `ctrl-<letter>` or `alt-<letter>`. Settings live in
`~/.config/hline/settings.json` unless `XDG_CONFIG_HOME` is set.

## Build

```bash
cargo build --release
./target/release/hline --help
```

## Release

```bash
git tag v0.1.5
git push origin main v0.1.5
```

## Compared to other history tools

| | hline | fzf history widget | atuin | mcfly |
| --- | --- | --- | --- | --- |
| Source of truth | your history file | your history file | own SQLite database, optional sync | own SQLite database |
| Multi-line favorites as aliases | yes | no | no | no |
| Multi-select and clipboard copy | yes | select only | no | no |
| Date filters | yes | no | yes | yes |
| Daemon or background process | no | no | optional | no |

Pick fzf if you already use it everywhere, atuin if you want synced history across machines. Pick hline if you want a visible list, favorites you can call by name, and nothing else running.

## License

MIT

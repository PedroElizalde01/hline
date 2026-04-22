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

Shell history TUI for Linux and macOS.

`hline` lets you browse Bash, Zsh, and Fish history with live filtering, timestamp-aware sorting, multi-select, clipboard copy, and stdout accept flow for shell widgets.

## Install

Latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/PedroElizalde01/hline/main/install.sh | bash
```

Then run:

```bash
hline
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
```

By default `hline` auto-detects the history format and chooses a default history file from your shell when possible.

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
- `y`: copy
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

## Shell Integration

Pressing `Enter` accepts the current selection and writes it to stdout after the TUI exits. That makes shell integration possible with command substitution.

Bash / Zsh example:

```bash
hline-widget() {
  local cmd
  cmd="$(hline)" || return
  [[ -n "$cmd" ]] || return
  READLINE_LINE="$cmd"
  READLINE_POINT=${#READLINE_LINE}
}
bind -x '"\C-r":hline-widget'
```

Fish example:

```fish
function hline-widget
    set cmd (hline)
    or return
    test -n "$cmd"; or return
    commandline -r -- $cmd
end
bind \cr hline-widget
```

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

## License

MIT

<div align="center">
<pre>
  ██╗  ██╗██╗     ██╗███╗   ██╗███████╗
  ██║  ██║██║     ██║████╗  ██║██╔════╝
███████║██║     ██║██╔██╗ ██║█████╗
██╔══██║██║     ██║██║╚██╗██║██╔══╝
  ██║  ██║███████╗██║██║ ╚████║███████╗
  ╚═╝  ╚═╝╚══════╝╚═╝╚═╝  ╚═══╝╚══════╝
</pre>

Bash history TUI
</div>

---

`hline` lets you browse `~/.bash_history` with live filtering, sorting, multi-select, and clipboard copy.

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
```

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
- `?`: help
- `q`: quit

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

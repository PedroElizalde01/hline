# Brag Plan: hline (narrated cut, cursor-accurate multi-select)

Run: `brag-output-2026-09-22-122904/`. Narration is **on** for this run (Kokoro, `af_heart`), carried forward from `brag-output-2026-09-22-112445/`.

## Change in this run
In the previous cut the amber cursor row sat on `cargo build --release` for the whole multi-select while the checkmarks appeared on three other rows — which is not how the TUI works. The cursor now walks: it lands on the first match after the filter, `Space` checks the row it is sitting on, then it moves down to the next row it is about to select. The `> ` gutter symbol and the reversed-video highlight move together, and each move gets a soft `ui/rollover2` cue distinct from the selection click. Everything else is unchanged.

## What is this app?
`hline` is a full-screen shell history TUI for Bash, Zsh, and Fish, written in Rust. It replaces `Ctrl+R` with a browsable, filterable list, date filters, multi-select, favorites, and aliases you can run by name.

## The angle
Everyone already has the feature and nobody likes it. `Ctrl+R` shows one match at a time with no context and no way back. The video does not pitch a new concept, it replaces a bad one on screen: cold-open on the stock `bck-i-search` dead end, kill it, then spend the rest of the runtime inside the real TUI doing real work. With narration carrying the argument, the frame belongs almost entirely to the product — on-screen captions are stripped down to one line of terminal syntax.

## Hook (first 2-3 seconds)
Black frame, one shell prompt, a blinking block cursor, and the narrator's first line already running: "Every shell has a history search." The `Ctrl+R` keycap flashes on the word, the stock `(reverse-i-search)` line appears with exactly one mangled match on "one match at a time", and stops there.

## Key moments (the middle)
- The `hline` TUI slamming into the full frame: bordered list titled `hline`, sixteen timestamped history rows, `> ` cursor, real status bar.
- `/cargo after:2026-03-01` typing out and `1284 total` collapsing to `6 shown` on a strong beat.
- The cursor walking down the filtered list, checking each row it lands on, then `f` and the status bar answering `saved favorite`, then `F` flipping to the favorites view with `check (3 lines)`.
- Back at the shell: `hline check` printing `Copied to clipboard:` on stderr and the three commands on stdout.

## Outro / punchline
The ASCII `HLINE` logo assembles on the narrator saying the name, then the README tagline and the install line. The punchline is that the whole thing is one static binary and one command.

## User flow worth showing
1. Press `Ctrl+R` (widget from `hline init zsh`), the TUI opens on your real history.
2. Filter it down (`/cargo after:2026-03-01`), select lines, save the block as a favorite (`f`).
3. Later, run it by name: `hline check` prints the block and copies it.

## Tone
- Preset: `default`
- Creative direction: terminal-native dev-tool demo, premium restraint, narrated like a colleague showing you the tool
- Interpretation: the voice sets the pace, so scene lengths follow the generated narration rather than a fixed grid. Motion stays fast and mechanical — cuts and single-frame state changes. On-screen captions are removed wherever the narrator already carries the point, leaving the terminal itself as the only thing to read.

## Format: landscape — 1920x1080
## Duration: 23.9s (set by the narration, not chosen up front)

## Voiceover script
Kokoro voice `af_heart`, one WAV per scene so each line lands on its own beat.

| Clip | Start | Length | Line |
|---|---|---|---|
| `vo1` | 0.30s | 3.75s | "Every shell has a history search. It shows you one match at a time." |
| `vo2` | 4.65s | 1.98s | "hline puts all of it on screen." |
| `vo3` | 7.30s | 1.94s | "Filter as you type, or by date." |
| `vo4` | 10.20s | 3.18s | "Save the lines you keep retyping as one named block." |
| `vo5` | 14.90s | 2.56s | "Then run it by name. It prints, and copies." |
| `vo6` | 19.10s | 3.80s | "hline. One static binary. Bash, zsh, and fish." |

Total speech 17.2s across a 23.9s cut. The script never reads the screen: the status bar, the query, and the printed commands are shown, not narrated, and the outro tagline on screen is "A Ctrl+R replacement written in Rust." while the voice says "One static binary."

## Visual identity (from the project)
The product is a ratatui TUI, so the identity is the terminal, reproduced faithfully.
- Background: `#0B0C0E`
- Text: `#E6E6E6`
- Accent: `#FFD866` (amber, matching `Color::Yellow` on the search prompt in `src/ui.rs`)
- Status bar: `#4A4A4A` background, `#FFFFFF` text (`Color::DarkGray` / `Color::White`)
- Dim text: `#8A8F98` (timestamps, stderr header, footer URL)
- Selection: reversed video — amber block, dark text, `> ` gutter symbol
- Display and body font: DejaVu Sans Mono, shipped with the composition
- Strongest visual element: the bordered list box titled `hline` with the live status bar, and the ASCII block logo from `README.md`

## Privacy note
All history rows, favorite blocks, and hosts on screen are fictional stand-ins (generic `cargo`, `git`, `docker`, `ssh` lines). No real history, hostname, username, or path from this machine appears anywhere.

## Share copy (draft)
Made hline: a shell history TUI for Bash, Zsh, and Fish. Ctrl+R shows one match at a time. hline shows the whole list.

## Audio direction
- Role: narration-led, with a music bed ducked under the voice and sparse motion-matched interface accents
- Music: `happy-beats-business-moves-vol-12-by-ende-dot-app.mp3` (steady and clean, 109.96 BPM)
- Music treatment: 0.10 under the cold open, 0.14 for the narrated body (ducked for the voice), lifting to 0.24 when the last line ends at 22.95s, fading to 0 by 23.9s
- Music cue guidance: bundled preset. Three strong-cue locks: **9.29s** (filter collapses), **13.11s** (favorite saved), **19.66s** (ASCII logo lands). Beat-grid windows: the three `Space` selections at 10.93 / 11.46 / 12.02, with the cursor moves falling between them at 11.30 / 11.86; the stderr header at 16.38 and the three printed lines at 16.93 / 17.47 / 18.02.
- Audio-reactive treatment: subtle; music RMS breathes the terminal vignette and the amber glow behind the selected row. No waveform, no bars, no pulsing text.
- SFX posture: sparse, every cue tied to a visible keystroke or state change, all levels cut ~25% from the unnarrated cut so nothing competes with the voice
- Restraint rule: no sound without a matching visible event, and nothing loud enough to sit on top of a spoken line.

## Storyboard

### Scene 1 — Cold open: Ctrl+R — 4.4s
Black frame, `$ ` prompt with a blinking block cursor. `vo1` starts at 0.30s. At 1.70s an amber `Ctrl+R` keycap flashes on "history search". At 2.15s the prompt is replaced by `(reverse-i-search)`carg': cargo build --release` and freezes there under "one match at a time". No caption.
Sequential/interaction: yes — simulated keypress, then the shell's own search line.
Audio intent: quiet and slightly dead; music at 0.10 so the first line lands clean.
Transition mood: hard → Scene 2

### Scene 2 — hline lands — 2.9s (4.4-7.3s)
Hard cut. The TUI fills the frame: bordered box titled `hline`, sixteen `[ ]` rows with `2026-03-18 14:02` timestamps, the `> ` cursor on row 4 in reversed amber, and the real status bar. Rows redraw top-to-bottom in 0.35s. `vo2` runs 4.65-6.63s.
Audio intent: dry weight on the landing, music opens to its ducked bed.
Transition mood: clean → Scene 3

### Scene 3 — Filter — 2.9s (7.3-10.2s)
Same frame, no cut. `/cargo after:2026-03-01` types out character by character from 7.30s under `vo3`. At **9.29s** the list collapses to six rows in one frame and the status bar updates to `filter: cargo after:2026-03-01 | 1284 total | 6 shown`. The only caption in the video sits under it: `after:` `before:` `on:` in amber, 7.45-10.25s.
Sequential/interaction: yes — per-character typing, then a single-frame collapse.
Transition mood: clean → Scene 4

### Scene 4 — Favorite it — 4.4s (10.2-14.6s)
Same frame. The cursor is on the first match, `cargo test --all`, from the moment the filter lands at 9.29s. At **10.93s** `Space` checks the row it is on and the counter reads `1 selected`. At 11.30s the cursor walks down to `cargo clippy -- -D warnings`; at **11.46s** `Space` checks it. At 11.86s it walks to `cargo fmt --all`; at **12.02s** `Space` checks it. The highlight is always the row being selected. At 12.55s an `f` keycap flashes and at **13.11s** the status bar appends `| saved favorite` on "one named block". At 13.65s the view flips to the favorites box: title `favorites`, row `[ ] check (3 lines)` with its three indented lines, status `favorites: 3 blocks`.
Sequential/interaction: yes — cursor move, Space, cursor move, Space, cursor move, Space, then `f`, then `F`.
Audio intent: a soft rollover under each cursor move, a crisp click on each Space, one warmer confirm on the save. Precision, not celebration.
Transition mood: hard → Scene 5

### Scene 5 — Run it by name — 4.1s (14.6-18.7s)
Hard cut to the bare shell. `$ hline check` types out 14.95-15.85s under "Then run it by name." At 16.38s the dim stderr header `Copied to clipboard:` appears, then the three commands print on 16.93 / 17.47 / 18.02 under "It prints, and copies."
Sequential/interaction: yes — typed command, then lines printing one by one.
Transition mood: clean → Scene 6

### Scene 6 — Logo and install — 5.2s (18.7-23.9s)
The ASCII `HLINE` logo wipes in column by column and lands at **19.66s**, on the narrator saying the name. "A Ctrl+R replacement written in Rust." at 20.5s, `cargo install hline-tui` in an amber box at 21.4s, `hline.vercel.app` at 22.5s. `vo6` ends at 22.9s; the music lifts, then fades out to 23.9s.
Audio intent: one resonant bell on the logo, then only the music resolving.
Transition mood: hold to black

**Music mood for this video:** steady and clean, permanently ducked under the voice.
**Audio summary:** A near-silent bed under the first spoken line, a ducked warm bed through the demo carried by keystrokes and interface clicks that all match a visible action, one bell at the logo, and the music rising briefly to close once the narrator stops.

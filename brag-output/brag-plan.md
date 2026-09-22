# Brag Plan: hline

## What is this app?
`hline` is a full-screen shell history TUI for Bash, Zsh, and Fish, written in Rust. It replaces `Ctrl+R` with a browsable, filterable list, date filters, multi-select, favorites, and aliases you can run by name.

## The angle
Everyone already has the feature. `Ctrl+R` ships with every shell and nobody likes it: one match at a time, no context, no way back. The video does not sell a new concept, it replaces a bad one in front of you. Cold-open on the stock `bck-i-search` prompt, kill it, and spend the rest of the video inside the real TUI doing real work: filter, favorite, alias. Terminal-native throughout. No browser chrome, no marketing gradient, no device mockup.

## Hook (first 2-3 seconds)
Black frame, one shell prompt, a blinking block cursor. `Ctrl+R` keycap flashes. The stock `(reverse-i-search)\`carg':` line appears with exactly one mangled match, and stops there. One overlay line: **"One match at a time."** That dead end is the hook.

## Key moments (the middle)
- The `hline` TUI slamming into the full frame: bordered list titled `hline`, timestamped history rows, `> ` cursor, real status bar at the bottom.
- Typing `/cargo after:2026-03-01` into the search line and watching `1284 total` collapse to `6 shown` on a single beat.
- Multi-selecting three lines with `Space`, pressing `f`, the status bar answering `saved favorite`, then `F` flipping to the favorites view with `check (3 lines)`.
- Back at the shell: `hline check` prints `Copied to clipboard:` on stderr and the three commands on stdout.

## Outro / punchline
The ASCII `HLINE` block logo from the README assembles, tagline under it, then the install line. The punchline is that the whole thing is one static binary you install with one command.

## User flow worth showing
Entry → key action → result:
1. Press `Ctrl+R` (widget from `hline init zsh`), the TUI opens on your real history.
2. Filter it down (`/cargo after:2026-03-01`), select lines, save the block as a favorite (`f`).
3. Later, run it by name: `hline check` prints the block and copies it.

## Tone
- Preset: `default`
- Creative direction: terminal-native dev-tool demo, premium restraint, no marketing voice
- Interpretation: six scenes instead of the usual four because the flow is the product. Motion is fast and mechanical (cuts, not dissolves), holds are long enough to read every line of terminal text. Copy is short and factual, phrased the way the tool's own status bar is. Nothing bounces, nothing glows, nothing is title-cased marketing.

## Format: landscape — 1920x1080
## Duration: 23.2s target

## Visual identity (from the project)
The product is a ratatui TUI, so there is no CSS palette. The identity is the terminal itself, reproduced faithfully.
- Background: `#0B0C0E` (near-black terminal ground)
- Text: `#E6E6E6` (default terminal foreground)
- Accent: `#FFD866` (amber, matching `Style::default().fg(Color::Yellow)` on the search and rename prompts in `src/ui.rs`)
- Status bar: `#4A4A4A` background, `#FFFFFF` text (`Color::DarkGray` bg / `Color::White` fg, `render_status`)
- Selection: reversed video (`Modifier::REVERSED`) — amber block behind dark text, `> ` gutter symbol
- Display font: JetBrains Mono (fallback: `ui-monospace`, `SFMono-Regular`, `Menlo`, monospace)
- Body font: same monospace stack — everything on screen is terminal text
- Strongest visual element: the bordered list box titled `hline` with the live status bar under it, and the ASCII block logo from `README.md`

## Privacy note
All history rows, favorite blocks, and paths on screen are fictional stand-ins (generic `cargo`, `git`, `docker`, `ssh` lines). No real history, hostnames, usernames, or paths from this machine are used anywhere in the composition.

## Share copy (draft)
Made hline: a shell history TUI for Bash, Zsh, and Fish. Ctrl+R, except you can see your history, filter it by date, and save the commands you keep retyping as named aliases.

## Audio direction
- Role: warm bed with sparse, motion-matched interface accents
- Music: `happy-beats-business-moves-vol-12-by-ende-dot-app.mp3` (steady and clean, 109.96 BPM)
- Music treatment: starts at 0.0 under the cold open at ~0.18, rises to 0.34 when the TUI lands, fades out over the final 1.2s so the last install line sits nearly dry
- Music cue guidance: bundled preset read from `assets/music/cues/happy-beats-business-moves-vol-12-by-ende-dot-app.music-cues.json`. Three strong-cue locks: **8.74s** (filter collapses), **13.11s** (favorite saved), **19.66s** (ASCII logo lands). Beat-grid windows for sequential reveals: the three `Space` selections at 10.93 / 11.46 / 12.02, and the stderr header at 16.38 and the three printed lines at 16.93 / 17.47 / 18.02. Final install line may bias toward 22.37.
- Audio-reactive treatment: subtle; use music RMS to breathe the terminal's background vignette and the amber glow behind the selected row. No waveform, no bars, no pulsing text.
- SFX posture: sparse to moderate, every cue tied to a visible keystroke or state change
- Audio-coupled moments: per-character typing on the search query and on `hline check`; a dry hit when the TUI lands; a click per `Space` selection; a soft confirm on `saved favorite`; one bell on the logo
- Restraint rule: no sound without a matching visible event. Nothing ambient, nothing decorative, no sound during the held logo except the music fading.

## Storyboard

### Scene 1 — Cold open: Ctrl+R — 2.7s
Black frame, `$ ` prompt top-left with a blinking block cursor. At 0.5s a small amber `Ctrl+R` keycap flashes center-right. At 0.8s the prompt line is replaced by `(reverse-i-search)`carg': cargo build --release` typed in one shot, and freezes there. At 1.5s the overlay line **"One match at a time."** fades up under it and holds 1.0s.
Sequential/interaction: yes — simulated keypress, then a single search line appears as if typed by the shell.
Audio intent: quiet, slightly dead. The music is barely there. The viewer should feel the dead end.
Audio-coupled idea: one keypress tick on the `Ctrl+R` flash, one soft tick as the search line appears.
Music: low bed, `vol-12` from 0.0 at 0.18 volume.
Transition mood: hard → Scene 2

### Scene 2 — hline lands — 3.7s
Hard cut. The `hline` TUI fills the frame: a single-line bordered box titled `hline`, twelve history rows with `[ ]` checkboxes and `2026-03-14 09:41` style timestamps, the `> ` cursor on row 4 highlighted in reversed amber, and the real status bar pinned to the bottom reading `sort: recency | filter: no filter | 1284 total | 1284 shown | 0 selected | enter=accept  y=copy  f=save fav  F=favorites  /=search`. Rows fill in top-to-bottom over 0.35s. At 5.2s a caption sits in the lower third: **"Your whole history. On screen."**
Sequential/interaction: yes — the history rows populate top to bottom, fast, like a redraw.
Audio intent: the arrival. Dry weight, then the music opens up to full bed.
Audio-coupled idea: one impact on the frame landing, a very short flurry under the row redraw.
Music: steps up to 0.34.
Transition mood: clean → Scene 3

### Scene 3 — Filter — 4.0s
Same frame, no cut. The search line appears at the bottom in amber and `/cargo after:2026-03-01` types out character by character from 6.6s to 8.5s. At **8.74s** the list collapses to six rows in one frame and the status bar updates to `filter: cargo after:2026-03-01 | 1284 total | 6 shown`. Caption: **"Text filter plus `after:` `before:` `on:`"** holds 1.3s.
Sequential/interaction: yes — per-character typing, then a single-frame list collapse.
Audio intent: mechanical, deliberate. Typing carries the scene, the collapse is the payoff.
Audio-coupled idea: randomized keypress per character; one dry hit on the collapse.
Music: steady. Collapse beat-locked to the 8.74s strong cue.
Transition mood: clean → Scene 4

### Scene 4 — Favorite it — 4.2s
Same frame. Three rows check `[x]` one at a time at 10.93 / 11.46 / 12.02 (beat grid), status `selected` counter ticking 1 → 2 → 3. At 12.6s an `f` keycap flashes and at **13.11s** the status bar appends `| saved favorite`. At 13.6s the view flips to the favorites box: title `favorites`, row `[ ] check (3 lines)` with its three indented command lines under it, status `favorites: 3 blocks`. Caption: **"Save a block. Name it."**
Sequential/interaction: yes — three `Space` selections one by one, then `f`, then `F` to the favorites view.
Audio intent: three crisp identical clicks, then a single warmer confirm. Precision, not celebration.
Audio-coupled idea: click per selection on the beat grid; soft confirm on `saved favorite`; a card-slide under the view flip.
Music: steady.
Transition mood: hard → Scene 5

### Scene 5 — Run it by name — 4.2s
Hard cut back to the bare shell. `$ hline check` types out from 15.0s to 16.0s. At 16.38s the stderr header `Copied to clipboard:` appears in dimmed text, then the three commands print on 16.93 / 17.47 / 18.02 (beat grid), each fully readable. Caption: **"Favorite titles are aliases."**
Sequential/interaction: yes — typed command, then lines printing one by one.
Audio intent: the payoff of the whole flow. Typing, then three quiet drops as the lines land.
Audio-coupled idea: randomized keypresses on the typed command; one soft drop per printed line.
Music: steady, starting to open toward the outro.
Transition mood: clean → Scene 6

### Scene 6 — Logo and install — 4.4s
The shell text clears upward. The ASCII `HLINE` block logo from `README.md` assembles column by column over 0.5s and lands at **19.66s**. Under it: **"Shell history TUI for Bash, Zsh, and Fish."** at 20.3s, then `cargo install hline-tui` in an amber-bordered box at 21.3s, then `hline.vercel.app` small and dim at 22.37s. Hold to 23.2s.
Sequential/interaction: yes — the logo assembles, then three lines arrive with full reading holds.
Audio intent: one resonant bell on the logo, then nothing but the music fading out under the install line.
Audio-coupled idea: bell on the logo landing; no SFX after it.
Music: fades from 0.34 to 0 over the final 1.2s.
Transition mood: hold to black

**Music mood for this video:** upbeat but restrained — steady and clean, never celebratory.
**Audio summary:** A low bed under the dead-end cold open, opening to a full warm bed the instant the TUI lands, carried by keystrokes and interface clicks that all match a visible action, resolving on one bell at the logo and a clean fade under the install line.

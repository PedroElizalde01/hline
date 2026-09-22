# Hyperframes Composition Brief: hline (narrated cut)

## Objective
Create a short, narrated launch-style brag video for `hline`, a shell history TUI for Bash, Zsh, and Fish written in Rust. Narration is enabled for this run (`/brag --voice`).

## Output
- Composition directory: `brag-output-2026-09-22-112445/composition/`
- Rendered video: `brag-output-2026-09-22-112445/brag.mp4`
- Format: landscape — 1920x1080, 30fps
- Duration: 23.9 seconds, set by the generated narration

## Source Material
- Project root: `/home/pedro/projects/hline`
- Primary files read: `README.md`, `Cargo.toml`, `CHANGELOG.md`, `src/ui.rs`, `src/app.rs`
- Product name: `hline`
- Tagline / strongest claim: "Shell history TUI for Bash, Zsh, and Fish. A Ctrl+R replacement written in Rust."
- Key UI to recreate: the ratatui TUI from `src/ui.rs` — a bordered list box titled `hline`, rows prefixed `[ ]` with `YYYY-MM-DD HH:MM` timestamps, the cursor row drawn with `> ` and reversed video, an amber search line, and a dark-gray status bar.
- Copy that must appear verbatim:
  - `(reverse-i-search)`carg': cargo build --release`
  - `hline` / `favorites` (list box titles)
  - `sort: recency | filter: no filter | 1284 total | 1284 shown | 0 selected`
  - `enter=accept  y=copy  f=save fav  F=favorites  /=search`
  - `/cargo after:2026-03-01`
  - `filter: cargo after:2026-03-01 | 1284 total | 6 shown`
  - `saved favorite`
  - `[ ] check (3 lines)`
  - `favorites: 3 blocks`
  - `$ hline check`
  - `Copied to clipboard:`
  - `A Ctrl+R replacement written in Rust.`
  - `cargo install hline-tui`
  - `hline.vercel.app`

## Privacy constraint
Every history row, favorite block, and host shown is a fictional stand-in (generic `cargo`, `git`, `docker`, `ssh` commands). No real shell history, hostname, username, email, or absolute path from this machine is rendered.

## Creative Direction
- Tone preset: `default`
- Creative direction: terminal-native dev-tool demo, premium restraint, narrated like a colleague showing you the tool
- Interpretation: the voice sets the pace — scene lengths follow the generated narration, not a fixed grid. Motion stays fast and mechanical, cuts and single-frame state changes. On-screen captions are stripped to one line of terminal syntax because the narrator carries everything else.
- Angle: everyone already has this feature and nobody likes it. Cold-open on the stock `bck-i-search` dead end, kill it, then spend the rest of the runtime inside the real TUI doing real work.
- Hook: `$ ` prompt, blinking cursor, `Ctrl+R` keycap flash, the stock reverse-i-search line appearing with exactly one mangled match — under the narrator's first line.
- Outro / punchline: the ASCII `HLINE` logo assembles on the narrator saying the name, then the tagline and `cargo install hline-tui`.
- Avoid:
  - Generic SaaS language
  - Abstract filler visuals, gradient washes, particle fields
  - Browser chrome or device mockups
  - Captions that repeat what the narrator just said
  - Redesigning the TUI — reproduce what `src/ui.rs` actually draws

## Visual Identity
- Background: `#0B0C0E`
- Text: `#E6E6E6`
- Accent: `#FFD866` (matches `Color::Yellow` on the search prompt in `src/ui.rs`)
- Status bar: `#4A4A4A` background, `#FFFFFF` text
- Dim text: `#8A8F98`
- Selection: reversed video — amber block, `#0B0C0E` text, `> ` gutter
- Font: DejaVu Sans Mono, shipped in `assets/fonts/` and declared with `@font-face`
- Visual references: single-line box borders with the title in the top border, the README ASCII banner, the real status-bar format from `App::status_hint_text` / `render_status`

## Voiceover (enabled)
- Engine: Kokoro via `npx hyperframes tts`, voice `af_heart`
- One WAV per scene in `assets/vo/`, each placed as its own `<audio>` clip so lines land on their own beats:
  - `vo1` 0.30s / 3.75s — "Every shell has a history search. It shows you one match at a time."
  - `vo2` 4.65s / 1.98s — "hline puts all of it on screen."
  - `vo3` 7.30s / 1.94s — "Filter as you type, or by date."
  - `vo4` 10.20s / 3.18s — "Save the lines you keep retyping as one named block."
  - `vo5` 14.90s / 2.56s — "Then run it by name. It prints, and copies."
  - `vo6` 19.10s / 3.80s — "hline. One static binary. Bash, zsh, and fish."
- Scene durations flex to the measured WAV lengths; nothing is hardcoded ahead of the audio.
- The script complements the screen instead of reading it: the query, the status bar, and the printed commands are shown, never spoken.

## Storyboard
Use the storyboard in `brag-output-2026-09-22-112445/brag-plan.md` as the creative contract.

Scene summary:
1. Cold open: Ctrl+R — 0.0-4.4s — reverse-i-search dead end under `vo1`.
2. hline lands — 4.4-7.3s — full-frame TUI, 16 rows redraw, real status bar, `vo2`.
3. Filter — 7.3-10.2s — query types out, list collapses to 6 on 9.29s, `after: before: on:` caption, `vo3`.
4. Favorite it — 10.2-14.6s — three `[x]` on the beat grid, `saved favorite` at 13.11s, flip to favorites, `vo4`.
5. Run it by name — 14.6-18.7s — `$ hline check`, `Copied to clipboard:`, three lines print, `vo5`.
6. Logo and install — 18.7-23.9s — ASCII logo lands at 19.66s, tagline, install line, URL, `vo6`.

## Audio
- Audio role: narration-led, with a ducked music bed and sparse motion-matched interface accents
- Audio arc: near-silent under the first spoken line, ducked warm bed through the demo, one bell at the logo, music rising briefly once the narrator stops, then fading out.
- Music: `assets/music/happy-beats-business-moves-vol-12-by-ende-dot-app.mp3`
- Music treatment: volume lane `0.10 → 0.14` (ducked under the voice for the whole body) `→ 0.24` at 22.95s `→ 0` at 23.9s
- Music cue guidance: bundled preset, 109.96 BPM. Strong-cue locks at **9.29s** (collapse), **13.11s** (`saved favorite`), **19.66s** (logo). Beat grid: selections 10.93 / 11.46 / 12.02; stderr header 16.38; printed lines 16.93 / 17.47 / 18.02.
- Audio-reactive treatment: subtle — music RMS breathes the terminal vignette and the amber presence behind the selected row. No waveform, no equalizer bars, no pulsing text.
- SFX selection guidance: everything maps to a visible keystroke or state change. `keyboard/keypress-*.wav` for typing, `interface/click_003` for selections, `interface/drop_*` for printed lines, `impact/impactSoft_medium_*` for the TUI landing and the collapse, `casino/card-slide-1` for the view flip, one `impact/impactBell_heavy_000` for the logo. All levels cut ~25% versus the unnarrated cut.
- Track allocation: voiceover on 3-8, music on 10, SFX from 11 upward, one index per clip.
- Audio files: music, SFX, and the six voiceover WAVs all live under `composition/assets/`.

## Hyperframes Instructions
Load `hyperframes-core`, `hyperframes-animation`, `hyperframes-creative`, `hyperframes-keyframes`, `hyperframes-cli`. /brag is its own workflow: do not enter the `hyperframes` entry-point intent interview.

Requirements:
- Reproduce the real TUI from `src/ui.rs` — the centerpiece, not a garnish.
- Keep all terminal text readable: 34px rows, 22px status bar, nothing smaller.
- Let the narration set scene lengths; do not hardcode durations ahead of the generated audio.
- Duck the music under every spoken line.
- Use only the three strong-cue locks; mark them `// beat-locked`. Snap sequential reveals to the listed beats within ±0.10s; mark them `// beat-grid`.
- Extract per-frame audio data and wire at least one subtle visual property to it.
- Local assets only, never absolute paths.
- `npx hyperframes check` must pass before render.

# Hyperframes Composition Brief: hline

## Objective
Create a short launch-style brag video for `hline`, a shell history TUI for Bash, Zsh, and Fish written in Rust.

## Output
- Composition directory: `brag-output/composition/`
- Rendered video: `brag-output/brag.mp4`
- Format: landscape — 1920x1080
- Duration: 23.2 seconds

## Source Material
- Project root: `/home/pedro/projects/hline`
- Primary files read: `README.md`, `Cargo.toml`, `CHANGELOG.md`, `src/ui.rs`, `src/app.rs`
- Product name: `hline`
- Tagline / strongest claim: "Shell history TUI for Bash, Zsh, and Fish. A Ctrl+R replacement written in Rust."
- Key UI to recreate: the ratatui TUI from `src/ui.rs` — a single-line bordered list box titled `hline`, rows prefixed `[ ]` with `YYYY-MM-DD HH:MM` timestamps, the cursor row drawn with `> ` and reversed video, an amber search line under it, and a dark-gray status bar pinned to the bottom.
- Copy that must appear verbatim:
  - `(reverse-i-search)`carg': cargo build --release`
  - `hline` (list box title)
  - `sort: recency | filter: no filter | 1284 total | 1284 shown | 0 selected`
  - `enter=accept  y=copy  f=save fav  F=favorites  /=search`
  - `/cargo after:2026-03-01`
  - `filter: cargo after:2026-03-01 | 1284 total | 6 shown`
  - `saved favorite`
  - `favorites` (list box title in favorites view)
  - `[ ] check (3 lines)`
  - `favorites: 3 blocks`
  - `$ hline check`
  - `Copied to clipboard:`
  - `Shell history TUI for Bash, Zsh, and Fish.`
  - `cargo install hline-tui`
  - `hline.vercel.app`

## Privacy constraint
Every history row, favorite block, and path shown must be a fictional stand-in (generic `cargo`, `git`, `docker`, `ssh` commands). Do not render any real shell history, hostname, username, email, or absolute path from this machine.

## Creative Direction
- Tone preset: `default`
- Creative direction: terminal-native dev-tool demo, premium restraint, no marketing voice
- Interpretation: six scenes because the user flow is the product. Motion is fast and mechanical — cuts and single-frame state changes, not dissolves. Holds are long enough to read every line of terminal text. Copy is short and factual, phrased like the tool's own status bar. No bouncing, no glow bloom, no title-cased marketing.
- Angle: everyone already has this feature and nobody likes it. `Ctrl+R` shows one match at a time with no context and no way back. The video does not pitch a new concept, it replaces a bad one on screen: cold-open on the stock `bck-i-search` dead end, kill it, then spend the rest of the runtime inside the real TUI doing real work — filter, favorite, alias.
- Hook: black frame, `$ ` prompt with a blinking block cursor, a `Ctrl+R` keycap flash, the stock reverse-i-search line appearing with exactly one mangled match and stopping there. Overlay: "One match at a time."
- Outro / punchline: the ASCII `HLINE` block logo from `README.md` assembles, tagline under it, then `cargo install hline-tui` — the whole thing is one static binary and one command.
- Avoid:
  - Generic SaaS language
  - Abstract filler visuals, gradient washes, particle fields
  - Browser chrome, laptop/phone mockups, or any non-terminal frame
  - Unrelated visual redesign of the TUI — reproduce what `src/ui.rs` actually draws

## Visual Identity
The product is a ratatui TUI, so there is no CSS palette. The identity is the terminal, reproduced faithfully.
- Background: `#0B0C0E`
- Text: `#E6E6E6`
- Accent: `#FFD866` (amber — matches `Color::Yellow` on the search and rename prompts in `src/ui.rs`)
- Status bar: `#4A4A4A` background, `#FFFFFF` text (`Color::DarkGray` / `Color::White` in `render_status`)
- Dim/secondary text: `#8A8F98` (stderr header, timestamps, footer URL)
- Selection treatment: reversed video — amber block, dark `#0B0C0E` text, `> ` gutter symbol (`Modifier::REVERSED` + `highlight_symbol("> ")`)
- Display font: JetBrains Mono, fallback `ui-monospace, SFMono-Regular, Menlo, monospace`
- Body font: same monospace stack. Everything on screen is terminal text.
- Visual references from the project:
  - Single-line box borders with a title in the top border (`Block::default().title(...).borders(Borders::ALL)`)
  - The ASCII block `HLINE` banner at the top of `README.md`
  - The real status-bar string format from `App::status_hint_text` and `render_status`

## Storyboard
Use the storyboard in `brag-output/brag-plan.md` as the creative contract.

Scene summary:
1. Cold open: Ctrl+R — 2.7s — the stock reverse-i-search dead end, one match, "One match at a time."
2. hline lands — 3.7s — full-frame TUI, 12 timestamped rows redraw top-to-bottom, real status bar, "Your whole history. On screen."
3. Filter — 4.0s — `/cargo after:2026-03-01` types out, list collapses to 6 rows in one frame, status updates, "Text filter plus after: before: on:"
4. Favorite it — 4.2s — three rows check `[x]` one by one, `f` flashes, status appends `saved favorite`, view flips to favorites showing `[ ] check (3 lines)`, "Save a block. Name it."
5. Run it by name — 4.2s — cut to bare shell, `$ hline check` types, `Copied to clipboard:` then three commands print one by one, "Favorite titles are aliases."
6. Logo and install — 4.4s — ASCII HLINE assembles, tagline, `cargo install hline-tui`, `hline.vercel.app`.

## Audio
- Audio role: warm bed with sparse, motion-matched interface accents
- Audio arc: nearly dry under the cold-open dead end, steps up to a full bed the instant the TUI lands, carried through by keystrokes and clicks that each match a visible event, resolves on one bell at the logo and fades to silence under the install line.
- Music: `assets/music/happy-beats-business-moves-vol-12-by-ende-dot-app.mp3`
- Music treatment: start at 0.0 at volume 0.18, step to 0.34 at the Scene 2 cut, fade 0.34 → 0 across the final 1.2s (22.0s → 23.2s).
- Music cue guidance: bundled preset, 109.96 BPM. Copy the cue JSON into `assets/music/cues/`. Three strong-cue locks only:
  - **8.74s** — the filtered list collapses to 6 rows (major)
  - **13.11s** — `saved favorite` appears in the status bar (major)
  - **19.66s** — the ASCII logo lands (major)
  Beat-grid windows for sequential reveals: the three `Space` selections at 10.93 / 11.46 / 12.02; the stderr header at 16.38 and the three printed alias lines at 16.93 / 17.47 / 18.02. The final install line may bias toward 22.37.
- Audio-reactive treatment: subtle — use music RMS to breathe the terminal background vignette and the amber presence behind the selected row. No waveform, no equalizer bars, no pulsing text.
- Audio-coupled moments:
  - Scene 1 `Ctrl+R` keycap flash — single keypress tick
  - Scene 1 reverse-i-search line appears — one soft tick
  - Scene 2 frame landing — one dry impact, short flurry under the row redraw
  - Scene 3 `/cargo after:2026-03-01` — randomized per-character keypresses
  - Scene 3 list collapse at 8.74s — one dry hit
  - Scene 4 three `Space` selections — one identical click each, on the beat grid
  - Scene 4 `saved favorite` at 13.11s — one soft confirm
  - Scene 4 view flip to favorites — card slide
  - Scene 5 `$ hline check` — randomized per-character keypresses
  - Scene 5 three printed lines — one soft drop each
  - Scene 6 logo landing at 19.66s — one bell, then no SFX for the rest of the video
- SFX selection guidance: everything must map to a visible keystroke or state change. `keyboard/keypress-*.wav` randomized for typing, `interface/click_*` or `ui/click*` for the `Space` selections, `interface/drop_*` for printed lines, `impact/impactSoft_medium_*` for the TUI landing and the list collapse, `casino/card-slide-*` for the favorites view flip, one `impact/impactBell_heavy_*` for the logo. Nothing ambient, nothing decorative.
- SFX analysis guidance: read `<skill-dir>/assets/sfx/sfx-analysis.md` and prefer low/medium high-frequency-risk files — the typing is repeated 30+ times and must not get fatiguing.
- Exact SFX choice: Hyperframes chooses filenames, timestamps, density, and volume based on the implemented animation. Music bed on track-index 10, SFX from 11 upward, never sharing a track-index between overlapping clips. SFX volume 0.55-0.75, typing at the low end.
- Audio files: copy the chosen music and any selected SFX into `brag-output/composition/assets/`.

## Hyperframes Instructions
Load the composition-building Hyperframes domain skills — `hyperframes-core`, `hyperframes-animation`, `hyperframes-creative`, `hyperframes-keyframes`, `hyperframes-cli`. /brag is its own workflow: do not enter the `hyperframes` entry-point intent interview and do not route into its generic promo / launch-video workflow. Prefer native Hyperframes conventions over anything in `/brag`.

Requirements:
- Reproduce the real TUI from `src/ui.rs` — this is the "show the thing" requirement and it is the centerpiece, not a garnish.
- Keep all terminal text readable in the final render. Terminal rows are small by nature: use a generous base font size (the 12-row list should fill most of a 1080p frame) and never render body rows below ~24px.
- Keep the video within 15-25 seconds (target 23.2s).
- Include the planned music/SFX layer.
- Treat /brag audio notes as guidance, not a fixed cue sheet. Choose SFX after the visual animation exists.
- Treat music cue metadata as optional timing hints; ignore cues that hurt readability or pacing.
- Use only the three strong-cue locks listed above; mark them `// beat-locked`.
- Snap the sequential reveals to the listed beat timestamps within ±0.10s; mark them `// beat-grid`.
- Honor the music fade-out under the final install line.
- Extract per-frame audio data and wire at least one subtle visual property to it.
- Use local assets only; never absolute paths.
- Run `npx hyperframes check` before render — it is brag's single gate.

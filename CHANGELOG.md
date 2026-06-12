# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2026-06-12

### Added
- Copy preview panel: on wide terminals, a side panel shows the exact lines the next copy/accept will emit (multi-selection in history, multi-line block in favorites)
- `r` in favorites view renames the current block; custom titles persist in `favorites.json` and an empty title reverts to the default `favorite n`
- Favorites search filter also matches custom block titles

### Fixed
- `f` in favorites view now removes the current favorite block (previously there was no way to unfavorite)
- Release workflow now fails if the tag does not match the `Cargo.toml` version, preventing releases that report a stale version and trigger endless update notices

### Added
- Local release packaging script (`scripts/release_local.sh`)
- Installer script for GitHub Releases (`install.sh`)
- Target detection helper (`scripts/print_target.sh`)
- Distribution documentation and release checklists
- Bash, Zsh, and Fish history parsing with configurable format selection
- Timestamp-aware history entries, timestamp sorting, and time-based search filters
- `Enter` accept flow that writes selected/current commands to stdout for shell integration
- Persisted favorites blocks with dedicated favorites view, whole-block copy, single-line copy, and block-jump navigation

## [0.1.0] - 2026-02-21

### Added
- Initial `hline` release: bash history TUI with filtering, sorting, multi-select, and clipboard copy

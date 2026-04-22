# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Local release packaging script (`scripts/release_local.sh`)
- Installer script for GitHub Releases (`install.sh`)
- Target detection helper (`scripts/print_target.sh`)
- Distribution documentation and release checklists
- Bash, Zsh, and Fish history parsing with configurable format selection
- Timestamp-aware history entries, timestamp sorting, and time-based search filters
- `Enter` accept flow that writes selected/current commands to stdout for shell integration

## [0.1.0] - 2026-02-21

### Added
- Initial `hline` release: bash history TUI with filtering, sorting, multi-select, and clipboard copy

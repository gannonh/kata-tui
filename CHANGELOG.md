# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-01-25

### Added

- **Terminal Dashboard** - View Kata `.planning/` files as a navigable tree structure
- **Two-Pane Layout** - Tree view (30%) and detail pane (70%) with responsive sizing
- **Keyboard Navigation** - Vim-style (j/k/h/l) and arrow key navigation
- **Focus Indicators** - Cyan border shows active pane, Tab switches between panes
- **Planning Data Parser** - Parses PROJECT.md, ROADMAP.md, and STATE.md
- **TEA Architecture** - Elm-style state management (Model/Update/View)
- **Safe Terminal Handling** - RAII cleanup and panic hook ensures terminal restoration
- **CLI Interface** - `--planning-dir` option to specify custom .planning directory

### Technical Details

- Built with Rust, Ratatui 0.28, and Crossterm 0.28
- Async event loop using Tokio
- 18 source files, ~1,650 lines of Rust
- 11 unit tests passing
- Minimum terminal size: 60x16

### Requirements Completed

- DISP-01: View .planning/ files parsed into structured data
- DISP-02: Project hierarchy in tree view
- DISP-03: Detailed content in detail pane
- NAV-01: Keyboard navigation (vim-style + arrows)
- NAV-02: Visual focus indicators
- PLAT-01: macOS support
- PLAT-02: Linux support

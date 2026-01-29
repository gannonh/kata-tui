# kata-tui

⚠️ In early dev, not yet usable 

A terminal dashboard for [Kata](https://github.com/anthropics/kata) project visibility.

## Overview

kata-tui provides real-time visibility into your Kata project status — phases, milestones, plans, and requirements — all in one terminal dashboard.

```
┌─ Project ──────────┬─ Phase 1: Foundation ────────────────┐
│ ▼ kata-tui         │                                      │
│   ▼ Phase 1 [x]    │ Goal: User can launch the TUI and    │
│     DISP-01 [x]    │ see their .planning/ files as        │
│     DISP-02 [x]    │ structured, navigable data.          │
│     DISP-03 [x]    │                                      │
│     NAV-01 [x]     │ Requirements:                        │
│     NAV-02 [x]     │ • DISP-01: View .planning/ files     │
│   ▶ Phase 2 [ ]    │ • DISP-02: Project hierarchy tree    │
│   ▶ Phase 3 [ ]    │ • DISP-03: Detail pane content       │
│   ▶ Phase 4 [ ]    │ • NAV-01: Keyboard navigation        │
│   ▶ Phase 5 [ ]    │ • NAV-02: Visual focus indicators    │
├────────────────────┴──────────────────────────────────────┤
│ [Tree] Phase 1 | Complete | q:quit j/k:nav Tab:switch     │
└───────────────────────────────────────────────────────────┘
```

## Installation

### From source

```bash
git clone https://github.com/gannonh/kata-tui.git
cd kata-tui
cargo build --release
```

### Via cargo (coming soon)

```bash
cargo install kata-tui
```

## Usage

Run in a directory containing a `.planning/` folder:

```bash
kata-tui
```

Or specify a custom path:

```bash
kata-tui --planning-dir /path/to/project/.planning
```

### Keybindings

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Collapse / Move left |
| `l` / `→` | Expand / Move right |
| `Tab` | Switch between panes |
| `q` / `Esc` | Quit |

## Features

### v0.1 (Current)

- Two-pane layout (tree view + detail pane)
- Parse and display PROJECT.md, ROADMAP.md, STATE.md
- Vim-style keyboard navigation
- Visual focus indicators
- macOS and Linux support

### Roadmap

- **Phase 2:** Color-coded status, progress bars, expand/collapse, help overlay, fuzzy search
- **Phase 3:** File watching, auto-refresh, markdown rendering
- **Phase 4:** Execute Kata commands from TUI with split-pane output
- **Phase 5:** Distribution via cargo install and GitHub releases

## Requirements

- Rust 1.70+
- Terminal with 60x16 minimum size
- macOS or Linux

## Development

```bash
# Run in development
cargo run

# Run tests
cargo test

# Build release
cargo build --release
```

## License

MIT

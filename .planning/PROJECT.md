# kata-tui

## What This Is

A terminal user interface dashboard for Kata projects that provides real-time visibility into project status — phases, milestones, plans, and recent activity — all in one glance.

## Current State

**v0.1 Foundation Preview** shipped 2026-01-25. Users can launch `kata-tui` and view their `.planning/` files as structured, navigable data with vim-style keyboard navigation.

**Tech stack:** Rust + Ratatui 0.28 + Crossterm 0.28 + Tokio
**Codebase:** ~1,650 LOC Rust across 18 source files, 11 tests

## Core Value

**Visibility.** Kata projects generate rich planning artifacts (.planning/ files) but there's no easy way to see the big picture. This dashboard surfaces that structure so users always know where they are, what's done, and what's next.

## The Problem

Currently checking Kata project status requires running commands and reading individual files. There's a visibility gap — users can't see their project status at a glance. This friction makes it harder to maintain awareness of project state, especially across sessions.

## The Solution

A real-time TUI dashboard that:
- Shows current phase progress (plan completion, success criteria)
- Displays all phases in a navigable sidebar
- Summarizes milestone progress and definition of done
- Tracks recent activity (commits, file changes)
- Updates automatically via file watching
- Lets users interact without leaving the terminal

## Target Users

Kata users — anyone using Kata for project planning and execution. This is a companion tool for the Kata ecosystem.

## Technical Approach

**Stack:**
- Rust for performance and single-binary distribution
- Ratatui for the TUI framework
- File system watching for real-time updates

**Platforms:**
- macOS (primary)
- Linux

**Distribution:**
- `cargo install kata-tui` from crates.io
- Prebuilt binaries from GitHub releases

## Key Features

### Display
- **Phase progress panel:** Plan completion status (done/in-progress/pending), success criteria checkboxes
- **Phase list sidebar:** All phases with status indicators, keyboard navigation
- **Milestone summary:** Overall progress, definition of done status
- **Recent activity:** Last commits, file changes, timestamps

### Interaction
- Navigate between phases (arrow keys)
- Expand/collapse plan details and criteria
- Copy Kata commands to clipboard
- Launch Kata commands with split view (dashboard stays visible, command output in pane)

### Real-time
- File watching on .planning/ directory
- Instant updates when state changes

## Constraints

- Must parse existing Kata .planning/ file formats (PROJECT.md, ROADMAP.md, STATE.md, etc.)
- No Windows support required for v1
- Should work in standard terminal emulators (iTerm2, Terminal.app, common Linux terminals)

## Non-Goals

- Not a replacement for Kata commands — complements them
- Not a project management tool — just visualizes Kata state
- Not an editor — read-only view with command launching

## Requirements

### Validated

- DISP-01: User can view .planning/ files parsed into structured data — v0.1
- DISP-02: User can see project hierarchy in a tree view — v0.1
- DISP-03: User can view detailed content of selected item in a detail pane — v0.1
- NAV-01: User can navigate using keyboard (vim-style j/k/h/l and arrow keys) — v0.1
- NAV-02: User can see clear visual focus indicators on the active element — v0.1
- PLAT-01: Application works on macOS — v0.1
- PLAT-02: Application works on Linux — v0.1

### Active

- [ ] DISP-04: Color-coded status indicators for phases and milestones
- [ ] DISP-05: Progress bars showing completion percentages
- [ ] NAV-03: Expand/collapse tree nodes
- [ ] NAV-04: Help overlay with keybindings (? key)
- [ ] NAV-05: Fuzzy search/filter (/ key)
- [ ] REAL-01: File watching on .planning/ directory
- [ ] REAL-02: Auto-refresh on file changes
- [ ] REND-01: Markdown rendering in detail pane
- [ ] CMD-01: Execute Kata commands from TUI
- [ ] CMD-02: Command output in split-pane layout
- [ ] DIST-01: Cargo install distribution
- [ ] DIST-02: Binary releases via GitHub

### Out of Scope

- Windows support — focus on macOS/Linux for v1
- Editing project files — this is a viewer, not an editor
- Multi-project view — one project at a time
- Remote project monitoring — local .planning/ only

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + Ratatui | Fast, single binary, modern TUI ecosystem, good cross-platform support | Implemented |
| TEA architecture | Predictable state management, ratatui recommendation | Implemented |
| Tokio async runtime | Non-blocking file watching and event handling | Implemented |
| Two-pane layout (Phase 1) | Simpler; output pane not needed until command execution | Implemented |
| 30/70 pane split | Standard dashboard ratio | Implemented |
| Semantic tree hierarchy | Users care about phases/requirements, not file names | Implemented |
| Auto-select current phase | Immediate context at launch | Implemented |
| Minimum terminal 60x16 | Graceful handling with friendly message | Implemented |
| File watching over polling | Instant updates, lower resource usage | — Pending (Phase 3) |
| Split view for commands | Keep context visible while running commands | — Pending (Phase 4) |
| No Windows v1 | Reduce scope, target primary Kata user platforms | — Standing |

---
*Last updated: 2026-01-25 after v0.1 milestone*

# kata-tui

## What This Is

A terminal user interface dashboard for Kata projects that provides real-time visibility into project status — phases, milestones, plans, and recent activity — all in one glance.

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

(None yet — ship to validate)

### Active

- [ ] Display current phase progress with plan completion status
- [ ] Show success criteria checkboxes for current phase
- [ ] Phase list sidebar with status indicators
- [ ] Keyboard navigation between phases
- [ ] Milestone summary panel
- [ ] Recent activity display (commits, file changes)
- [ ] File watching for real-time updates
- [ ] Expand/collapse for plan details
- [ ] Copy Kata commands to clipboard
- [ ] Launch commands with split view output
- [ ] Parse Kata .planning/ file formats
- [ ] macOS support
- [ ] Linux support
- [ ] Cargo install distribution
- [ ] Binary releases via GitHub

### Out of Scope

- Windows support — focus on macOS/Linux for v1
- Editing project files — this is a viewer, not an editor
- Multi-project view — one project at a time
- Remote project monitoring — local .planning/ only

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Rust + Ratatui | Fast, single binary, modern TUI ecosystem, good cross-platform support | — Pending |
| File watching over polling | Instant updates, lower resource usage | — Pending |
| Split view for commands | Keep context visible while running commands | — Pending |
| No Windows v1 | Reduce scope, target primary Kata user platforms | — Pending |

---
*Last updated: 2026-01-25 after initialization*

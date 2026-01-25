# Roadmap

## Overview

kata-tui delivers a terminal dashboard for Kata project visibility in 5 phases. The roadmap progresses from core viewing capability (foundation + display) through real-time updates and command execution, culminating in distribution packaging. Each phase delivers a complete, verifiable capability.

## Phases

### Phase 1: Foundation & Core Display

**Goal:** User can launch the TUI and see their .planning/ files as structured, navigable data.

**Dependencies:** None (first phase)

**Requirements:**
- DISP-01: User can view .planning/ files parsed into structured data
- DISP-02: User can see project hierarchy in a tree view
- DISP-03: User can view detailed content of selected item in a detail pane
- NAV-01: User can navigate using keyboard (vim-style j/k/h/l and arrow keys)
- NAV-02: User can see clear visual focus indicators on the active element
- PLAT-01: Application works on macOS
- PLAT-02: Application works on Linux

**Success Criteria:**
1. User can run `kata-tui` in a project directory and see PROJECT.md, ROADMAP.md, STATE.md content
2. User can navigate between phases/milestones using j/k or arrow keys
3. User sees a highlighted border indicating which element has focus
4. User can view details of any selected item in a right-side pane
5. Application runs identically on macOS and Linux terminals

**Research Notes:** Phase 1 architecture is critical. Must implement terminal cleanup on panic, async event loop with TEA pattern, and platform-aware key handling from day one per PITFALLS.md.

---

### Phase 2: Enhanced Display & Navigation

**Goal:** User has a professional-grade navigation experience with visual status and quick access features.

**Dependencies:** Phase 1 (foundation and basic navigation)

**Requirements:**
- DISP-04: User can see color-coded status indicators for phases and milestones
- DISP-05: User can see progress bars showing completion percentages
- NAV-03: User can expand and collapse tree nodes
- NAV-04: User can view help with keybindings by pressing ?
- NAV-05: User can fuzzy search/filter items by pressing /

**Success Criteria:**
1. User sees green/yellow/red indicators next to phases based on completion status
2. User sees progress bars that fill based on requirement/criteria completion percentages
3. User can press Enter or right arrow to expand a phase, showing its plans and criteria
4. User can press ? to see a help overlay showing all keybindings
5. User can press / and type to filter the tree view to matching items

---

### Phase 3: Real-time Updates & Markdown Rendering

**Goal:** Dashboard reflects current project state automatically and displays formatted content.

**Dependencies:** Phase 1 (file parsing infrastructure)

**Requirements:**
- REAL-01: System watches .planning/ directory for file changes
- REAL-02: UI automatically refreshes when files change
- REND-01: User can see markdown content rendered with formatting in the detail pane

**Success Criteria:**
1. When user saves changes to STATE.md in another editor, the TUI updates within 500ms
2. User sees bold, italic, and code formatting preserved in the detail pane
3. User sees lists and headers with appropriate visual hierarchy in the detail pane

**Research Notes:** Must implement file watcher debouncing (100-500ms) to prevent event flooding per PITFALLS.md.

---

### Phase 4: Command Integration

**Goal:** User can execute Kata commands and see output without leaving the TUI.

**Dependencies:** Phase 1 (TEA architecture), Phase 3 (real-time infrastructure)

**Requirements:**
- CMD-01: User can execute Kata commands from the TUI
- CMD-02: User can see command output in a split-pane layout

**Success Criteria:**
1. User can press a key to run a Kata command (e.g., plan-phase, verify-phase)
2. User sees command output streaming in real-time in a bottom or right pane
3. User can continue navigating the dashboard while a command runs
4. User can see when a command completes (success/failure indicator)

**Research Notes:** High complexity phase. If interactive subprocess support is required (commands needing user input), use `/kata:research-phase` for PTY handling investigation. Current implementation assumes non-interactive stdout/stderr capture.

---

### Phase 5: Distribution & Packaging

**Goal:** Users can install kata-tui through standard Rust distribution channels.

**Dependencies:** All prior phases (complete, tested application)

**Requirements:**
- DIST-01: User can install via `cargo install kata-tui`
- DIST-02: User can download prebuilt binaries from GitHub releases

**Success Criteria:**
1. User can run `cargo install kata-tui` and have a working binary
2. User can download a prebuilt binary from GitHub releases for macOS/Linux
3. Binary size is reasonable (<20MB) and starts up in under 1 second

---

## Progress

| Phase | Name | Status | Requirements |
|-------|------|--------|--------------|
| 1 | Foundation & Core Display | Pending | DISP-01, DISP-02, DISP-03, NAV-01, NAV-02, PLAT-01, PLAT-02 |
| 2 | Enhanced Display & Navigation | Pending | DISP-04, DISP-05, NAV-03, NAV-04, NAV-05 |
| 3 | Real-time Updates & Markdown Rendering | Pending | REAL-01, REAL-02, REND-01 |
| 4 | Command Integration | Pending | CMD-01, CMD-02 |
| 5 | Distribution & Packaging | Pending | DIST-01, DIST-02 |

---
*Last updated: 2026-01-25*

# Roadmap

## Overview

kata-tui delivers a terminal dashboard for Kata project visibility. v0.1 shipped the foundation (Phase 1). v1.0 continues with enhanced navigation, real-time updates, command integration, and distribution.

## Milestones

- **v0.1 Foundation Preview** — Phase 1 (shipped 2026-01-25) [archived](milestones/v0.1-ROADMAP.md)
- **v1.0 MVP** — Phases 2-5 (in progress)

## Phases

<details>
<summary>v0.1 Foundation Preview (Phase 1) — SHIPPED 2026-01-25</summary>

- [x] Phase 1: Foundation & Core Display (5/5 plans) — completed 2026-01-25

</details>

### v1.0 MVP (Phases 2-5)

### Phase 2: Enhanced Display & Navigation

**Goal:** User has a professional-grade navigation experience with visual status and quick access features.

**Dependencies:** Phase 1 (foundation and basic navigation)

**Plans:** 5 plans

Plans:
- [ ] 02-01-PLAN.md — InputMode state + color-coded status indicators (DISP-04)
- [ ] 02-02-PLAN.md — Expand/collapse tree nodes (NAV-03)
- [ ] 02-03-PLAN.md — Progress bars for completion percentages (DISP-05)
- [ ] 02-04-PLAN.md — Help overlay with keybindings (NAV-04)
- [ ] 02-05-PLAN.md — Fuzzy search/filter (NAV-05)

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

**Research Notes:** High complexity phase. If interactive subprocess support is required (commands needing user input), use `/kata:research-phase` for PTY handling investigation.

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

| Phase | Milestone | Status | Requirements |
|-------|-----------|--------|--------------|
| 1 | v0.1 | Complete | DISP-01, DISP-02, DISP-03, NAV-01, NAV-02, PLAT-01, PLAT-02 |
| 2 | v1.0 | Planned | DISP-04, DISP-05, NAV-03, NAV-04, NAV-05 |
| 3 | v1.0 | Pending | REAL-01, REAL-02, REND-01 |
| 4 | v1.0 | Pending | CMD-01, CMD-02 |
| 5 | v1.0 | Pending | DIST-01, DIST-02 |

---
*Last updated: 2026-01-25*

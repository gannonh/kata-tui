# Requirements

## v1.0 Requirements (Phases 2-5)

> v0.1 requirements (Phase 1) archived to [milestones/v0.1-REQUIREMENTS.md](milestones/v0.1-REQUIREMENTS.md)

### Enhanced Display (Phase 2) ✓

- [x] **DISP-04**: User can see color-coded status indicators for phases and milestones
- [x] **DISP-05**: User can see progress bars showing completion percentages

### Enhanced Navigation (Phase 2) ✓

- [x] **NAV-03**: User can expand and collapse tree nodes
- [x] **NAV-04**: User can view help with keybindings by pressing ?
- [x] **NAV-05**: User can fuzzy search/filter items by pressing /

### Real-time (Phase 3)

- [ ] **REAL-01**: System watches .planning/ directory for file changes
- [ ] **REAL-02**: UI automatically refreshes when files change

### Rendering (Phase 3)

- [ ] **REND-01**: User can see markdown content rendered with formatting in the detail pane

### Command Integration (Phase 4)

- [ ] **CMD-01**: User can execute Kata commands from the TUI
- [ ] **CMD-02**: User can see command output in a split-pane layout

### Distribution (Phase 5)

- [ ] **DIST-01**: User can install via `cargo install kata-tui`
- [ ] **DIST-02**: User can download prebuilt binaries from GitHub releases

## v2 Requirements (Deferred)

- [ ] Copy commands to clipboard
- [ ] Command history (show/rerun previous commands)
- [ ] Theme support (light/dark color schemes)
- [ ] Mouse support (click to select, scroll wheel)
- [ ] Jump-to-file ($EDITOR integration)

## Out of Scope

- **Dependency graph view** - High complexity, visualization adds risk for v1
- **Full text editing** - Use $EDITOR; TUI is a viewer, not an editor
- **Database/persistent state** - .planning/ files are the source of truth
- **Network features** - Local project viewer only
- **Plugin system** - Know the domain first, consider for v2+
- **Windows support** - Focus on macOS/Linux for v1

## Traceability

| REQ-ID | Phase | Status |
|--------|-------|--------|
| DISP-04 | Phase 2 | Complete |
| DISP-05 | Phase 2 | Complete |
| NAV-03 | Phase 2 | Complete |
| NAV-04 | Phase 2 | Complete |
| NAV-05 | Phase 2 | Complete |
| REAL-01 | Phase 3 | Pending |
| REAL-02 | Phase 3 | Pending |
| REND-01 | Phase 3 | Pending |
| CMD-01 | Phase 4 | Pending |
| CMD-02 | Phase 4 | Pending |
| DIST-01 | Phase 5 | Pending |
| DIST-02 | Phase 5 | Pending |

---
*Last updated: 2026-01-25 — Phase 2 complete*

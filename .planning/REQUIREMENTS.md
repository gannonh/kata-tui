# Requirements

## v1 Requirements

### Core Display

- [ ] **DISP-01**: User can view .planning/ files parsed into structured data (PROJECT.md, ROADMAP.md, STATE.md)
- [ ] **DISP-02**: User can see project hierarchy in a tree view (phases/milestones/requirements)
- [ ] **DISP-03**: User can view detailed content of selected item in a detail pane
- [ ] **DISP-04**: User can see color-coded status indicators for phases and milestones
- [ ] **DISP-05**: User can see progress bars showing completion percentages

### Navigation

- [ ] **NAV-01**: User can navigate using keyboard (vim-style j/k/h/l and arrow keys)
- [ ] **NAV-02**: User can see clear visual focus indicators on the active element
- [ ] **NAV-03**: User can expand and collapse tree nodes
- [ ] **NAV-04**: User can view help with keybindings by pressing ?
- [ ] **NAV-05**: User can fuzzy search/filter items by pressing /

### Real-time

- [ ] **REAL-01**: System watches .planning/ directory for file changes
- [ ] **REAL-02**: UI automatically refreshes when files change

### Command Integration

- [ ] **CMD-01**: User can execute Kata commands from the TUI
- [ ] **CMD-02**: User can see command output in a split-pane layout

### Rendering

- [ ] **REND-01**: User can see markdown content rendered with formatting in the detail pane

### Platform Support

- [ ] **PLAT-01**: Application works on macOS
- [ ] **PLAT-02**: Application works on Linux

### Distribution

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
| DISP-01 | Phase 1 | Pending |
| DISP-02 | Phase 1 | Pending |
| DISP-03 | Phase 1 | Pending |
| DISP-04 | Phase 2 | Pending |
| DISP-05 | Phase 2 | Pending |
| NAV-01 | Phase 1 | Pending |
| NAV-02 | Phase 1 | Pending |
| NAV-03 | Phase 2 | Pending |
| NAV-04 | Phase 2 | Pending |
| NAV-05 | Phase 2 | Pending |
| REAL-01 | Phase 3 | Pending |
| REAL-02 | Phase 3 | Pending |
| CMD-01 | Phase 4 | Pending |
| CMD-02 | Phase 4 | Pending |
| REND-01 | Phase 3 | Pending |
| PLAT-01 | Phase 1 | Pending |
| PLAT-02 | Phase 1 | Pending |
| DIST-01 | Phase 5 | Pending |
| DIST-02 | Phase 5 | Pending |

---
*Last updated: 2026-01-25*

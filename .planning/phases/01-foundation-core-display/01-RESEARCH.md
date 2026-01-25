# Phase 1 Research: Foundation & Core Display

**Phase:** 01 - Foundation & Core Display
**Status:** Consolidated from project research
**Source:** .planning/research/ (STACK.md, ARCHITECTURE.md, PITFALLS.md, FEATURES.md, SUMMARY.md)

## Phase Goal

User can launch the TUI and see their .planning/ files as structured, navigable data.

## Requirements Covered

- DISP-01: User can view .planning/ files parsed into structured data
- DISP-02: User can see project hierarchy in a tree view
- DISP-03: User can view detailed content of selected item in a detail pane
- NAV-01: User can navigate using keyboard (vim-style j/k/h/l and arrow keys)
- NAV-02: User can see clear visual focus indicators on the active element
- PLAT-01: Application works on macOS
- PLAT-02: Application works on Linux

## Technology Stack (from STACK.md)

### Core Dependencies

```toml
[dependencies]
# TUI Framework
ratatui = "0.30"
crossterm = { version = "0.29", features = ["event-stream"] }

# Async Runtime
tokio = { version = "1.49", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"

# Markdown Parsing
pulldown-cmark = "0.13"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"

# Error Handling
color-eyre = "0.5"

# CLI
clap = { version = "4.5", features = ["derive"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tracing-appender = "0.2"

[dev-dependencies]
insta = "1"
```

### Key Technology Decisions

| Choice | Rationale |
|--------|-----------|
| ratatui 0.30 | De facto standard Rust TUI, active maintenance, comprehensive widgets |
| crossterm 0.29 | Cross-platform backend, async event streams |
| tokio 1.49 | Dominant async runtime, required for non-blocking events |
| pulldown-cmark 0.13 | Fast CommonMark parser, sufficient for .planning/ files |
| color-eyre 0.5 | Beautiful panic hooks for TUI apps |

## Architecture Pattern (from ARCHITECTURE.md)

### The Elm Architecture (TEA)

Phase 1 MUST implement TEA from the start:

```
Model (state) → Message (events) → Update (state transitions) → View (render) → Loop
```

### Component Structure

```
src/
├── main.rs           # Entry point, tokio runtime setup
├── app.rs            # App struct, main loop, lifecycle
├── state.rs          # AppState (Model), Message enum
├── update.rs         # Update function (state transitions)
├── view.rs           # View function (rendering coordination)
├── event.rs          # EventHandler, Event enum
├── terminal.rs       # Terminal wrapper, setup/teardown, panic hook
├── layout.rs         # Layout computation (30/70 split)
├── components/       # UI components
│   ├── mod.rs
│   ├── tree_view.rs  # Left pane: project hierarchy
│   ├── detail_pane.rs # Right pane: selected item content
│   └── status_bar.rs # Bottom: current state, help hints
└── data/             # Data models and parsing
    ├── mod.rs
    ├── project.rs    # Project struct from PROJECT.md
    ├── roadmap.rs    # Roadmap/Phase structs from ROADMAP.md
    ├── state.rs      # ProjectState from STATE.md
    └── parser.rs     # Markdown parsing with pulldown-cmark
```

### Data Flow

1. Events arrive from keyboard (crossterm EventStream)
2. Events converted to Messages (enum)
3. Update function mutates AppState based on Message
4. View function renders current AppState to terminal
5. Repeat

### Layout Specification

Two-pane layout with 30/70 split:

```
┌─────────────────┬─────────────────────────────────────────────┐
│                 │                                             │
│   Tree View     │         Detail Pane                         │
│   (30%)         │         (70%)                               │
│                 │                                             │
│   ▸ Phase 1     │  ## Phase 1: Foundation                    │
│     ├ DISP-01   │                                             │
│     ├ DISP-02   │  **Goal:** User can launch...              │
│     └ NAV-01    │                                             │
│   ▸ Phase 2     │  **Requirements:**                         │
│   ▸ Phase 3     │  - DISP-01: User can view...               │
│                 │                                             │
├─────────────────┴─────────────────────────────────────────────┤
│ Phase 1 | Foundation | q:quit j/k:nav ?:help                  │
└───────────────────────────────────────────────────────────────┘
```

Minimum terminal: 60×16 (show friendly message if too small)

## Critical Pitfalls for Phase 1 (from PITFALLS.md)

### Pitfall 1: Terminal State Not Restored on Panic

**MUST address FIRST before any other code.**

```rust
fn init_panic_hook() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Restore terminal FIRST
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen
        );
        // Now show the panic
        default_panic(info);
    }));
}
```

Plus RAII via Drop impl on Terminal wrapper.

### Pitfall 2: Blocking the Event Loop

**Never call blocking I/O in render loop.**

- Use `tokio::spawn` for file operations
- Use channels (`tokio::sync::mpsc`) for communication
- Keep render loop async

### Pitfall 5: Platform-Specific Key Events

**Always filter for KeyEventKind::Press only.**

```rust
if let Event::Key(key) = event {
    if key.kind == KeyEventKind::Press {
        // Handle key - cross-platform safe
    }
}
```

### Pitfall 7: State Management Complexity

**Use TEA pattern - single state location, no RefCell/Mutex fights.**

## Key Patterns to Implement

### 1. Terminal Wrapper with RAII

```rust
pub struct Terminal {
    inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.inner.backend_mut(), LeaveAlternateScreen);
    }
}
```

### 2. Async Event Handler

```rust
pub enum Event {
    Key(KeyEvent),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
}

impl EventHandler {
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
```

### 3. Message-Based Updates

```rust
pub enum Message {
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Select,
    Quit,
    Tick,
}
```

### 4. Stateful Widgets

```rust
// External state persists across renders
pub struct UIState {
    pub tree_state: ListState,
    pub scroll_state: ScrollbarState,
}
```

## .planning/ File Parsing

### Files to Parse

1. **PROJECT.md** - Project metadata
2. **ROADMAP.md** - Phases with requirements
3. **STATE.md** - Current progress

### Parser Strategy

- Use pulldown-cmark for markdown parsing
- Extract headers, lists, and content blocks
- Build semantic tree: Phases → Requirements → Details

### Data Models

```rust
pub struct Project {
    pub name: String,
    pub description: String,
}

pub struct Roadmap {
    pub phases: Vec<Phase>,
}

pub struct Phase {
    pub number: u8,
    pub name: String,
    pub goal: String,
    pub requirements: Vec<Requirement>,
    pub status: PhaseStatus,
}

pub struct Requirement {
    pub id: String,
    pub description: String,
    pub status: RequirementStatus,
}
```

## Navigation Implementation

### Keybindings

| Key | Action |
|-----|--------|
| j / ↓ | Navigate down |
| k / ↑ | Navigate up |
| h / ← | Collapse / Navigate left |
| l / → / Enter | Expand / Navigate right |
| q / Esc | Quit |

### Focus Model

- Single focus: either tree view or detail pane
- Tab / Shift+Tab to switch focus between panes
- Visual indicator: highlighted border on focused pane

## Testing Strategy

### Unit Tests

- Use `ratatui::backend::TestBackend` for widget rendering
- Test state transitions (update function)
- Test markdown parsing against sample files

### Integration Tests

- Snapshot tests with `insta` crate
- Test at various terminal sizes

## Success Criteria Validation

1. ✓ Run `kata-tui` in project directory → see PROJECT.md, ROADMAP.md, STATE.md content
2. ✓ Navigate between phases/milestones using j/k or arrow keys
3. ✓ See highlighted border indicating which element has focus
4. ✓ View details of any selected item in right-side pane
5. ✓ Application runs identically on macOS and Linux terminals

## Build Order

Based on dependencies:

1. **Terminal wrapper** - Setup/teardown, panic hook (FIRST)
2. **Basic layout** - 30/70 split computation
3. **Event handler** - Keyboard events with tokio
4. **State/Model** - AppState, Message enum
5. **Update function** - Navigation state transitions
6. **Data models** - Project, Roadmap, Phase, Requirement
7. **Markdown parser** - Parse .planning/ files
8. **Tree view component** - Left pane with selection
9. **Detail pane component** - Right pane content display
10. **Status bar** - Bottom bar with hints
11. **View composition** - Render all components
12. **Main loop** - Wire everything together

## Research Confidence

| Area | Confidence | Source |
|------|------------|--------|
| Stack selection | HIGH | Official docs, Jan 2026 releases |
| TEA architecture | HIGH | Ratatui official recommendation |
| Pitfall prevention | HIGH | Official docs + community experience |
| Layout patterns | HIGH | Standard ratatui patterns |
| Parsing approach | MEDIUM | Needs validation against actual files |

---
*Consolidated: 2026-01-25*

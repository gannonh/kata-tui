# Research Summary: kata-tui

**Project:** TUI Dashboard for Kata Project Planning
**Research Completed:** 2026-01-25
**Overall Confidence:** HIGH

## Executive Summary

kata-tui is a terminal user interface dashboard for viewing and managing Kata project planning files. Based on comprehensive research across stack selection, feature landscape, architecture patterns, and domain pitfalls, the recommended approach is clear: build on **ratatui 0.30** with **crossterm** backend, using **The Elm Architecture (TEA)** pattern for predictable state management, and **tokio** for async file watching and command execution.

The Rust TUI ecosystem has matured significantly around ratatui (the actively maintained fork of tui-rs). The stack is well-documented, battle-tested in production applications like GitUI, lazygit, and k9s, and follows established patterns that make TUI development straightforward. The primary technical risk is not the stack itself, but architectural decisions around async event handling and process output capture—both have well-documented solutions if implemented correctly from the start.

The key differentiator for kata-tui is **integrated command execution** with real-time output display. Most TUI viewers are read-only; the ability to run Kata commands and see output in a split pane transforms this from a viewer into a control center. This feature is high complexity but achievable using tokio's process handling with proper stdout/stderr streaming. The critical success factor is avoiding common pitfalls: blocking the event loop, failing to restore terminal state on panic, and file watcher event flooding.

## Key Findings

### From STACK.md: Recommended Technologies

**Core TUI Framework:**
- **ratatui 0.30** - The de facto standard Rust TUI framework. Sub-millisecond rendering, comprehensive widget library, actively maintained with strong community support.
- **crossterm 0.29** - Default backend for ratatui. Cross-platform terminal support with async event streams.

**Async Runtime:**
- **tokio 1.49** - Dominant async runtime. Required for file watching and non-blocking event handling. The ratatui ecosystem is built around tokio.
- **tokio-util 0.7** - CancellationToken for graceful shutdown.
- **futures 0.3** - Stream combinators for crossterm's EventStream.

**File System Watching:**
- **notify 8.2** - The only mature cross-platform file watching library in Rust. Uses native OS APIs (FSEvents on macOS, inotify on Linux).
- **notify-debouncer-mini 0.4** - Essential for preventing event flooding during rapid file changes.

**Markdown Parsing:**
- **pulldown-cmark 0.13** - Pull parser for CommonMark. Lower memory usage than AST-based parsers. Used by cargo doc. Sufficient for .planning/ markdown files.

**Supporting Stack:**
- **color-eyre 0.5** - Error reporting with beautiful panic hooks. Ratatui community standard.
- **clap 4.5** - CLI argument parsing with derive macros.
- **tracing 0.1** - Structured logging (TUI apps cannot log to stdout).
- **serde 1.0, serde_yaml 0.9, toml 0.8** - Serialization for YAML frontmatter and config files.

**Testing:**
- **insta 1.x** - Snapshot testing for TUI output (official ratatui recommendation).
- **ratatui TestBackend** - Built-in unit testing support.

**Critical Version Requirements:**
- Rust 1.85+ (edition 2024)
- All dependencies are current as of January 2026

### From FEATURES.md: What to Build

**Table Stakes (Must-Have for MVP):**
1. Keyboard navigation (vim-style hjkl + arrow keys)
2. Clear visual focus indicators (highlighted borders)
3. Quit command (q/Esc/Ctrl+C)
4. Help system (? key for keybindings)
5. Responsive rendering (<100ms)
6. Scrollable content (List/Table widgets)
7. Panel-based layout (lazygit pattern: left nav + right detail)
8. Status indicators (color-coded green/yellow/red)
9. Tree/hierarchical view (phases > milestones > requirements)
10. Real-time file watching (.planning/ directory monitoring)

**Differentiators (Competitive Advantage):**
1. **Integrated command execution pane** - Run Kata commands and see output in split view (HIGH complexity, key differentiator)
2. Markdown rendering in-TUI - Display plan content with formatting
3. Fuzzy search/filter - Command palette style navigation
4. Copy command to clipboard - Export commands for external use
5. Progress visualization - Progress bars for phase completion
6. Jump-to-file - Open source markdown in $EDITOR
7. Mouse support - Optional enhancement (everything keyboard-accessible)

**Anti-Features (Explicitly Avoid):**
- Full text editing (use $EDITOR instead)
- Mouse-required interactions (breaks SSH workflows)
- Blocking operations (async everything)
- Complex nested modals (keep UI flat)
- Automatic command execution (security risk)
- Over-customization (sensible defaults > config hell)
- Rich media support (terminal compatibility varies)
- Persistent state beyond filesystem (no separate database)
- Network features (local project viewer only)
- Plugin system for v1 (premature abstraction)

**MVP Scope (Phase 1):**
Focus on read-only dashboard with file watching. Defer command execution to Phase 2. This allows validating the core UX and architecture before tackling the complex PTY/process handling required for command output.

### From ARCHITECTURE.md: How to Structure It

**Recommended Architecture:** Hybrid approach combining:
1. **The Elm Architecture (TEA)** - Model (state) → Message (events) → Update (state transitions) → View (render)
2. **Component-based organization** - Encapsulated UI regions (ProjectPane, RoadmapPane, OutputPane, StatusBar)
3. **Async event handling** - Tokio-based event collection from multiple sources via channels

**Component Boundaries:**

| Component | Responsibility |
|-----------|----------------|
| **App** | Application lifecycle, main loop orchestration, terminal setup/teardown |
| **EventHandler** | Async event collection (keyboard, file, process, tick), event normalization |
| **State/Model** | Project data, UI state (focus, scroll positions), command output buffer |
| **View** | Layout computation, widget rendering, frame composition |
| **FileWatcher** | Monitor .planning/ directory, debounce changes, emit reload events |
| **ProcessRunner** | Spawn Kata commands, capture stdout/stderr, stream output |
| **MarkdownParser** | Parse PROJECT.md, ROADMAP.md, STATE.md, extract structured data |
| **UI Components** | ProjectPane, RoadmapPane, OutputPane, StatusBar (composed during render) |

**Data Flow (Unidirectional):**
```
Events (keyboard, file, process)
  → EventHandler
  → Message enum
  → Update function (mutates state)
  → View function (renders current state)
  → Terminal output
  → Loop
```

**Key Patterns to Follow:**
1. **TEA Pattern** - Separate Model/Update/View for predictable state management
2. **Async Event Handler** - Dedicated tokio task collecting events from multiple sources
3. **Constraint-Based Layout** - Ratatui's Layout system for responsive split-pane UI
4. **Stateful Widgets** - External ListState for scrolling/selection persistence
5. **Process Output Streaming** - Concurrent stdout/stderr readers to avoid deadlock

**Anti-Patterns to Avoid:**
1. Blocking the main loop (async everything)
2. Widget state in widgets (use StatefulWidget with external state)
3. Tight coupling between components (message-based communication)
4. Not restoring terminal state (RAII pattern with Drop + panic hook)
5. Polling instead of event-driven (use tokio::select!)

**Suggested Directory Structure:**
```
kata-tui/
├── src/
│   ├── main.rs           # Entry point, tokio runtime setup
│   ├── app.rs            # App struct, main loop, lifecycle
│   ├── state.rs          # AppState (Model), Message enum
│   ├── update.rs         # Update function (state transitions)
│   ├── view.rs           # View function (rendering coordination)
│   ├── event.rs          # EventHandler, Event enum
│   ├── terminal.rs       # Terminal wrapper, setup/teardown
│   ├── layout.rs         # Layout computation, adaptive layouts
│   ├── components/       # UI components (ProjectPane, etc.)
│   ├── data/             # Data models and parsing
│   ├── services/         # Background services (FileWatcher, ProcessRunner)
│   └── config.rs         # Configuration, keybindings
```

### From PITFALLS.md: What Can Go Wrong

**Critical Pitfalls (Require Rewrites if Missed):**

1. **Terminal State Not Restored on Panic** (Phase 1)
   - *What:* Panic while in raw mode leaves terminal corrupted
   - *Prevention:* Custom panic hook that restores terminal BEFORE default handler
   - *Pattern:* RAII Drop impl + std::panic::set_hook
   - *Detection:* First panic reveals this immediately

2. **Blocking the Event Loop** (Phase 1)
   - *What:* UI freezes during file parsing, unresponsive to keyboard
   - *Prevention:* tokio::spawn for all I/O, channels for communication
   - *Pattern:* Event-driven architecture with async/await
   - *Detection:* UI stutter, delayed keyboard response

3. **File Watcher Event Flooding** (Phase 2/3)
   - *What:* Multiple events per file save causes excessive re-parsing and flicker
   - *Prevention:* Use notify-debouncer-mini with 100-500ms delay
   - *Pattern:* Debounced events with deduplication
   - *Detection:* UI flickers during file editing, high CPU

4. **Process Output Capture Deadlock** (Phase 3/4)
   - *What:* Spawned commands hang when stdout/stderr buffers fill
   - *Prevention:* Concurrent async readers for both streams
   - *Pattern:* tokio::spawn separate tasks to drain stdout and stderr
   - *Detection:* Commands with large output freeze

**Moderate Pitfalls (Cause Delays/Technical Debt):**

5. **Platform-Specific Key Event Handling** (Phase 1)
   - Windows sends Press + Release, macOS/Linux send Press only
   - Filter for KeyEventKind::Press only

6. **Layout Constraint Non-Determinism** (Phase 2)
   - Cassowary solver returns arbitrary solutions when constraints conflict
   - Test at various terminal sizes, use Min(0) for remaining space

7. **State Management Complexity** (Phase 1)
   - RefCell/Mutex fights indicate architecture mismatch
   - Use TEA pattern with single state location

8. **Flickering During Render** (Phase 2)
   - Excessive redraws cause visual flicker
   - Dirty flag pattern: only redraw when state changes

**Minor Pitfalls (Annoying but Fixable):**

9. Unicode width miscalculation (use unicode-width crate)
10. Missing Nerd Font characters (document font requirements, provide ASCII fallbacks)
11. Testing difficulty (use TestBackend, separate business logic)
12. Markdown parsing edge cases (test with real .planning/ files)

## Implications for Roadmap

### Recommended Phase Structure

Based on component dependencies and risk mitigation, the roadmap should follow this structure:

**Phase 1: Foundation & Read-Only Viewer (4-6 weeks)**
- **What it delivers:** Functional TUI dashboard that displays .planning/ files with keyboard navigation
- **Features:** Terminal setup with panic hooks, TEA architecture, keyboard navigation, basic layout, markdown parsing, tree view of phases/milestones, detail pane, status bar, file watching with debouncing
- **Rationale:** Establishes core architecture correctly before adding complexity. Validates UX patterns. All critical pitfalls must be addressed here (terminal cleanup, async event loop, TEA pattern).
- **Pitfalls to avoid:** Terminal state restoration (#1), blocking event loop (#2), platform key events (#5), state management complexity (#7)
- **Research needs:** SKIP - well-documented patterns in ratatui docs

**Phase 2: Enhanced Navigation & UI Polish (2-3 weeks)**
- **What it delivers:** Professional-grade navigation experience with search and responsive layouts
- **Features:** Fuzzy search/filter, help system (?), responsive layouts for different terminal sizes, progress visualization, improved status indicators, theme support basics
- **Rationale:** Must come after foundation is solid. Builds on proven event handling and state management.
- **Pitfalls to avoid:** Layout constraints (#6), flickering (#8), Unicode width (#9)
- **Research needs:** SKIP - standard widget patterns

**Phase 3: Command Execution Integration (4-5 weeks)**
- **What it delivers:** Interactive control center for running Kata commands with real-time output
- **Features:** Split-pane layout with output view, process runner service, streaming command output (stdout/stderr), command status indicators, basic command palette, kill running commands
- **Rationale:** Highest complexity feature. Requires solid foundation from Phase 1. This is the key differentiator.
- **Pitfalls to avoid:** File watcher flooding (#3), process deadlock (#4)
- **Research needs:** MAY NEED /kata:research-phase if PTY/interactive process support is required. Current research assumes non-interactive commands.

**Phase 4: Advanced Features & Polish (2-3 weeks)**
- **What it delivers:** Production-ready with quality-of-life features
- **Features:** Copy to clipboard, jump-to-file ($EDITOR integration), command history, keyboard shortcuts customization, mouse support (optional), error recovery
- **Rationale:** Polish and convenience features after core functionality proven.
- **Pitfalls to avoid:** Platform-specific clipboard issues
- **Research needs:** SKIP - recipes available in ratatui docs

**Phase 5: Testing & Hardening (1-2 weeks)**
- **What it delivers:** Production-ready quality with comprehensive tests
- **Features:** Snapshot tests with insta, integration tests with TestBackend, cross-platform validation, performance testing with large .planning/ directories, documentation
- **Rationale:** Final validation before release.
- **Research needs:** SKIP - testing infrastructure established in Phase 1

### Dependency Chain

```
Phase 1 (Foundation)
    ↓
Phase 2 (Navigation) ← can partially overlap with Phase 1
    ↓
Phase 3 (Command Execution) ← MUST wait for Phase 1 completion
    ↓
Phase 4 (Polish) ← can overlap with Phase 3
    ↓
Phase 5 (Testing) ← can run in parallel with Phase 4
```

### Research Flags

**Phases that need deeper research:**
- **Phase 3** if interactive subprocess support is required (PTY handling, terminal emulation within TUI). Current research assumes kata commands are non-interactive and output to stdout/stderr. If commands need user input mid-execution, use `/kata:research-phase Phase-3-Command-Execution` to investigate vt100 crate, pty-process crate, or embedded-terminal patterns.

**Phases with well-documented patterns (skip research):**
- Phase 1 (Foundation) - TEA architecture is thoroughly documented in ratatui
- Phase 2 (Navigation) - Standard widget patterns
- Phase 4 (Polish) - Recipes available for clipboard, $EDITOR spawning
- Phase 5 (Testing) - TestBackend and insta well-documented

### Critical Success Factors

1. **Get Phase 1 architecture right** - Terminal cleanup, async event handling, TEA pattern. These are hard to retrofit.
2. **Test with real .planning/ files early** - Validate markdown parsing assumptions against actual data structure.
3. **Test on all target platforms** - macOS, Linux at minimum. Windows if in scope.
4. **Build command execution incrementally** - Start with simple non-interactive commands, validate stdout/stderr streaming works before adding complexity.
5. **Performance test with large projects** - What happens with 50+ phases, 200+ requirements? Test the scalability assumptions.

## Confidence Assessment

| Area | Confidence | Notes |
|------|-----------|-------|
| Stack Selection | HIGH | Ratatui is the clear choice. Well-documented, actively maintained, proven in production. All dependencies are mature. |
| Feature Landscape | HIGH | Table stakes validated against multiple successful TUI applications (lazygit, k9s, taskwarrior-tui). Differentiators clearly identified. |
| Architecture Patterns | HIGH | TEA pattern is official ratatui recommendation with extensive documentation and examples. Component boundaries are well-established. |
| Pitfall Awareness | HIGH | Critical pitfalls identified from official docs and community experience. Prevention strategies are known and documented. |

### Gaps to Address

**During Planning:**
1. **Kata command interface** - What commands will kata-tui execute? What are their output formats? Are they interactive or non-interactive? This affects Phase 3 complexity.
2. **.planning/ file structure** - Actual schema of PROJECT.md, ROADMAP.md, STATE.md needs validation. Current research assumes standard markdown with possible YAML frontmatter.
3. **Target platforms** - Confirm macOS and Linux support. Windows support decision affects testing scope.
4. **Performance requirements** - What's the maximum expected project size? This affects data structure choices and rendering optimizations.

**During Development:**
1. **Validate parser assumptions** - Test pulldown-cmark against real .planning/ files in Phase 1. If files use GFM extensions (tables, task lists, strikethrough), may need to switch to comrak.
2. **PTY requirements** - In Phase 3, validate if kata commands are truly non-interactive. If they need terminal input, requires deeper research into PTY handling.
3. **Cross-platform validation** - Test keyboard events and file watching on Linux early to catch platform differences.

## Sources

All research is based on high-confidence sources:

### Official Documentation
- [Ratatui Documentation](https://ratatui.rs/) - Framework patterns, widgets, recipes
- [Ratatui GitHub Releases](https://github.com/ratatui/ratatui/releases) - v0.30.0 verification
- [Crossterm GitHub Releases](https://github.com/crossterm-rs/crossterm/releases) - v0.29 verification
- [Tokio Documentation](https://docs.rs/tokio/) - Async runtime patterns
- [Notify Documentation](https://docs.rs/notify/) - File watching API
- [Pulldown-cmark GitHub](https://github.com/pulldown-cmark/pulldown-cmark) - Markdown parsing

### Real-World Examples
- [GitUI](https://github.com/gitui-org/gitui) - Async-first TUI architecture
- [lazygit](https://github.com/jesseduffield/lazygit) - Panel-based UI patterns
- [k9s](https://k9scli.io/) - Real-time monitoring patterns
- [taskwarrior-tui](https://kdheepak.com/taskwarrior-tui/) - Keybinding conventions
- [Turborepo TUI](https://deepwiki.com/vercel/turborepo/5.1-terminal-ui) - Split-pane command output

### Community Resources
- [Awesome Ratatui](https://github.com/ratatui/awesome-ratatui) - Ecosystem overview
- [Ratatui Templates](https://github.com/ratatui/templates) - Project starters
- [spotify-tui async improvements](https://keliris.dev/articles/improving-spotify-tui) - Async refactoring case study

## Next Steps

The research synthesis is complete. The roadmapper agent has clear guidance to:

1. Structure roadmap into 5 phases based on dependency chain
2. Prioritize Phase 1 foundation (terminal setup, TEA architecture, file parsing)
3. Defer command execution to Phase 3 (high complexity, needs solid base)
4. Flag Phase 3 for possible deeper research if PTY support is needed
5. Plan for cross-platform testing throughout
6. Validate .planning/ file structure assumptions early in Phase 1

**Ready for Requirements Definition.**

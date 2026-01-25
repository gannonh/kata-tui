# Feature Landscape

**Domain:** TUI Project Status Viewer (kata-tui)
**Researched:** 2026-01-25
**Confidence:** HIGH (verified via official docs, multiple TUI applications, community patterns)

## Table Stakes

Features users expect from a TUI dashboard. Missing = product feels incomplete or frustrating.

| Feature | Why Expected | Complexity | Notes |
|---------|-------------|------------|-------|
| Keyboard navigation (vim-style j/k/h/l) | Universal TUI convention; terminal users expect efficient keyboard-first interaction | Low | Arrow keys as fallback; hjkl for power users |
| Clear visual focus indicator | Users must always know which pane/item is selected; ambiguous focus breaks keyboard UX | Low | Highlighted borders, color contrast on active element |
| Quit command (q/Esc/Ctrl+C) | Terminal apps must have obvious exit; users panic without it | Low | q to quit, Esc to go back/cancel |
| Help system (? key) | Discoverability is essential; users need to learn keybindings | Low-Med | ? shows available commands; footer hints for common actions |
| Responsive rendering (<100ms) | Terminal users expect speed; sluggish TUIs feel broken | Med | Ratatui handles this well with immediate-mode rendering |
| Scrollable content | Project data will exceed screen height; users expect to scroll | Low | Built-in with ratatui List/Table widgets |
| Panel-based layout | Multi-pane UIs are the standard for dashboards (lazygit, k9s pattern) | Med | Left nav + right detail is a proven pattern |
| Status indicators | Users need at-a-glance understanding of project health | Low | Color-coded status (green/yellow/red), icons |
| Tree/hierarchical view | Projects have phases > milestones > requirements; natural hierarchy | Med | tui-tree-widget or custom; expand/collapse with Space/Enter |
| Real-time file watching | .planning/ files change; UI must update without restart | Med | notify crate for fs events; debounced refresh |

### Rationale for Table Stakes

Based on analysis of successful TUI applications ([lazygit](https://github.com/jesseduffield/lazygit), [k9s](https://k9scli.io/), [taskwarrior-tui](https://kdheepak.com/taskwarrior-tui/)), users have strong expectations:

1. **Keyboard-first**: Terminal users chose the terminal for speed. Mouse support is nice-to-have, not required.
2. **Instant feedback**: Sub-100ms response or users feel lag.
3. **Visual clarity**: Which pane has focus? What can I do here? Always answer these visually.
4. **Discoverability**: Help accessible via ?, keybinding hints in footer/header.

## Differentiators

Features that set kata-tui apart. Not expected, but create delight and competitive advantage.

| Feature | Value Proposition | Complexity | Notes |
|---------|------------------|------------|-------|
| Integrated command execution pane | Run phase commands without leaving TUI; see output in split pane | High | Requires embedded terminal/PTY handling; turborepo TUI does this |
| Copy command to clipboard | Quick export of commands for external use | Low-Med | Platform-specific clipboard (arboard crate); OSC 52 for remote |
| Markdown rendering in-TUI | Display plan/requirement content with formatting | Med | termimad or tui-markdown crate; syntax highlighting |
| Fuzzy search/filter | Quickly find phases, milestones, requirements by name | Med | nucleo or fuzzy-matcher crate; command palette style |
| Progress visualization | Visual progress bars for phase completion | Low | Built-in Gauge widget; aggregate requirement completion |
| Dependency graph view | Show requirement/milestone dependencies visually | High | Canvas widget or ASCII art; nice-to-have for v2 |
| Command history | Re-run previously executed commands | Med | Persist to file; show recent in picker |
| Theme support | Light/dark/custom color schemes | Low-Med | User preference; respect terminal colors |
| Mouse support | Click to select, scroll with wheel | Low | Crossterm handles this; optional enhancement |
| Jump-to-file | Open source markdown file in $EDITOR | Low | Spawn editor subprocess; useful for editing |

### Differentiator Analysis

The **integrated command execution pane** is the key differentiator for kata-tui. Most project viewers are read-only; executing commands inline transforms it from a viewer to a control center.

From [turborepo's TUI implementation](https://deepwiki.com/vercel/turborepo/5.1-terminal-ui): "split-screen interface with task list navigation on the left and detailed task output on the right" with "scrollable terminal output with ANSI color support."

This requires careful implementation:
- Virtual terminal to capture subprocess output
- ANSI escape code handling
- Frame isolation (child output must not escape its pane)

## Anti-Features

Features to explicitly NOT build. Common mistakes in this domain.

| Anti-Feature | Why Avoid | What to Do Instead |
|--------------|-----------|-------------------|
| Full text editing | Scope creep; editors exist; TUI editors are complex | Jump-to-file with $EDITOR |
| Mouse-required interactions | Alienates terminal power users; breaks SSH workflows | Everything keyboard-accessible; mouse as optional enhancement |
| Blocking operations | Freezing UI during file operations destroys UX | Async everything; show loading indicators |
| Complex nested modals | TUIs should feel flat and fast; modal stacking confuses | Single modal at a time; prefer inline editing |
| Automatic command execution | Security risk; user should explicitly trigger actions | Always require confirmation for commands with side effects |
| Over-customization | Too many config options overwhelms; diminishing returns | Sensible defaults; theme support; limited keybinding overrides |
| Rich media (images, video) | Terminal support varies wildly; creates compatibility hell | Stick to text, ASCII art, box-drawing characters |
| Persistent state beyond filesystem | Database adds complexity; .planning/ is the source of truth | Read from files; no separate DB |
| Network features | Scope creep; this is a local project viewer | Future consideration only if explicit demand |
| Plugin system (v1) | Premature abstraction; know the domain first | Build features directly; consider plugins for v2+ |

### Anti-Pattern Warnings

From [TUI development experience](https://p.janouch.name/article-tui.html):
- "Without understanding how terminals work behind the scenes... it becomes difficult to troubleshoot when problems arise"
- Color handling across terminals is inconsistent; don't rely solely on color for meaning
- Unicode assumptions can break on older terminals

From [BubbleTea patterns](https://taranveerbains.ca/blog/13-making-a-tui-with-go):
- "TUIs are state machines. Rendering is the easy part; correctness and UX consistency are the hard part"
- Goroutine anti-patterns apply to async in Rust too; use framework's event loop

## Feature Dependencies

```
Core Foundation (must build first)
    |
    +-- Markdown Parsing (.planning/ files)
    |       |
    |       +-- Tree View (phases/milestones/requirements)
    |       |       |
    |       |       +-- Expand/Collapse
    |       |       +-- Status Indicators
    |       |
    |       +-- Detail Pane (selected item content)
    |               |
    |               +-- Markdown Rendering (optional enhancement)
    |
    +-- Keyboard Navigation
    |       |
    |       +-- Help System (? key)
    |       +-- Fuzzy Search (/ key)
    |
    +-- File Watching
            |
            +-- Real-time Updates

Command Execution (phase 2+)
    |
    +-- Split Pane Layout
    |       |
    |       +-- Command Output Pane
    |               |
    |               +-- PTY/Virtual Terminal
    |               +-- ANSI Color Support
    |
    +-- Command Invocation
            |
            +-- Copy to Clipboard
            +-- Command History
```

## MVP Recommendation

For MVP (Phase 1), prioritize:

1. **Markdown parsing** - Core functionality; parse .planning/ structure
2. **Tree view with expand/collapse** - Navigate project hierarchy
3. **Detail pane** - Show selected item content
4. **Keyboard navigation** - vim-style j/k/h/l, Enter to select, Esc to back
5. **Help system** - ? shows keybindings
6. **File watching** - Auto-refresh on .planning/ changes
7. **Status indicators** - Color-coded phase/milestone status

Defer to Phase 2:
- **Command execution pane** - High complexity; requires PTY handling
- **Clipboard support** - Platform-specific; nice-to-have
- **Fuzzy search** - Enhancement once basic nav works
- **Markdown rendering** - Plain text is acceptable initially

Defer to Phase 3+:
- **Dependency graph** - Visualization is complex
- **Theme support** - Sensible defaults first
- **Command history** - After command execution works

## Complexity Budget

| Phase | Features | Estimated Complexity |
|-------|----------|---------------------|
| 1 (MVP) | Parse + Tree + Detail + Nav + Help + Watch | Medium |
| 2 | Command Execution + Split Pane + Clipboard | High |
| 3 | Fuzzy Search + Markdown Rendering + Themes | Medium |

## Sources

### High Confidence (Official Docs, Context7, GitHub)
- [Ratatui Widgets Documentation](https://ratatui.rs/concepts/widgets/) - Built-in widget capabilities
- [awesome-ratatui](https://github.com/ratatui/awesome-ratatui) - Third-party widgets ecosystem
- [lazygit](https://github.com/jesseduffield/lazygit) - Panel-based TUI reference implementation
- [k9s](https://k9scli.io/) - Real-time monitoring TUI patterns
- [taskwarrior-tui](https://kdheepak.com/taskwarrior-tui/keybindings/) - Keybinding conventions

### Medium Confidence (Verified WebSearch)
- [Turborepo TUI](https://deepwiki.com/vercel/turborepo/5.1-terminal-ui) - Split pane with command output pattern
- [termimad](https://lib.rs/crates/termimad) - Terminal markdown rendering
- [tui-tree-widget](https://crates.io/crates/tui-tree-widget) - Hierarchical data widget
- [bdui](https://github.com/assimelha/bdui) - Real-time file watching in TUI

### Low Confidence (Single Source, Needs Validation)
- Clipboard via OSC 52 for remote terminals (community pattern, verify compatibility)
- Virtual terminal embedding complexity estimates (varies by implementation)

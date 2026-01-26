# Phase 2: Enhanced Display & Navigation - Research

**Researched:** 2026-01-25
**Domain:** TUI widgets, interactive navigation, visual feedback
**Confidence:** HIGH

## Summary

Phase 2 enhances the existing TUI foundation with professional-grade visual feedback and navigation features. The research covers five main requirements: color-coded status indicators (DISP-04), progress bars (DISP-05), expandable/collapsible tree nodes (NAV-03), help overlay (NAV-04), and fuzzy search/filter (NAV-05).

The existing codebase already uses Ratatui 0.28 with a TEA architecture and a flat List-based tree representation. For expand/collapse functionality, there are two viable approaches: (1) use the `tui-tree-widget` crate (v0.24.0) which provides native tree functionality with `TreeItem` and `TreeState`, or (2) extend the existing flat list with manual expand/collapse state tracking. Given the current implementation already has a working tree-like display, **extending the existing approach** is recommended to avoid a major refactor.

For fuzzy search, the `nucleo-matcher` crate is the modern, high-performance choice used by helix-editor. For progress bars, Ratatui's built-in `Gauge` and `LineGauge` widgets are sufficient. The help overlay follows the standard Ratatui popup pattern using the `Clear` widget.

**Primary recommendation:** Incrementally enhance the existing flat-list tree with expand/collapse state, add `Gauge`/`LineGauge` for progress visualization, implement an input mode enum for search functionality, and use the standard popup pattern for help overlay.

## Standard Stack

The established libraries/tools for this phase:

### Core (Already in Cargo.toml)
| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| ratatui | 0.28 | TUI framework | Already in use, has Gauge/LineGauge widgets |
| crossterm | 0.28 | Terminal events | Already in use for keyboard input |

### New Dependencies
| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| nucleo-matcher | 0.3 | Fuzzy matching | NAV-05: search/filter functionality |

### Alternatives Considered
| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| nucleo-matcher | fuzzy-matcher | nucleo is 6x faster, better Unicode support |
| Manual expand/collapse | tui-tree-widget | tui-tree-widget requires major refactor of existing TreeItem implementation |
| Manual popup | tui-popup | Extra dependency for something easily built with Clear widget |

**Installation:**
```bash
cargo add nucleo-matcher@0.3
```

## Architecture Patterns

### Recommended Project Structure Additions
```
src/
├── state.rs              # MODIFY: Add InputMode enum, expand state
├── update.rs             # MODIFY: Handle new messages (toggle, search)
├── view.rs               # MODIFY: Render overlays conditionally
├── components/
│   ├── tree_view.rs      # MODIFY: Color indicators, expand/collapse icons
│   ├── help_overlay.rs   # NEW: Help keybinding display
│   └── search_input.rs   # NEW: Search input box with fuzzy matching
└── search.rs             # NEW: Nucleo fuzzy matcher integration
```

### Pattern 1: Input Mode State Machine

**What:** Enum-based mode tracking for normal navigation vs search input
**When to use:** When the same keys have different meanings in different contexts

```rust
// Source: Common TUI pattern, steam_tui, tui-realm
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
    Help,
}

pub struct AppState {
    pub input_mode: InputMode,
    pub search_query: String,
    pub search_results: Vec<usize>,  // Indices of matching items
    // ... existing fields
}
```

**Event handling pattern:**
```rust
pub fn key_to_message(key: KeyEvent, mode: InputMode) -> Option<Message> {
    match mode {
        InputMode::Normal => match key.code {
            KeyCode::Char('/') => Some(Message::EnterSearchMode),
            KeyCode::Char('?') => Some(Message::ShowHelp),
            // ... existing keybindings
        },
        InputMode::Search => match key.code {
            KeyCode::Esc => Some(Message::ExitSearchMode),
            KeyCode::Enter => Some(Message::ConfirmSearch),
            KeyCode::Char(c) => Some(Message::SearchInput(c)),
            KeyCode::Backspace => Some(Message::SearchBackspace),
            _ => None,
        },
        InputMode::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => Some(Message::HideHelp),
            _ => None,
        },
    }
}
```

### Pattern 2: Expand/Collapse State Tracking

**What:** Track which phases are expanded using a HashSet of phase numbers
**When to use:** Flat list with logical tree hierarchy (current implementation)

```rust
// Source: Manual state tracking pattern
use std::collections::HashSet;

pub struct AppState {
    pub expanded_phases: HashSet<u8>,  // Phase numbers that are expanded
    // ... existing fields
}

impl AppState {
    pub fn toggle_expansion(&mut self, phase_num: u8) {
        if self.expanded_phases.contains(&phase_num) {
            self.expanded_phases.remove(&phase_num);
        } else {
            self.expanded_phases.insert(phase_num);
        }
    }

    pub fn is_expanded(&self, phase_num: u8) -> bool {
        self.expanded_phases.contains(&phase_num)
    }
}
```

**Tree item filtering during build:**
```rust
pub fn build_tree_items(data: &PlanningData, expanded: &HashSet<u8>) -> Vec<TreeItem> {
    let mut items = Vec::new();

    if !data.project.name.is_empty() {
        items.push(TreeItem::Project(data.project.name.clone()));
    }

    for phase in &data.roadmap.phases {
        items.push(TreeItem::Phase(phase.clone()));

        // Only include requirements if phase is expanded
        if expanded.contains(&phase.number) {
            for req in &phase.requirements {
                items.push(TreeItem::Requirement {
                    phase_num: phase.number,
                    requirement: req.clone(),
                });
            }
        }
    }

    items
}
```

### Pattern 3: Popup/Overlay Rendering

**What:** Render widgets on top of existing content using Clear widget
**When to use:** Help dialogs, search input, confirmation modals

```rust
// Source: https://ratatui.rs/examples/apps/popup/
use ratatui::widgets::Clear;
use ratatui::layout::{Constraint, Flex, Layout};

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)])
        .flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)])
        .flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

fn render_help_overlay(frame: &mut Frame) {
    let area = popup_area(frame.area(), 60, 60);
    frame.render_widget(Clear, area);  // Clear background FIRST

    let help_text = Paragraph::new(vec![
        Line::from(Span::styled("Keybindings", Style::default().bold())),
        Line::from(""),
        Line::from("j/k or arrows  Navigate up/down"),
        Line::from("h/l or arrows  Navigate left/right"),
        Line::from("Enter          Expand/collapse"),
        Line::from("/              Search"),
        Line::from("?              This help"),
        Line::from("q/Esc          Quit"),
    ])
    .block(Block::bordered().title(" Help "));

    frame.render_widget(help_text, area);
}
```

### Pattern 4: Color-Coded Status Indicators

**What:** Use the Stylize trait for consistent status coloring
**When to use:** Any status-based visual feedback

```rust
// Source: https://docs.rs/ratatui/latest/ratatui/style/trait.Stylize.html
use ratatui::style::Stylize;

impl PhaseStatus {
    pub fn color(&self) -> Color {
        match self {
            PhaseStatus::Complete => Color::Green,
            PhaseStatus::InProgress => Color::Yellow,
            PhaseStatus::Pending => Color::DarkGray,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            PhaseStatus::Complete => "[x]",
            PhaseStatus::InProgress => "[~]",
            PhaseStatus::Pending => "[ ]",
        }
    }
}

// In tree_view.rs rendering:
let status_span = Span::styled(
    phase.status.icon(),
    Style::default().fg(phase.status.color())
);
```

### Pattern 5: Progress Bar with LineGauge

**What:** Thin progress bar showing completion percentage
**When to use:** Compact progress display inline with tree items

```rust
// Source: https://docs.rs/ratatui/latest/ratatui/widgets/struct.LineGauge.html
use ratatui::widgets::LineGauge;
use ratatui::symbols;

fn render_phase_progress(phase: &Phase) -> LineGauge {
    let ratio = phase.completion_percentage() / 100.0;

    LineGauge::default()
        .filled_style(Style::default().fg(phase.status.color()))
        .unfilled_style(Style::default().fg(Color::DarkGray))
        .filled_symbol(symbols::line::THICK_HORIZONTAL)
        .ratio(ratio as f64)
}
```

### Anti-Patterns to Avoid
- **Nested RefCell/Mutex:** Use TEA pattern with single mutable state location
- **Blocking in search:** Fuzzy matching should complete within single frame
- **Modal state without escape:** Every overlay MUST have a clear exit keybinding
- **Rebuilding tree on every render:** Cache tree_items, rebuild only when data or expand state changes

## Don't Hand-Roll

Problems that look simple but have existing solutions:

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| Fuzzy matching | Simple substring search | nucleo-matcher | Proper Smith-Waterman algorithm, scoring, 6x faster |
| Progress bar rendering | Manual Unicode blocks | Gauge/LineGauge | Handles edge cases, Unicode precision, styling |
| Color helpers | Manual Style::default().fg() chains | Stylize trait | "hello".red().bold() is cleaner than Style boilerplate |
| Centered popup | Manual Rect arithmetic | Layout::vertical/horizontal with Flex::Center | Handles edge cases, cleaner API |

**Key insight:** Ratatui's built-in widgets and traits handle many edge cases (Unicode width, terminal capabilities, color fallbacks) that are easy to get wrong when hand-rolling.

## Common Pitfalls

### Pitfall 1: Selection Index Out of Bounds After Collapse

**What goes wrong:** User collapses a phase while a child requirement is selected; index now points to invalid item
**Why it happens:** Expand/collapse changes the item count without updating selection
**How to avoid:** After any collapse operation, clamp selection to valid range
**Warning signs:** Panic on tree_items.get(selected_index)

```rust
// After collapsing or filtering:
if let Some(selected) = state.tree_state.selected() {
    if selected >= tree_items.len() {
        let new_idx = tree_items.len().saturating_sub(1);
        state.tree_state.select(Some(new_idx));
    }
}
```

### Pitfall 2: Key Events Consumed in Wrong Mode

**What goes wrong:** Pressing 'j' in search mode navigates instead of typing 'j'
**Why it happens:** Mode not checked before key handling
**How to avoid:** Always check InputMode FIRST before mapping keys to actions
**Warning signs:** Inconsistent behavior between modes

### Pitfall 3: Search Performance on Large Trees

**What goes wrong:** UI stutters when typing search queries
**Why it happens:** Rebuilding tree and re-matching on every keystroke
**How to avoid:** Debounce search input, or use incremental matching
**Warning signs:** Noticeable delay between keystroke and UI update

```rust
// Simple approach: only match when Enter pressed or after debounce
// Better: nucleo supports incremental updates via Nucleo struct (not nucleo-matcher)
```

### Pitfall 4: Help Overlay Captures Wrong Keys

**What goes wrong:** Help overlay doesn't close, or wrong keys work
**Why it happens:** Event handling falls through to normal mode
**How to avoid:** Help mode should be a distinct InputMode that handles its own exit
**Warning signs:** Can navigate tree while help is shown

### Pitfall 5: LineGauge Unicode in Limited Terminals

**What goes wrong:** Progress bars render as garbage characters
**Why it happens:** Terminal doesn't support required Unicode
**How to avoid:** Test with basic terminals, provide ASCII fallback
**Warning signs:** Box-drawing characters appear as question marks

## Code Examples

Verified patterns from official sources:

### Message Enum Extension
```rust
// Source: Existing pattern in update.rs + TEA documentation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    // Navigation (existing)
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
    Select,
    SwitchPane,
    ScrollUp,
    ScrollDown,

    // Expand/Collapse (new)
    ToggleExpand,  // Toggle current item expansion
    ExpandAll,
    CollapseAll,

    // Search (new)
    EnterSearchMode,
    ExitSearchMode,
    SearchInput(char),
    SearchBackspace,
    ConfirmSearch,
    ClearSearch,

    // Help (new)
    ShowHelp,
    HideHelp,

    // System
    Quit,
    Tick,
}
```

### Fuzzy Matching with Nucleo
```rust
// Source: https://docs.rs/nucleo-matcher/latest/nucleo_matcher/
use nucleo_matcher::{Config, Matcher, Nucleo};
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};

pub struct SearchState {
    matcher: Matcher,
    query: String,
}

impl SearchState {
    pub fn new() -> Self {
        Self {
            matcher: Matcher::new(Config::DEFAULT),
            query: String::new(),
        }
    }

    pub fn search(&mut self, items: &[TreeItem]) -> Vec<usize> {
        if self.query.is_empty() {
            return (0..items.len()).collect();
        }

        let pattern = Pattern::parse(
            &self.query,
            CaseMatching::Ignore,
            Normalization::Smart,
        );

        items.iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                let text = item.searchable_text();
                let mut buf = Vec::new();
                pattern.score(
                    nucleo_matcher::Utf32Str::new(&text, &mut buf),
                    &mut self.matcher,
                ).map(|_score| idx)
            })
            .collect()
    }
}

impl TreeItem {
    pub fn searchable_text(&self) -> String {
        match self {
            TreeItem::Project(name) => name.clone(),
            TreeItem::Phase(phase) => format!("Phase {} {}", phase.number, phase.name),
            TreeItem::Requirement { requirement, .. } => {
                format!("{} {}", requirement.id, requirement.description)
            }
        }
    }
}
```

### Status Bar with Search Query Display
```rust
// Source: Existing status_bar.rs pattern
impl<'a> Widget for StatusBar<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = match self.input_mode {
            InputMode::Normal => {
                Line::from(vec![
                    Span::raw("q:quit j/k:nav Enter:expand ?:help /:search"),
                ])
            }
            InputMode::Search => {
                Line::from(vec![
                    Span::styled("Search: ", Style::default().bold()),
                    Span::raw(&self.search_query),
                    Span::styled("_", Style::default().slow_blink()),  // Cursor
                ])
            }
            InputMode::Help => {
                Line::from(vec![
                    Span::styled("Press ? or Esc to close", Style::default().italic()),
                ])
            }
        };

        Paragraph::new(content)
            .style(Style::default().bg(Color::DarkGray))
            .render(area, buf);
    }
}
```

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| fuzzy-matcher | nucleo-matcher | 2024+ | 6x faster, better Unicode |
| Manual popup rect | Layout with Flex::Center | Ratatui 0.26 | Cleaner API |
| Style::default().fg() | Stylize trait shortcuts | Ratatui 0.22+ | More ergonomic |
| tui-tree-widget for simple trees | Flat List with expand state | Always valid | Less complexity for simple hierarchies |

**Deprecated/outdated:**
- tui-rs: Superseded by ratatui (unmaintained since 2022)
- ratatui-widgets standalone crate: Name reserved for internal use, use built-ins or tui-framework-experiment

## Open Questions

Things that couldn't be fully resolved:

1. **Unicode fallback strategy**
   - What we know: LineGauge uses Unicode box drawing by default
   - What's unclear: Best practice for detecting terminal Unicode support
   - Recommendation: Test on common terminals, provide ASCII fallback via conditional compilation or runtime config

2. **Search debouncing**
   - What we know: Nucleo is fast enough for most cases
   - What's unclear: At what tree size does per-keystroke search become noticeable?
   - Recommendation: Implement without debouncing initially, add if performance issues arise

## Sources

### Primary (HIGH confidence)
- [Ratatui Gauge docs](https://docs.rs/ratatui/latest/ratatui/widgets/struct.Gauge.html) - Progress bar API
- [Ratatui LineGauge docs](https://docs.rs/ratatui/latest/ratatui/widgets/struct.LineGauge.html) - Thin progress bar
- [Ratatui Stylize trait](https://docs.rs/ratatui/latest/ratatui/style/trait.Stylize.html) - Color shorthand methods
- [Ratatui popup example](https://ratatui.rs/examples/apps/popup/) - Overlay pattern with Clear widget
- [Ratatui TEA pattern](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/) - Message-based updates
- [tui-tree-widget docs](https://docs.rs/tui-tree-widget/latest/tui_tree_widget/) - TreeState and TreeItem API
- [nucleo-matcher docs](https://docs.rs/nucleo-matcher/latest/nucleo_matcher/) - Fuzzy matching API
- [helix-editor/nucleo GitHub](https://github.com/helix-editor/nucleo) - Performance benchmarks

### Secondary (MEDIUM confidence)
- [Ratatui event handling](https://ratatui.rs/concepts/event-handling/) - Mode-based key handling patterns
- [Ratatui FAQ](https://ratatui.rs/faq/) - Common issues and solutions

### Tertiary (LOW confidence)
- WebSearch results on TUI input modes - Pattern exists across multiple crates

## Metadata

**Confidence breakdown:**
- Standard stack: HIGH - Using existing dependencies plus well-documented nucleo-matcher
- Architecture: HIGH - Patterns are extensions of existing TEA implementation
- Pitfalls: HIGH - Based on common ratatui issues and logical analysis

**Research date:** 2026-01-25
**Valid until:** 2026-02-25 (30 days - Ratatui ecosystem is stable)

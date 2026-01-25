# Architecture Patterns

**Domain:** Rust TUI Dashboard Application (kata-tui)
**Researched:** 2026-01-25
**Confidence:** HIGH (verified via official Ratatui documentation, real-world examples)

## Recommended Architecture

kata-tui should use a **hybrid architecture** combining:
1. **The Elm Architecture (TEA)** for core application state and message flow
2. **Component-based organization** for encapsulating UI regions
3. **Async event handling** via Tokio for file watching and process management

This recommendation is based on:
- [Ratatui's official patterns documentation](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)
- [Ratatui component architecture guide](https://ratatui.rs/concepts/application-patterns/component-architecture/)
- Real-world implementations like [GitUI](https://github.com/gitui-org/gitui) which uses async-first design with component separation

### High-Level Diagram

```
+------------------------------------------------------------------+
|                         kata-tui Application                      |
+------------------------------------------------------------------+
|                                                                   |
|  +---------------------+     +------------------------------+     |
|  |    Event Handler    |     |        App State (Model)     |     |
|  |---------------------|     |------------------------------|     |
|  | - Keyboard events   |     | - Project data               |     |
|  | - File watch events |---->| - UI state (focus, scroll)   |     |
|  | - Process events    |     | - Command output buffer      |     |
|  | - Tick events       |     | - Navigation state           |     |
|  +---------------------+     +------------------------------+     |
|           |                              |                        |
|           v                              v                        |
|  +---------------------+     +------------------------------+     |
|  |   Message Dispatch  |     |      View (Render)           |     |
|  |---------------------|     |------------------------------|     |
|  | - Update routing    |     | - Layout computation         |     |
|  | - Action processing |     | - Widget rendering           |     |
|  | - State mutation    |     | - Component composition      |     |
|  +---------------------+     +------------------------------+     |
|                                                                   |
+------------------------------------------------------------------+
                              |
         +--------------------+--------------------+
         |                    |                    |
         v                    v                    v
+----------------+   +----------------+   +------------------+
|  File Watcher  |   | Process Runner |   | Markdown Parser  |
|----------------|   |----------------|   |------------------|
| - notify crate |   | - tokio::proc  |   | - pulldown-cmark |
| - .planning/*  |   | - Kata CLI     |   | - frontmatter    |
| - debounced    |   | - stdout/err   |   | - serde/yaml     |
+----------------+   +----------------+   +------------------+
```

### Component Boundaries

| Component | Responsibility | Communicates With |
|-----------|----------------|-------------------|
| **App** | Application lifecycle, main loop orchestration, terminal setup/teardown | All components via message passing |
| **EventHandler** | Async event collection (keyboard, file, process, tick), event normalization | App (sends events via channel) |
| **State/Model** | Project data, UI state (focus, scroll positions, selections), command output buffer | Update function (mutated by messages) |
| **View** | Layout computation, widget rendering, frame composition | State (reads for rendering) |
| **FileWatcher** | Monitor `.planning/` directory, debounce changes, emit reload events | EventHandler (sends file change events) |
| **ProcessRunner** | Spawn Kata commands, capture stdout/stderr, stream output | EventHandler (sends output events), State (stores output) |
| **MarkdownParser** | Parse PROJECT.md, ROADMAP.md, STATE.md, extract structured data | State (populates project data) |
| **UI Components** | Encapsulated UI regions (ProjectPane, RoadmapPane, OutputPane, StatusBar) | View (composed during render), State (read UI state) |

### Data Flow

```
1. EVENTS ENTER
   +-----------+     +-----------+     +-----------+
   | Keyboard  | --> |           |     |           |
   +-----------+     |           |     |           |
   +-----------+     |  Event    | --> |   App     |
   | File      | --> |  Handler  |     |   Loop    |
   +-----------+     |           |     |           |
   +-----------+     |           |     |           |
   | Process   | --> |           |     |           |
   +-----------+     +-----------+     +-----------+
                                            |
                                            v
2. STATE UPDATES                    +---------------+
                                    |    Update     |
                              +---->|   Function    |
                              |     +---------------+
                              |            |
   +------------+             |            v
   |  Message   |-------------+     +---------------+
   |   Enum     |                   |     State     |
   +------------+                   |    (Model)    |
   | - KeyPress |                   +---------------+
   | - FileChanged |                       |
   | - ProcessOutput |                     |
   | - Tick      |                         v
   | - Navigate  |              3. VIEW RENDERS
   | - Execute   |                  +---------------+
   +------------+                   |     View      |
                                    |   Function    |
                                    +---------------+
                                           |
                                           v
                                    +---------------+
                                    |   Terminal    |
                                    |    Output     |
                                    +---------------+
```

**Unidirectional flow:**
1. Events arrive from multiple sources (keyboard, file watcher, process output)
2. Events are converted to Messages and dispatched to Update function
3. Update function modifies State based on Message
4. View function renders current State to terminal
5. Repeat

## Patterns to Follow

### Pattern 1: The Elm Architecture (TEA)

**What:** Separate application into Model (state), Update (state transitions), View (rendering)
**When:** Core application structure, especially for state that affects multiple UI regions
**Why:** Predictable state management, easy debugging, clear data flow

**Implementation:**

```rust
// Model: All application state
#[derive(Debug, Default)]
pub struct AppState {
    // Project data from parsed markdown
    pub project: Option<Project>,
    pub roadmap: Option<Roadmap>,
    pub state: Option<ProjectState>,

    // UI state
    pub focus: Focus,
    pub scroll_positions: HashMap<Pane, usize>,
    pub selected_items: HashMap<Pane, usize>,

    // Command execution
    pub command_output: Vec<String>,
    pub command_running: bool,

    // App lifecycle
    pub running: bool,
}

// Messages: All possible actions
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    FocusNext,
    FocusPrevious,
    ScrollUp,
    ScrollDown,
    Select,

    // File changes
    FileChanged(PathBuf),
    ProjectReloaded(Project),
    RoadmapReloaded(Roadmap),

    // Commands
    ExecuteCommand(KataCommand),
    CommandOutput(String),
    CommandComplete(ExitStatus),

    // Lifecycle
    Tick,
    Quit,
}

// Update: Pure state transitions
pub fn update(state: &mut AppState, message: Message) -> Option<Message> {
    match message {
        Message::ScrollUp => {
            if let Some(pos) = state.scroll_positions.get_mut(&state.focus) {
                *pos = pos.saturating_sub(1);
            }
            None
        }
        Message::FileChanged(path) => {
            // Return message to trigger reload
            Some(Message::ReloadFile(path))
        }
        Message::Quit => {
            state.running = false;
            None
        }
        // ... other handlers
    }
}

// View: Render current state
pub fn view(state: &AppState, frame: &mut Frame) {
    let layout = compute_layout(frame.area());

    render_project_pane(state, frame, layout.project);
    render_roadmap_pane(state, frame, layout.roadmap);
    render_output_pane(state, frame, layout.output);
    render_status_bar(state, frame, layout.status);
}
```

**Sources:** [Ratatui TEA Guide](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)

### Pattern 2: Async Event Handler

**What:** Dedicated async task collecting events from multiple sources via channels
**When:** Applications with file watching, process output, or other async operations
**Why:** Non-blocking UI, responsive to multiple event sources

**Implementation:**

```rust
use tokio::sync::mpsc;
use crossterm::event::{Event as CrosstermEvent, EventStream};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use futures::StreamExt;

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    FileChanged(PathBuf),
    ProcessOutput(String),
    ProcessComplete(ExitStatus),
    Tick,
    Resize(u16, u16),
}

pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    // Keep handles alive
    _watcher: RecommendedWatcher,
}

impl EventHandler {
    pub fn new(watch_path: PathBuf) -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel();

        // Keyboard events task
        let tx_key = tx.clone();
        tokio::spawn(async move {
            let mut reader = EventStream::new();
            while let Some(event) = reader.next().await {
                if let Ok(CrosstermEvent::Key(key)) = event {
                    let _ = tx_key.send(Event::Key(key));
                }
            }
        });

        // File watcher
        let tx_file = tx.clone();
        let watcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                for path in event.paths {
                    let _ = tx_file.send(Event::FileChanged(path));
                }
            }
        })?;
        watcher.watch(&watch_path, RecursiveMode::Recursive)?;

        // Tick events
        let tx_tick = tx.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_millis(250));
            loop {
                interval.tick().await;
                if tx_tick.send(Event::Tick).is_err() {
                    break;
                }
            }
        });

        Ok(Self { rx, _watcher: watcher })
    }

    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}
```

**Sources:**
- [Ratatui Async Event Stream Tutorial](https://ratatui.rs/tutorials/counter-async-app/async-event-stream/)
- [Ratatui Event Handling](https://ratatui.rs/concepts/event-handling/)

### Pattern 3: Constraint-Based Responsive Layout

**What:** Use Ratatui's Layout with constraints for adaptive split-pane UI
**When:** Dashboard layouts that need to adapt to terminal size
**Why:** Automatic resizing, professional appearance, handles small terminals gracefully

**Implementation:**

```rust
use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub struct AppLayout {
    pub sidebar: Rect,      // Project info, navigation
    pub main: Rect,         // Roadmap/content view
    pub output: Rect,       // Command output pane
    pub status: Rect,       // Status bar
}

pub fn compute_layout(area: Rect) -> AppLayout {
    // Main vertical split: content area + status bar
    let [content_area, status] = Layout::vertical([
        Constraint::Min(10),     // Content takes remaining space
        Constraint::Length(1),   // Status bar is 1 line
    ]).areas(area);

    // Horizontal split: sidebar + main content
    let [left, right] = Layout::horizontal([
        Constraint::Percentage(30),  // Sidebar 30%
        Constraint::Percentage(70),  // Main content 70%
    ]).areas(content_area);

    // Vertical split of main content: view + output
    let [main, output] = Layout::vertical([
        Constraint::Percentage(60),  // Main view 60%
        Constraint::Percentage(40),  // Output pane 40%
    ]).areas(right);

    AppLayout {
        sidebar: left,
        main,
        output,
        status,
    }
}

// Adaptive layout based on terminal size
pub fn compute_adaptive_layout(area: Rect) -> AppLayout {
    if area.width < 80 {
        // Narrow terminal: stack vertically
        compute_narrow_layout(area)
    } else if area.height < 24 {
        // Short terminal: hide output pane
        compute_compact_layout(area)
    } else {
        compute_layout(area)
    }
}
```

**Sources:** [Ratatui Layout Concepts](https://ratatui.rs/concepts/layout/)

### Pattern 4: Stateful Widgets for Scrolling/Selection

**What:** Use StatefulWidget trait with external state for interactive elements
**When:** Lists, tables, scrollable content that needs to maintain position
**Why:** State persists across renders, enables keyboard navigation

**Implementation:**

```rust
use ratatui::widgets::{List, ListItem, ListState, StatefulWidget};

pub struct RoadmapPane {
    items: Vec<Phase>,
}

impl RoadmapPane {
    pub fn render(
        &self,
        frame: &mut Frame,
        area: Rect,
        state: &mut ListState,  // External state
    ) {
        let items: Vec<ListItem> = self.items
            .iter()
            .map(|phase| {
                let style = if phase.complete {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}: {}", phase.id, phase.name))
                    .style(style)
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Roadmap"))
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, state);
    }
}

// In AppState, maintain ListState for each scrollable region
pub struct UIState {
    pub roadmap_state: ListState,
    pub output_state: ListState,
    pub project_state: ListState,
}
```

**Sources:** [Ratatui Widgets Introduction](https://ratatui.rs/concepts/widgets/)

### Pattern 5: Process Output Streaming

**What:** Spawn external processes and stream output to UI in real-time
**When:** Executing Kata commands and displaying results
**Why:** Users see output as it happens, not just final result

**Implementation:**

```rust
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct ProcessRunner {
    event_tx: mpsc::UnboundedSender<Event>,
}

impl ProcessRunner {
    pub async fn run_command(&self, cmd: &str, args: &[&str]) -> Result<()> {
        let mut child = Command::new(cmd)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let tx = self.event_tx.clone();

        // Stream stdout
        let tx_out = tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx_out.send(Event::ProcessOutput(line));
            }
        });

        // Stream stderr
        let tx_err = tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stderr);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx_err.send(Event::ProcessOutput(line));
            }
        });

        // Wait for completion
        let status = child.wait().await?;
        tx.send(Event::ProcessComplete(status))?;

        Ok(())
    }
}
```

**Sources:** [Tokio Process Documentation](https://docs.rs/tokio/latest/tokio/process/index.html)

## Anti-Patterns to Avoid

### Anti-Pattern 1: Blocking the Main Loop

**What:** Performing long operations (file parsing, process execution) synchronously in the render loop
**Why bad:** UI freezes, no keyboard response, poor user experience
**Instead:** Use async tasks and channels; never block the main loop

```rust
// BAD: Blocks UI
fn handle_file_change(path: &Path) {
    let content = std::fs::read_to_string(path).unwrap(); // BLOCKS!
    let parsed = parse_markdown(&content);                // BLOCKS!
    // ...
}

// GOOD: Async with message passing
async fn handle_file_change(path: PathBuf, tx: mpsc::Sender<Message>) {
    tokio::spawn(async move {
        let content = tokio::fs::read_to_string(&path).await;
        if let Ok(content) = content {
            let parsed = parse_markdown(&content);
            let _ = tx.send(Message::ProjectReloaded(parsed)).await;
        }
    });
}
```

### Anti-Pattern 2: Widget State in Widgets

**What:** Storing scroll position, selection state inside widget structs
**Why bad:** State resets on each render (ratatui uses immediate mode rendering)
**Instead:** Use external state with StatefulWidget pattern

```rust
// BAD: State lost on each render
struct MyList {
    items: Vec<String>,
    selected: usize,  // This resets every frame!
}

// GOOD: External state persists
struct MyList {
    items: Vec<String>,
}
impl StatefulWidget for MyList {
    type State = ListState;  // External, persisted
    // ...
}
```

### Anti-Pattern 3: Tight Coupling Between Components

**What:** Components directly calling methods on each other
**Why bad:** Hard to test, difficult to modify, circular dependencies
**Instead:** Components communicate via messages through central dispatch

```rust
// BAD: Direct coupling
impl ProjectPane {
    fn on_select(&mut self, roadmap: &mut RoadmapPane) {
        roadmap.filter_by_project(self.selected_project);
    }
}

// GOOD: Message-based decoupling
impl ProjectPane {
    fn on_select(&self) -> Message {
        Message::ProjectSelected(self.selected_project.clone())
    }
}
// Central update handles coordination
fn update(state: &mut AppState, msg: Message) {
    match msg {
        Message::ProjectSelected(project) => {
            state.selected_project = Some(project);
            // Update any dependent state here
        }
    }
}
```

### Anti-Pattern 4: Not Restoring Terminal State

**What:** Crashing or exiting without restoring terminal to normal mode
**Why bad:** Terminal left in raw mode, user's shell unusable
**Instead:** Use RAII pattern with proper cleanup in Drop

```rust
// GOOD: Terminal cleanup in Drop
pub struct Terminal {
    inner: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let inner = ratatui::Terminal::new(backend)?;
        Ok(Self { inner })
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.inner.backend_mut(), LeaveAlternateScreen);
    }
}
```

**Sources:** [Ratatui Spawn External Editor Recipe](https://ratatui.rs/recipes/apps/spawn-vim/)

### Anti-Pattern 5: Polling Instead of Event-Driven

**What:** Using busy loops to check for events
**Why bad:** High CPU usage, battery drain, poor responsiveness
**Instead:** Use async select! or blocking event reads with timeout

```rust
// BAD: Busy polling
loop {
    if crossterm::event::poll(Duration::ZERO)? {
        // handle event
    }
    // Wastes CPU cycles
}

// GOOD: Event-driven with select
loop {
    tokio::select! {
        event = event_handler.next() => {
            handle_event(event);
        }
        _ = shutdown_signal.recv() => {
            break;
        }
    }
}
```

## Suggested Directory Structure

```
kata-tui/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, tokio runtime setup
│   ├── app.rs               # App struct, main loop, lifecycle
│   ├── state.rs             # AppState (Model), Message enum
│   ├── update.rs            # Update function (state transitions)
│   ├── view.rs              # View function (rendering coordination)
│   ├── event.rs             # EventHandler, Event enum
│   ├── terminal.rs          # Terminal wrapper, setup/teardown
│   ├── layout.rs            # Layout computation, adaptive layouts
│   │
│   ├── components/          # UI components (encapsulated regions)
│   │   ├── mod.rs
│   │   ├── project_pane.rs  # Project info display
│   │   ├── roadmap_pane.rs  # Roadmap/phases list
│   │   ├── state_pane.rs    # Current state display
│   │   ├── output_pane.rs   # Command output view
│   │   └── status_bar.rs    # Bottom status bar
│   │
│   ├── data/                # Data models and parsing
│   │   ├── mod.rs
│   │   ├── project.rs       # Project struct from PROJECT.md
│   │   ├── roadmap.rs       # Roadmap/Phase structs from ROADMAP.md
│   │   ├── state.rs         # ProjectState from STATE.md
│   │   └── parser.rs        # Markdown parsing with pulldown-cmark
│   │
│   ├── services/            # Background services
│   │   ├── mod.rs
│   │   ├── file_watcher.rs  # File system monitoring
│   │   └── process.rs       # Kata command execution
│   │
│   └── config.rs            # Configuration, keybindings
│
└── tests/
    ├── integration/
    └── data/                # Test fixtures (.planning/ samples)
```

## Build Order Implications

Based on component dependencies, the suggested build order for phases:

### Phase 1: Foundation (No Dependencies)
1. **Terminal wrapper** (`terminal.rs`) - Setup/teardown, RAII cleanup
2. **Basic layout** (`layout.rs`) - Constraint-based layout computation
3. **Empty App shell** (`app.rs`) - Main loop skeleton that renders empty frame

**Rationale:** These have no dependencies on data or services. Establishes the render loop.

### Phase 2: Core Architecture (Depends on Phase 1)
1. **State/Model** (`state.rs`) - Define AppState, Message enum
2. **Update function** (`update.rs`) - Basic message handling (quit, navigation)
3. **Event handler** (`event.rs`) - Keyboard events only initially

**Rationale:** TEA architecture needs these three pieces working together. Keyboard events are simplest to start with.

### Phase 3: Data Layer (Depends on Phase 2)
1. **Markdown parser** (`data/parser.rs`) - Parse .planning/ files
2. **Data models** (`data/project.rs`, `data/roadmap.rs`, `data/state.rs`) - Structured data
3. **File watcher** (`services/file_watcher.rs`) - Monitor for changes

**Rationale:** Parser must exist before file watcher makes sense. Models depend on understanding file formats.

### Phase 4: UI Components (Depends on Phase 3)
1. **Project pane** - Display PROJECT.md data
2. **Roadmap pane** - Display ROADMAP.md with selection
3. **Status bar** - Show current state, help hints

**Rationale:** Components need data models to render meaningful content.

### Phase 5: Command Integration (Depends on Phase 4)
1. **Process runner** (`services/process.rs`) - Spawn Kata commands
2. **Output pane** - Display streaming command output
3. **Command palette** - Select/execute Kata commands

**Rationale:** Most complex feature, requires all other pieces to be working.

## Scalability Considerations

| Concern | Small Project | Large Project | Mitigation |
|---------|---------------|---------------|------------|
| File parsing | Instant | 100ms+ | Parse async, show loading indicator |
| Roadmap phases | All visible | 50+ phases | Virtual scrolling, lazy rendering |
| Command output | Small buffer | 10K+ lines | Ring buffer, limit stored lines |
| File watching | Few files | 100+ files | Debounce, batch updates |
| Memory usage | Negligible | Potential bloat | Clear old command output, limit history |

## Sources

### Official Documentation (HIGH Confidence)
- [Ratatui Official Documentation](https://ratatui.rs/)
- [Ratatui TEA Pattern](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)
- [Ratatui Component Architecture](https://ratatui.rs/concepts/application-patterns/component-architecture/)
- [Ratatui Layout Concepts](https://ratatui.rs/concepts/layout/)
- [Ratatui Event Handling](https://ratatui.rs/concepts/event-handling/)
- [Ratatui Widgets](https://ratatui.rs/concepts/widgets/)
- [Ratatui Async Event Stream](https://ratatui.rs/tutorials/counter-async-app/async-event-stream/)
- [Ratatui Spawn External Editor](https://ratatui.rs/recipes/apps/spawn-vim/)
- [Tokio Process Module](https://docs.rs/tokio/latest/tokio/process/index.html)

### Real-World Examples (HIGH Confidence)
- [GitUI](https://github.com/gitui-org/gitui) - Async-first Git TUI demonstrating performance patterns
- [Ratatui Templates](https://github.com/ratatui/templates) - Official project templates
- [TUI Core Framework](https://github.com/AstekGroup/tui-core) - Component-based architecture example

### Community Resources (MEDIUM Confidence)
- [tui-realm](https://github.com/veeso/tui-realm) - React/Elm inspired component framework
- [Ratatui Awesome List](https://github.com/ratatui/awesome-ratatui) - Curated TUI applications

# Domain Pitfalls

**Domain:** Rust TUI Dashboard Application (kata-tui)
**Researched:** 2026-01-25
**Confidence:** HIGH (verified via official ratatui documentation and community sources)

## Critical Pitfalls

Mistakes that cause rewrites, data loss, or major user-facing issues.

### Pitfall 1: Terminal State Not Restored on Panic

**What goes wrong:** Application panics while in raw mode, leaving the terminal in a corrupted state. User sees garbled output, can't type normally, and must restart their terminal session.

**Why it happens:** Raw mode disables terminal line buffering and echo. If the application panics without cleanup, the terminal remains in this state. The default panic handler doesn't know about terminal state.

**Consequences:**
- Terminal becomes unusable after crash
- Panic messages are unreadable (raw mode mangles newlines)
- Development iteration becomes painful (restart terminal every crash)
- User trust erodes if released software corrupts their terminal

**Prevention:**
1. Implement a custom panic hook that restores terminal state BEFORE the default handler runs
2. Use RAII pattern with a `Drop` impl as backup
3. Handle signals (SIGINT, SIGTERM) with the same cleanup

**Code pattern (Crossterm):**
```rust
fn init_panic_hook() {
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        // Restore terminal FIRST (don't panic here!)
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

**Detection:** First panic during development will reveal this issue immediately.

**Phase to address:** Phase 1 (Foundation) - Must be in place before any other development.

**Sources:**
- [Ratatui Panic Hooks Recipe](https://ratatui.rs/recipes/apps/panic-hooks/)
- [crossterm panic issues](https://github.com/crossterm-rs/crossterm/issues/368)

---

### Pitfall 2: Blocking the Event Loop

**What goes wrong:** UI freezes, becomes unresponsive to keyboard input. User thinks application has crashed. Long operations (file parsing, command execution) block rendering.

**Why it happens:**
- Awaiting network/file operations in the main render loop
- Synchronous I/O in async context
- Not spawning long-running tasks to background threads/tasks

**Consequences:**
- UI appears frozen during file system operations
- Keyboard input queues up and fires all at once
- Poor perceived performance even on fast machines
- For kata-tui specifically: parsing large `.planning/` directories could freeze UI

**Prevention:**
1. Use `tokio::spawn` for all I/O operations
2. Use channels (`tokio::sync::mpsc`) to communicate between background tasks and UI
3. Never call blocking I/O directly from the render/event loop
4. Use `tokio::task::spawn_blocking` for CPU-intensive work

**Architecture pattern:**
```
Main Thread: Event Loop + Render
  |
  +-- Background Task: File Watcher (notify crate)
  |     sends FileChanged events via channel
  |
  +-- Background Task: Command Executor
        sends CommandOutput events via channel
```

**Detection:**
- UI stutter when navigating to directories with many files
- Delay between keypress and visual response
- Input feels "queued up" after operations complete

**Phase to address:** Phase 1 (Foundation) - Event architecture must be correct from the start.

**Sources:**
- [Improving spotify-tui: going async](https://keliris.dev/articles/improving-spotify-tui)
- [Ratatui Async Event Stream](https://ratatui.rs/tutorials/counter-async-app/async-event-stream/)

---

### Pitfall 3: File Watcher Event Flooding

**What goes wrong:** Editing a file triggers duplicate events, causing UI to re-parse and re-render repeatedly. File saves trigger multiple events (write, modify, close). System becomes slow or stuttery during active editing.

**Why it happens:**
- `notify` crate fires multiple events for single file operations
- Different editors have different save behaviors (some use temp files)
- No debouncing of file events

**Consequences:**
- Excessive CPU usage during file editing
- UI flickers as it re-renders multiple times
- Potential race conditions if parsing isn't complete before next event
- For kata-tui: editing a plan file while viewing it causes visual chaos

**Prevention:**
1. Use debounced events: `notify-debouncer-mini` or `notify-debouncer-full`
2. Set appropriate debounce delay (100-500ms typically)
3. Deduplicate events by path before processing
4. Consider content hash comparison for large files

**Code pattern:**
```rust
use notify_debouncer_mini::{new_debouncer, DebouncedEventKind};
use std::time::Duration;

let (tx, rx) = std::sync::mpsc::channel();
let mut debouncer = new_debouncer(Duration::from_millis(200), tx)?;
debouncer.watcher().watch(path, RecursiveMode::Recursive)?;
```

**Detection:**
- Console logging shows multiple events per file save
- UI visibly flickers during editing
- High CPU during file operations

**Phase to address:** Phase 2/3 (File System Integration) - When implementing file watching.

**Sources:**
- [notify crate documentation](https://docs.rs/notify/)
- [notify duplicate events issue](https://users.rust-lang.org/t/problem-with-notify-crate-v6-1/99877)

---

### Pitfall 4: Process Output Capture Deadlock

**What goes wrong:** Spawned command hangs indefinitely. Application becomes unresponsive waiting for command output. Some commands work, others freeze.

**Why it happens:**
- `stdout` and `stderr` buffers fill up when both are piped
- Parent process waits for child to exit
- Child blocks on full buffer waiting for parent to read
- Classic producer-consumer deadlock

**Consequences:**
- External commands hang the entire application
- Commands that produce lots of output fail
- For kata-tui: running Claude or other commands in split view freezes

**Prevention:**
1. Read stdout/stderr asynchronously and concurrently
2. Use `tokio::process::Command` with async readers
3. Spawn separate tasks to drain each stream
4. Consider using `rust-subprocess` crate for complex scenarios

**Code pattern:**
```rust
use tokio::process::Command;
use tokio::io::{AsyncBufReadExt, BufReader};

let mut child = Command::new("your-command")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

let stdout = child.stdout.take().unwrap();
let stderr = child.stderr.take().unwrap();

// Spawn tasks to read both streams concurrently
let stdout_task = tokio::spawn(async move {
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        // Send to UI via channel
    }
    Ok::<_, std::io::Error>(())
});
```

**Detection:**
- Commands that produce output to both stdout and stderr hang
- Large output commands freeze
- Works fine with short/quiet commands

**Phase to address:** Phase 3/4 (Command Execution) - When implementing split-pane command output.

**Sources:**
- [Rust process deadlock discussion](https://users.rust-lang.org/t/display-and-capture-stdout-and-stderr-from-a-command/81296)
- [tokio::process documentation](https://docs.rs/tokio/latest/tokio/process/index.html)

---

## Moderate Pitfalls

Mistakes that cause delays, technical debt, or degraded user experience.

### Pitfall 5: Platform-Specific Key Event Handling

**What goes wrong:** Keyboard input works on macOS but not Linux, or vice versa. Some key combinations are intercepted by the OS. Windows users see double key presses.

**Why it happens:**
- Windows sends both `KeyEventKind::Press` AND `KeyEventKind::Release` events
- macOS/Linux only send `Press` events
- OS-level shortcuts (Ctrl+Up, Ctrl+1) may be intercepted
- Different terminal emulators have different behaviors

**Consequences:**
- Commands execute twice on Windows
- Key bindings that work in testing fail on user machines
- For kata-tui: navigation could be broken on certain platforms

**Prevention:**
1. Always filter for `KeyEventKind::Press` only
2. Test on all target platforms (at least macOS and Linux)
3. Avoid key combinations commonly used by OS/terminal
4. Document which terminal emulators are supported

**Code pattern:**
```rust
if let Event::Key(key) = event {
    if key.kind == KeyEventKind::Press {
        // Handle key - this is cross-platform safe
        match key.code {
            KeyCode::Char('q') => return Ok(()),
            // ...
        }
    }
}
```

**Detection:**
- User reports of double actions
- Key bindings that don't trigger on certain platforms
- Testing on different OSes reveals inconsistencies

**Phase to address:** Phase 1 (Foundation) - Event handling setup.

**Sources:**
- [Ratatui FAQ](https://ratatui.rs/faq/)
- [crossterm key event discussion](https://users.rust-lang.org/t/need-to-prevent-os-interception-of-some-keyboard-shortcuts-in-my-tui-rs-crossterm-app/107472)

---

### Pitfall 6: Layout Constraint Non-Determinism

**What goes wrong:** UI layout behaves unpredictably at certain terminal sizes. Widgets appear in wrong positions. Layout "jumps" when resizing terminal.

**Why it happens:**
- Ratatui's Cassowary constraint solver may return arbitrary solutions when constraints conflict
- The specific result is non-deterministic in edge cases
- Manual coordinate calculations go out of bounds
- Not handling the "remaining space" in layouts

**Consequences:**
- UI looks broken at certain terminal sizes
- Panics from out-of-bounds buffer access
- Poor responsive behavior during resize
- For kata-tui: dashboard could break on small terminals

**Prevention:**
1. Always test at various terminal sizes (especially small)
2. Add `Min(0)` as final constraint to consume remaining space
3. Use `area.intersection(buffer.area())` before rendering
4. Avoid manual coordinate calculations when possible
5. Handle `Resize` events to trigger re-layout

**Code pattern:**
```rust
// Always handle remaining space
let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),  // Header
        Constraint::Min(0),     // Content - fills remaining
        Constraint::Length(1),  // Footer
    ])
    .split(area);

// Safe bounds checking
let safe_area = area.intersection(frame.area());
frame.render_widget(widget, safe_area);
```

**Detection:**
- Visual glitches at specific terminal sizes
- Panics with "index out of bounds" in buffer operations
- Footer appearing in wrong position (known issue)

**Phase to address:** Phase 2 (UI Layout) - When implementing dashboard layout.

**Sources:**
- [Ratatui Layout Documentation](https://ratatui.rs/concepts/layout/)
- [Widget positioning breaks in large terminals](https://github.com/ratatui/ratatui/issues/2167)

---

### Pitfall 7: State Management Complexity with RefCell/Mutex

**What goes wrong:** Borrow checker fights become constant. Runtime panics from RefCell borrow conflicts. Deadlocks with Mutex. Code becomes convoluted with `Rc<RefCell<T>>` everywhere.

**Why it happens:**
- Trying to apply OOP patterns to Rust
- Not understanding immediate-mode rendering
- Sharing mutable state across components
- Fighting the ownership system instead of working with it

**Consequences:**
- Runtime panics instead of compile-time errors
- Complex, hard-to-maintain code
- Deadlock potential with Mutex
- For kata-tui: managing dashboard state across multiple views becomes painful

**Prevention:**
1. Use The Elm Architecture (TEA) pattern: Model + Message + Update
2. Keep state in a single location (the "model")
3. Avoid interior mutability unless truly necessary
4. Use channels for communication instead of shared state
5. Consider `tui-realm` if you want React-like components

**Architecture pattern:**
```rust
// TEA-style architecture
struct App {
    state: AppState,  // Single source of truth
}

enum Message {
    FileChanged(PathBuf),
    KeyPressed(KeyEvent),
    CommandOutput(String),
}

impl App {
    fn update(&mut self, msg: Message) {
        match msg {
            Message::FileChanged(path) => {
                // Pure state update
                self.state.reload_file(&path);
            }
            // ...
        }
    }

    fn view(&self, frame: &mut Frame) {
        // Pure rendering, no mutations
    }
}
```

**Detection:**
- `RefCell` borrow panics at runtime
- Convoluted code with multiple `Rc<RefCell<T>>` types
- Difficulty reasoning about state changes

**Phase to address:** Phase 1 (Foundation) - Architecture decision.

**Sources:**
- [The Elm Architecture in Ratatui](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)
- [tui-realm framework](https://github.com/veeso/tui-realm)

---

### Pitfall 8: Flickering During Render

**What goes wrong:** Screen flickers during updates. Visible "tearing" as widgets redraw. Worse in debug mode. Certain widgets (gauges, charts) flicker more.

**Why it happens:**
- Debug mode is significantly slower
- Excessive redraws (every event triggers render)
- Complex widgets take longer to compute
- Not using double-buffering correctly

**Consequences:**
- Unpleasant visual experience
- Users perceive app as low-quality
- For kata-tui: dashboard updates when files change could cause flicker

**Prevention:**
1. Always test in release mode for performance
2. Only redraw when state actually changes
3. Use terminal's alternate screen buffer
4. Avoid rendering on every tick - use dirty flag pattern
5. Consider frame rate limiting (e.g., 30fps max)

**Code pattern:**
```rust
struct App {
    dirty: bool,  // Track if redraw needed
}

// In event loop
match event {
    Event::Key(key) => {
        app.handle_key(key);
        app.dirty = true;
    }
    Event::Tick => {
        if app.dirty {
            terminal.draw(|f| app.view(f))?;
            app.dirty = false;
        }
    }
}
```

**Detection:**
- Visible flicker during operation
- Flicker worse in debug builds
- CPU usage higher than expected during idle

**Phase to address:** Phase 2 (UI/Rendering) - Core render loop implementation.

**Sources:**
- [Ratatui flickering debug discussion](https://forum.ratatui.rs/t/how-to-debug-flickering-in-my-app/106)
- [Cursive termion backend flickering](https://github.com/gyscos/Cursive/issues/142)

---

## Minor Pitfalls

Mistakes that cause annoyance but are easily fixable.

### Pitfall 9: Unicode Width Miscalculation

**What goes wrong:** Text alignment is off. Emojis and CJK characters break layout. Cursor position is wrong in text input fields.

**Why it happens:**
- Rust `String::len()` returns bytes, not display width
- Emojis and CJK characters are "wide" (2 cells)
- Some characters combine (ZWJ sequences)
- Different terminals handle Unicode differently

**Prevention:**
1. Use `unicode-width` crate for display width calculations
2. Use `unicode-segmentation` for proper grapheme handling
3. Consider `runefix-core` for CJK/emoji edge cases
4. Test with actual Unicode content

**Phase to address:** Phase 2 (UI) - Text rendering.

**Sources:**
- [runefix-core crate](https://crates.io/crates/runefix-core)
- [Terminal.Gui Unicode discussion](https://github.com/gui-cs/Terminal.Gui/discussions/2939)

---

### Pitfall 10: Missing Nerd Font Characters

**What goes wrong:** Box-drawing characters, icons, and symbols appear as replacement characters (rectangles). UI looks broken.

**Why it happens:**
- User's terminal font doesn't include required glyphs
- Different fonts have different Unicode coverage
- Nerd Font patches add many symbols regular fonts lack

**Prevention:**
1. Document font requirements (recommend Nerd Fonts)
2. Provide ASCII fallbacks for essential UI elements
3. Use only basic box-drawing characters for essential borders
4. Test with default terminal fonts

**Phase to address:** Any phase with custom characters - document in README.

**Sources:**
- [Ratatui FAQ - Missing Characters](https://ratatui.rs/faq/)

---

### Pitfall 11: Testing Difficulty

**What goes wrong:** No tests for TUI logic. Tests are flaky or hard to write. Can't test terminal-specific behavior.

**Why it happens:**
- TUI code seems "hard to test"
- Mixing rendering logic with business logic
- Not knowing about TestBackend

**Prevention:**
1. Use `TestBackend` for unit testing widgets
2. Separate business logic from rendering
3. Test state transitions independently
4. Use `ratatui-testlib` for integration tests
5. Consider snapshot testing with `insta`

**Phase to address:** Phase 1 (Foundation) - Set up test infrastructure early.

**Sources:**
- [TestBackend documentation](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html)
- [ratatui-testlib](https://lib.rs/crates/ratatui-testlib)

---

### Pitfall 12: Markdown Parsing Edge Cases

**What goes wrong:** Parser fails on valid markdown. Certain constructs cause panics. Output doesn't match expected rendering.

**Why it happens:**
- Markdown spec has many edge cases
- Custom/extended markdown syntax
- Parser library limitations

**Prevention:**
1. Use well-tested parser (`pulldown-cmark` is standard)
2. Handle parse errors gracefully
3. Test with real-world `.planning/` file samples
4. For kata-tui: focus on the subset of markdown actually used

**Note for kata-tui:** The `.planning/` files likely use a predictable subset of markdown. Focus on parsing what's actually needed rather than full CommonMark compliance.

**Phase to address:** Phase 2/3 (File Parsing) - When implementing markdown parsing.

**Sources:**
- [nom markdown parser discussion](https://developerlife.com/2024/06/28/md-parser-rust-from-r3bl-tui/)

---

## Phase-Specific Warnings

| Phase Topic | Likely Pitfall | Mitigation |
|-------------|----------------|------------|
| Foundation (Event Loop) | Blocking event loop | Use async from day 1, channels for background tasks |
| Foundation (Terminal) | Panic without cleanup | Implement panic hook FIRST before any other code |
| Foundation (Architecture) | RefCell/Mutex complexity | Choose TEA pattern, single state location |
| File Watching | Event flooding | Use debouncer, test with rapid file saves |
| UI Layout | Constraint non-determinism | Test at many terminal sizes, use Min(0) |
| Command Execution | Process deadlock | Async stdout/stderr, concurrent readers |
| Cross-platform | Key event differences | Filter KeyEventKind::Press, test on all platforms |
| Polish | Flickering | Dirty flag pattern, release mode testing |

## Research Flags for Later Phases

- **Phase with external process integration:** Deeper research on PTY handling may be needed if kata-tui needs interactive subprocess support
- **Phase with complex markdown:** If `.planning/` files use frontmatter or custom syntax, research specific parser requirements
- **Phase with mouse support:** Ratatui mouse handling has its own set of pitfalls not covered here

## Sources Summary

**HIGH Confidence (Official Documentation):**
- [Ratatui Documentation](https://ratatui.rs/)
- [Ratatui FAQ](https://ratatui.rs/faq/)
- [Ratatui Panic Hooks](https://ratatui.rs/recipes/apps/panic-hooks/)
- [Ratatui Async Events](https://ratatui.rs/tutorials/counter-async-app/async-event-stream/)
- [Ratatui Layout](https://ratatui.rs/concepts/layout/)
- [notify crate docs](https://docs.rs/notify/)
- [crossterm docs](https://docs.rs/crossterm/)
- [tokio::process docs](https://docs.rs/tokio/latest/tokio/process/index.html)

**MEDIUM Confidence (Community Sources):**
- [spotify-tui async article](https://keliris.dev/articles/improving-spotify-tui)
- [ratatui-testlib](https://lib.rs/crates/ratatui-testlib)
- [tui-realm framework](https://github.com/veeso/tui-realm)
- [Rust state machine patterns](https://hoverbear.org/blog/rust-state-machine-pattern/)

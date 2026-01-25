# Technology Stack

**Project:** kata-tui
**Researched:** 2026-01-25
**Overall Confidence:** HIGH

## Executive Summary

The 2025 Rust TUI ecosystem has consolidated around **ratatui** as the de facto standard framework, with **crossterm** as the preferred backend for cross-platform terminal support. For a project like kata-tui that needs file watching, markdown parsing, and keyboard navigation, this stack is mature, well-documented, and actively maintained.

## Recommended Stack

### Core TUI Framework

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [ratatui](https://github.com/ratatui/ratatui) | 0.30.0 | TUI framework | The actively maintained fork of tui-rs. Sub-millisecond rendering, zero-cost abstractions, comprehensive widget library (tabs, tables, scrollbars, layouts). Used by rust-analyzer, bottom, gitui. Released Dec 2024 with modularized architecture. | HIGH |
| [crossterm](https://github.com/crossterm-rs/crossterm) | 0.29 | Terminal backend | Default backend for ratatui. Cross-platform (macOS, Linux, Windows). Async event stream support. Released Apr 2025. | HIGH |

**Rationale:** Ratatui is the clear winner for Rust TUI in 2025. The original tui-rs is no longer maintained. Ratatui has an active community, regular releases, comprehensive documentation at [ratatui.rs](https://ratatui.rs/), and an ecosystem of widgets and extensions.

### Async Runtime

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [tokio](https://github.com/tokio-rs/tokio) | 1.49.0 | Async runtime | The dominant Rust async runtime. Required for async file watching and non-blocking event handling. async-std is discontinued; smol is lighter but has smaller ecosystem. Released Jan 2026. | HIGH |
| [tokio-util](https://crates.io/crates/tokio-util) | 0.7.x | Tokio utilities | CancellationToken for graceful shutdown, codec utilities. | HIGH |
| [futures](https://crates.io/crates/futures) | 0.3.x | Async abstractions | Required for crossterm's event-stream feature. Stream combinators. | HIGH |

**Rationale:** kata-tui needs async for file watching and non-blocking input. The ratatui ecosystem is built around tokio. Using `tokio::select!` with crossterm's EventStream is the recommended pattern for async TUI apps.

### File System Watching

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [notify](https://github.com/notify-rs/notify) | 8.2.0 | File system events | Cross-platform filesystem notification library. Used by cargo-watch, rust-analyzer, deno, mdBook. Supports debouncing. Released Aug 2025. | HIGH |
| [notify-debouncer-mini](https://crates.io/crates/notify-debouncer-mini) | 0.4.x | Event debouncing | Lightweight debouncer for notify. Prevents event flooding during rapid file changes. | HIGH |

**Rationale:** notify is the only mature cross-platform file watching solution in Rust. It uses native OS APIs (FSEvents on macOS, inotify on Linux). The debouncer is essential for markdown file editing where saves may trigger multiple events.

**Known Limitations:**
- Network filesystems (NFS, SMB) may not emit events
- Docker on macOS M1 requires PollWatcher fallback

### Markdown Parsing

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [pulldown-cmark](https://github.com/pulldown-cmark/pulldown-cmark) | 0.13.0 | Markdown parsing | Pull parser for CommonMark. Used by cargo doc. Lower memory usage than AST-based parsers. Fast. Released Feb 2025. | HIGH |

**Alternative Considered:**
- **comrak** (0.50.0): Full GFM support, AST-based. Better for GitHub-flavored markdown with all extensions. Used by crates.io, docs.rs, GitLab.

**Recommendation:** Use **pulldown-cmark** for kata-tui. The project parses `.planning/` markdown files which are likely standard CommonMark. pulldown-cmark is faster and uses less memory. If you need full GFM (task lists, strikethrough, tables), switch to comrak.

### Configuration & Serialization

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [serde](https://crates.io/crates/serde) | 1.0.x | Serialization framework | De facto standard. Required by nearly everything. | HIGH |
| [serde_yaml](https://crates.io/crates/serde_yaml) | 0.9.x | YAML parsing | For parsing YAML frontmatter in markdown files (if needed). | MEDIUM |
| [toml](https://crates.io/crates/toml) | 0.8.x | TOML parsing | For config files. TOML is idiomatic for Rust projects. | HIGH |

**Rationale:** kata-tui will parse markdown files that may contain YAML frontmatter. serde_yaml handles this. For any app configuration, use TOML (Rust ecosystem standard).

### Error Handling

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [color-eyre](https://github.com/eyre-rs/color-eyre) | 0.5.7 | Error reporting | Colorful, human-readable error reports. Excellent panic hooks. Recommended by ratatui documentation. | HIGH |

**Alternative Considered:**
- **anyhow**: Simpler, slightly faster. But color-eyre's panic hooks and colored output provide better UX for TUI apps.

**Rationale:** color-eyre is the ratatui community standard. It integrates well with TUI apps by providing beautiful error output when the terminal is restored after a crash.

### CLI Arguments

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [clap](https://github.com/clap-rs/clap) | 4.5.x | Argument parsing | Most popular CLI parser. Derive macro for ergonomic usage. Shell completions. | HIGH |

**Rationale:** kata-tui will likely accept arguments like `--path` or `--watch`. clap with derive feature is the standard choice.

### Logging & Debugging

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [tracing](https://crates.io/crates/tracing) | 0.1.x | Structured logging | Modern logging with spans. Better than log for async apps. | HIGH |
| [tracing-subscriber](https://crates.io/crates/tracing-subscriber) | 0.3.x | Log formatting | Format and filter tracing output. | HIGH |
| [tracing-appender](https://crates.io/crates/tracing-appender) | 0.2.x | File logging | Non-blocking file appender. TUI apps can't log to stdout. | HIGH |

**Alternative Considered:**
- **log4rs**: More configuration options, but tracing is more modern and better for async.
- **tui-logger**: Widget for displaying logs in the TUI itself. Consider adding later if needed.

**Rationale:** TUI apps cannot log to stdout (it would corrupt the display). tracing-appender writes to files. Use `RUST_LOG` environment variable for filtering.

### Testing

| Technology | Version | Purpose | Why | Confidence |
|------------|---------|---------|-----|------------|
| [insta](https://crates.io/crates/insta) | 1.x | Snapshot testing | Capture rendered TUI output as snapshots. Official ratatui recommendation. | HIGH |
| [ratatui TestBackend](https://docs.rs/ratatui/latest/ratatui/backend/struct.TestBackend.html) | (built-in) | Unit testing | Built into ratatui. Render to buffer for assertions. | HIGH |

**Rationale:** Snapshot testing with insta is the recommended approach for TUI testing. TestBackend allows rendering widgets without a real terminal.

## Complete Cargo.toml Dependencies

```toml
[package]
name = "kata-tui"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

[dependencies]
# TUI Framework
ratatui = "0.30"
crossterm = { version = "0.29", features = ["event-stream"] }

# Async Runtime
tokio = { version = "1.49", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"

# File Watching
notify = "8.2"
notify-debouncer-mini = "0.4"

# Markdown Parsing
pulldown-cmark = "0.13"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
toml = "0.8"

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

## What NOT to Use

| Technology | Why Not |
|------------|---------|
| **tui-rs** | Unmaintained since 2022. Use ratatui instead (direct fork). |
| **termion** | Less cross-platform than crossterm. No Windows support. |
| **async-std** | Discontinued as of March 2025. Use tokio. |
| **anyhow** | Works fine, but color-eyre provides better UX for TUI apps with colored panic output. |
| **log** (alone) | Use tracing instead. Better for async, structured logging. |
| **cursive** | Different paradigm (callback-based). Ratatui is more flexible and performant. |

## Architecture Recommendation

Based on the ratatui ecosystem, use **The Elm Architecture (TEA)** pattern:

```
Model (state) -> View (render) -> Message (input) -> Update (state change) -> Model
```

This pattern is:
- Recommended by ratatui documentation
- Well-suited for TUI apps with keyboard-driven interaction
- Easy to test (pure view functions, predictable state updates)
- Used by many production ratatui apps

For kata-tui specifically:
- **Model**: Project state (phases, milestones, plans loaded from `.planning/`)
- **View**: Split-pane layout with project tree + content panel
- **Message**: Keyboard events, file change events
- **Update**: Parse markdown, update state

## Sources

### Official Documentation (HIGH confidence)
- [Ratatui Documentation](https://ratatui.rs/)
- [Ratatui GitHub Releases](https://github.com/ratatui/ratatui/releases) - v0.30.0, Dec 2024
- [Crossterm GitHub Releases](https://github.com/crossterm-rs/crossterm/releases) - v0.29, Apr 2025
- [Notify GitHub Releases](https://github.com/notify-rs/notify/releases) - v8.2.0, Aug 2025
- [Tokio GitHub Releases](https://github.com/tokio-rs/tokio/releases) - v1.49.0, Jan 2026
- [Pulldown-cmark GitHub Releases](https://github.com/pulldown-cmark/pulldown-cmark/releases) - v0.13.0, Feb 2025

### Tutorials & Best Practices (MEDIUM confidence)
- [Ratatui Async Counter Tutorial](https://ratatui.rs/tutorials/counter-async-app/)
- [Ratatui Application Patterns](https://ratatui.rs/concepts/application-patterns/)
- [Ratatui Snapshot Testing](https://ratatui.rs/recipes/testing/snapshots/)
- [The Elm Architecture in Ratatui](https://ratatui.rs/concepts/application-patterns/the-elm-architecture/)

### Ecosystem Research (MEDIUM confidence)
- [From tui-rs to Ratatui](https://blog.orhun.dev/ratatui-0-23-0/) - History of the fork
- [State of Async Rust](https://corrode.dev/blog/async/) - async-std discontinuation
- [Awesome Ratatui](https://github.com/ratatui/awesome-ratatui) - Ecosystem overview

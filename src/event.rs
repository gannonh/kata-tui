use std::time::Duration;

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;

/// Application events
#[derive(Debug, Clone)]
pub enum Event {
    /// Keyboard input
    Key(KeyEvent),
    /// Terminal resize
    Resize(u16, u16),
    /// Periodic tick (for future animations/updates)
    Tick,
}

/// Async event handler
///
/// Spawns a background task that reads terminal events and sends them
/// through a channel. This keeps the main loop non-blocking.
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<Event>,
    // Keep handle to abort on drop
    _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate
    pub fn new(tick_rate: Duration) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();

        let task = tokio::spawn(async move {
            let mut reader = EventStream::new();
            let mut tick_interval = tokio::time::interval(tick_rate);

            loop {
                let tick_delay = tick_interval.tick();
                let crossterm_event = reader.next().fuse();

                tokio::select! {
                    _ = tick_delay => {
                        if tx.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                    maybe_event = crossterm_event => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                match event {
                                    CrosstermEvent::Key(key) => {
                                        // CRITICAL: Only handle Press events for cross-platform compatibility
                                        // Windows sends Press+Release, macOS/Linux may vary
                                        if key.kind == KeyEventKind::Press
                                            && tx.send(Event::Key(key)).is_err()
                                        {
                                            break;
                                        }
                                    }
                                    CrosstermEvent::Resize(w, h) => {
                                        if tx.send(Event::Resize(w, h)).is_err() {
                                            break;
                                        }
                                    }
                                    // Mouse, Paste, Focus events intentionally ignored
                                    CrosstermEvent::Mouse(_)
                                    | CrosstermEvent::Paste(_)
                                    | CrosstermEvent::FocusGained
                                    | CrosstermEvent::FocusLost => {}
                                }
                            }
                            Some(Err(e)) => {
                                eprintln!("Terminal event error: {:?}", e);
                                break;
                            }
                            None => break,
                        }
                    }
                }
            }
        });

        Self { rx, _task: task }
    }

    /// Get the next event, waiting asynchronously
    pub async fn next(&mut self) -> Option<Event> {
        self.rx.recv().await
    }
}

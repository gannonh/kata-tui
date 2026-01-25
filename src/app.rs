use std::path::PathBuf;
use std::time::Duration;

use color_eyre::Result;

use crate::components::tree_view::{build_tree_items, TreeItem};
use crate::data::{load_planning_data, PlanningData};
use crate::event::{Event, EventHandler};
use crate::state::AppState;
use crate::terminal::Terminal;
use crate::update::{key_to_message, update};
use crate::view::view;

/// Application state and lifecycle manager
pub struct App {
    /// Terminal wrapper
    terminal: Terminal,
    /// Application state
    state: AppState,
    /// Planning data from .planning/ files
    data: PlanningData,
    /// Flattened tree items for rendering
    tree_items: Vec<TreeItem>,
}

impl App {
    /// Create a new App instance
    ///
    /// Loads planning data from the specified directory (or current directory if None).
    pub fn new(planning_dir: Option<PathBuf>) -> Result<Self> {
        let terminal = Terminal::new()?;

        // Determine planning directory
        let dir = planning_dir.unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(".planning")
        });

        // Load planning data
        let data = load_planning_data(&dir)?;

        // Build tree items
        let tree_items = build_tree_items(&data);

        // Initialize state with correct tree bounds
        let mut state = AppState::new();

        // Ensure selection is valid
        if tree_items.is_empty() {
            state.tree_state.select(None);
        }

        Ok(Self {
            terminal,
            state,
            data,
            tree_items,
        })
    }

    /// Run the application main loop
    pub async fn run(&mut self) -> Result<()> {
        // Create event handler with 250ms tick rate
        let mut events = EventHandler::new(Duration::from_millis(250));

        // Main loop
        loop {
            // Render current state
            self.terminal.draw(|frame| {
                view(frame, &mut self.state, &self.data, &self.tree_items);
            })?;

            // Wait for next event
            if let Some(event) = events.next().await {
                match event {
                    Event::Key(key) => {
                        if let Some(message) = key_to_message(key) {
                            // Update state
                            update(&mut self.state, message);

                            // Clamp selection to valid range
                            if let Some(selected) = self.state.tree_state.selected() {
                                if selected >= self.tree_items.len() {
                                    self.state
                                        .tree_state
                                        .select(Some(self.tree_items.len().saturating_sub(1)));
                                }
                            }

                            // Check if we should quit
                            if self.state.should_quit {
                                break;
                            }
                        }
                    }
                    Event::Resize(_, _) => {
                        // Terminal resize - just redraw (handled by next loop iteration)
                    }
                    Event::Tick => {
                        // Periodic tick - could refresh data here in future
                    }
                }
            }
        }

        Ok(())
    }
}

/// Run the application
pub async fn run(planning_dir: Option<PathBuf>) -> Result<()> {
    let mut app = App::new(planning_dir)?;
    app.run().await
}

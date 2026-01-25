use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;

use color_eyre::Result;

use crate::components::tree_view::{build_tree_items, phases_with_requirements, TreeItem};
use crate::data::{load_planning_data, PlanningData};
use crate::event::{Event, EventHandler};
use crate::search::FuzzyMatcher;
use crate::state::{AppState, FocusedPane, InputMode, Message};
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
    /// Phases that have requirements (children)
    phases_with_children: HashSet<u8>,
    /// Fuzzy matcher for search
    fuzzy_matcher: FuzzyMatcher,
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

        // Determine which phases have children
        let phases_with_children = phases_with_requirements(&data);

        // Initialize state with correct tree bounds
        let mut state = AppState::new();

        // Build tree items with expanded state
        let tree_items = build_tree_items(&data, &state.expanded_phases);

        // Ensure selection is valid
        if tree_items.is_empty() {
            state.tree_state.select(None);
        }

        Ok(Self {
            terminal,
            state,
            data,
            tree_items,
            phases_with_children,
            fuzzy_matcher: FuzzyMatcher::new(),
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
                view(
                    frame,
                    &mut self.state,
                    &self.data,
                    &self.tree_items,
                    &self.phases_with_children,
                );
            })?;

            // Wait for next event
            if let Some(event) = events.next().await {
                match event {
                    Event::Key(key) => {
                        if let Some(mut message) = key_to_message(key, self.state.input_mode) {
                            // Convert Select/NavigateLeft/NavigateRight to ToggleExpand when on a Phase
                            message = self.maybe_convert_to_expand_message(message);

                            // Update state
                            let state_changed = update(&mut self.state, message);

                            // Rebuild tree items if expansion state might have changed
                            if state_changed {
                                self.tree_items =
                                    build_tree_items(&self.data, &self.state.expanded_phases);

                                // Clamp selection to valid range after rebuild
                                if let Some(selected) = self.state.tree_state.selected() {
                                    if selected >= self.tree_items.len() {
                                        let new_idx = self.tree_items.len().saturating_sub(1);
                                        self.state.tree_state.select(Some(new_idx));
                                        self.state.selected_index = new_idx;
                                    }
                                }

                                // Update search matches when in search mode
                                if self.state.input_mode == InputMode::Search {
                                    self.update_search_matches();
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

    /// Update search matches based on current query
    fn update_search_matches(&mut self) {
        if self.state.search_query.is_empty() {
            self.state.search_matches.clear();
            self.state.current_match = 0;
            return;
        }

        self.state.search_matches = self
            .tree_items
            .iter()
            .enumerate()
            .filter_map(|(idx, item)| {
                let text = Self::item_searchable_text(item);
                if self.fuzzy_matcher.matches(&self.state.search_query, &text) {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        // Clamp current_match to valid range
        if !self.state.search_matches.is_empty() {
            if self.state.current_match >= self.state.search_matches.len() {
                self.state.current_match = 0;
            }
            // Auto-select first match
            let match_idx = self.state.search_matches[self.state.current_match];
            self.state.tree_state.select(Some(match_idx));
            self.state.selected_index = match_idx;
        } else {
            self.state.current_match = 0;
        }
    }

    /// Get searchable text from a tree item
    fn item_searchable_text(item: &TreeItem) -> String {
        match item {
            TreeItem::Project(name) => name.clone(),
            TreeItem::Phase(phase) => format!("Phase {}: {}", phase.number, phase.name),
            TreeItem::Requirement { requirement, .. } => {
                format!("{}: {}", requirement.id, requirement.description)
            }
        }
    }

    /// Convert Select/NavigateLeft/NavigateRight to ToggleExpand when appropriate
    fn maybe_convert_to_expand_message(&self, message: Message) -> Message {
        // Only apply in Tree pane
        if self.state.focused_pane != FocusedPane::Tree {
            return message;
        }

        // Get the currently selected item
        let selected_idx = match self.state.tree_state.selected() {
            Some(idx) => idx,
            None => return message,
        };

        let selected_item = match self.tree_items.get(selected_idx) {
            Some(item) => item,
            None => return message,
        };

        // Check if selected item is a Phase
        let phase_num = match selected_item.phase_number() {
            Some(num) => num,
            None => return message,
        };

        // Only phases with children can be expanded
        if !self.phases_with_children.contains(&phase_num) {
            return message;
        }

        match message {
            Message::Select => Message::ToggleExpand(phase_num),
            Message::NavigateRight if !self.state.is_expanded(phase_num) => {
                Message::ToggleExpand(phase_num)
            }
            Message::NavigateLeft if self.state.is_expanded(phase_num) => {
                Message::ToggleExpand(phase_num)
            }
            _ => message,
        }
    }
}

/// Run the application
pub async fn run(planning_dir: Option<PathBuf>) -> Result<()> {
    let mut app = App::new(planning_dir)?;
    app.run().await
}

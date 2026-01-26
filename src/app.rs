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

        let dir = planning_dir.unwrap_or_else(|| {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(".planning")
        });

        let data = load_planning_data(&dir)?;
        let phases_with_children = phases_with_requirements(&data);
        let mut state = AppState::new();
        let tree_items = build_tree_items(&data, &state.expanded_phases);

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
        let mut events = EventHandler::new(Duration::from_millis(250));

        loop {
            self.terminal.draw(|frame| {
                view(
                    frame,
                    &mut self.state,
                    &self.data,
                    &self.tree_items,
                    &self.phases_with_children,
                );
            })?;

            if let Some(event) = events.next().await {
                match event {
                    Event::Key(key) => {
                        if let Some(mut message) = key_to_message(key, self.state.input_mode) {
                            // Convert navigation to expand/collapse when on expandable phases
                            message = self.maybe_convert_to_expand_message(message);

                            // Update state with current tree length for bounds checking
                            let state_changed =
                                update(&mut self.state, message, self.tree_items.len());

                            // Rebuild tree items if expansion state might have changed
                            if state_changed {
                                self.tree_items =
                                    build_tree_items(&self.data, &self.state.expanded_phases);

                                // Clamp selection to valid range after rebuild
                                self.clamp_selection_to_tree_bounds();

                                // Update search matches when in search mode (indices may have shifted)
                                if self.state.input_mode == InputMode::Search {
                                    self.update_search_matches();
                                }
                            }

                            if self.state.should_quit {
                                break;
                            }
                        }
                    }
                    Event::Resize(_, _) => {}
                    Event::Tick => {
                        // TODO(Phase 3): Refresh data from .planning/ files
                    }
                    Event::Error(e) => {
                        // Log error after terminal is restored (on drop)
                        eprintln!("Terminal event error: {}", e);
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Clamp selection to valid tree bounds after rebuild
    fn clamp_selection_to_tree_bounds(&mut self) {
        if self.tree_items.is_empty() {
            self.state.tree_state.select(None);
            self.state.selected_index = 0;
        } else if let Some(selected) = self.state.tree_state.selected() {
            if selected >= self.tree_items.len() {
                let new_idx = self.tree_items.len() - 1;
                self.state.tree_state.select(Some(new_idx));
                self.state.selected_index = new_idx;
            }
        }
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

        // Clamp current_match and auto-select
        if self.state.current_match >= self.state.search_matches.len() {
            self.state.current_match = 0;
        }
        if let Some(&match_idx) = self.state.search_matches.get(self.state.current_match) {
            self.state.tree_state.select(Some(match_idx));
            self.state.selected_index = match_idx;
        }
    }

    /// Get searchable text from a tree item.
    /// Projects use name, phases include number + name, requirements include ID + description.
    fn item_searchable_text(item: &TreeItem) -> String {
        match item {
            TreeItem::Project(name) => name.clone(),
            TreeItem::Phase(phase) => format!("Phase {}: {}", phase.number, phase.name),
            TreeItem::Requirement { requirement, .. } => {
                format!("{}: {}", requirement.id, requirement.description)
            }
        }
    }

    /// Convert navigation messages to ToggleExpand for expandable phases.
    /// Enter/Right expands collapsed phases, Left collapses expanded phases.
    fn maybe_convert_to_expand_message(&self, message: Message) -> Message {
        if self.state.focused_pane != FocusedPane::Tree {
            return message;
        }

        let selected_idx = match self.state.tree_state.selected() {
            Some(idx) => idx,
            None => return message,
        };

        let phase_num = match self
            .tree_items
            .get(selected_idx)
            .and_then(|item| item.phase_number())
        {
            Some(num) => num,
            None => return message,
        };

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

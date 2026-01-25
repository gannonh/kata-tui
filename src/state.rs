use std::collections::HashSet;

use ratatui::widgets::ListState;

/// Input mode for modal state (normal navigation vs search vs help)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    #[default]
    Normal,
    Search,
    Help,
}

/// Which pane currently has focus
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusedPane {
    #[default]
    Tree,
    Detail,
}

/// Application state - the single source of truth (TEA Model)
#[derive(Debug)]
pub struct AppState {
    /// Whether the application should quit
    pub should_quit: bool,

    /// Currently focused pane
    pub focused_pane: FocusedPane,

    /// Current input mode (normal navigation, search, help)
    pub input_mode: InputMode,

    /// Tree view selection state (which item is selected)
    pub tree_state: ListState,

    /// Index of selected item in flattened tree (for detail pane)
    pub selected_index: usize,

    /// Scroll offset for detail pane
    pub detail_scroll: u16,

    /// Which phase numbers are currently expanded (showing requirements)
    pub expanded_phases: HashSet<u8>,

    /// Current search query (empty when not searching)
    pub search_query: String,

    /// Indices of tree items that match the search query
    pub search_matches: Vec<usize>,

    /// Current match index (for cycling through matches)
    pub current_match: usize,
}

impl Default for AppState {
    fn default() -> Self {
        let mut tree_state = ListState::default();
        tree_state.select(Some(0)); // Select first item by default

        Self {
            should_quit: false,
            focused_pane: FocusedPane::Tree,
            input_mode: InputMode::Normal,
            tree_state,
            selected_index: 0,
            detail_scroll: 0,
            expanded_phases: HashSet::new(), // All collapsed initially
            search_query: String::new(),
            search_matches: Vec::new(),
            current_match: 0,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle expansion state for a phase
    pub fn toggle_expansion(&mut self, phase_num: u8) {
        if self.expanded_phases.contains(&phase_num) {
            self.expanded_phases.remove(&phase_num);
        } else {
            self.expanded_phases.insert(phase_num);
        }
    }

    /// Check if a phase is expanded
    pub fn is_expanded(&self, phase_num: u8) -> bool {
        self.expanded_phases.contains(&phase_num)
    }
}

/// Messages that can trigger state changes (TEA Message)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message {
    /// Navigate up in the current view
    NavigateUp,
    /// Navigate down in the current view
    NavigateDown,
    /// Navigate left (collapse tree node or switch to tree pane)
    NavigateLeft,
    /// Navigate right (expand tree node or switch to detail pane)
    NavigateRight,
    /// Select current item / expand
    Select,
    /// Switch focus between panes
    SwitchPane,
    /// Scroll detail pane up
    ScrollUp,
    /// Scroll detail pane down
    ScrollDown,
    /// Toggle expand/collapse of a phase
    ToggleExpand(u8),
    /// Show help overlay
    ShowHelp,
    /// Hide help overlay
    HideHelp,
    /// Enter search mode
    EnterSearchMode,
    /// Exit search mode
    ExitSearchMode,
    /// Character input for search
    SearchInput(char),
    /// Backspace in search
    SearchBackspace,
    /// Confirm search (navigate to match)
    ConfirmSearch,
    /// Navigate to next match
    NextMatch,
    /// Navigate to previous match
    PrevMatch,
    /// Quit the application
    Quit,
    /// Tick event for periodic updates (future use)
    Tick,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_expand_adds_to_set() {
        let mut state = AppState::new();
        assert!(!state.is_expanded(1));

        state.toggle_expansion(1);

        assert!(state.is_expanded(1));
    }

    #[test]
    fn test_toggle_expand_removes_from_set() {
        let mut state = AppState::new();
        state.toggle_expansion(1); // Add
        assert!(state.is_expanded(1));

        state.toggle_expansion(1); // Remove

        assert!(!state.is_expanded(1));
    }

    #[test]
    fn test_multiple_phases_can_be_expanded() {
        let mut state = AppState::new();

        state.toggle_expansion(1);
        state.toggle_expansion(2);

        assert!(state.is_expanded(1));
        assert!(state.is_expanded(2));
        assert!(!state.is_expanded(3));
    }
}

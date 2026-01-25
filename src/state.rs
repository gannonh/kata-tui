use ratatui::widgets::ListState;

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

    /// Tree view selection state (which item is selected)
    pub tree_state: ListState,

    /// Index of selected item in flattened tree (for detail pane)
    pub selected_index: usize,

    /// Scroll offset for detail pane
    pub detail_scroll: u16,
}

impl Default for AppState {
    fn default() -> Self {
        let mut tree_state = ListState::default();
        tree_state.select(Some(0)); // Select first item by default

        Self {
            should_quit: false,
            focused_pane: FocusedPane::Tree,
            tree_state,
            selected_index: 0,
            detail_scroll: 0,
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
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
    /// Quit the application
    Quit,
    /// Tick event for periodic updates (future use)
    Tick,
}

use crate::state::{AppState, FocusedPane, Message};

/// Maximum number of items in tree (will be dynamic later)
const MAX_TREE_ITEMS: usize = 10;

/// TEA Update function - handles all state transitions
///
/// Takes the current state and a message, returns whether state changed.
/// This is the ONLY place state mutations happen (TEA pattern).
pub fn update(state: &mut AppState, message: Message) -> bool {
    match message {
        Message::Quit => {
            state.should_quit = true;
            true
        }

        Message::NavigateUp => match state.focused_pane {
            FocusedPane::Tree => {
                let current = state.tree_state.selected().unwrap_or(0);
                if current > 0 {
                    state.tree_state.select(Some(current - 1));
                    state.selected_index = current - 1;
                    true
                } else {
                    false
                }
            }
            FocusedPane::Detail => {
                if state.detail_scroll > 0 {
                    state.detail_scroll = state.detail_scroll.saturating_sub(1);
                    true
                } else {
                    false
                }
            }
        },

        Message::NavigateDown => match state.focused_pane {
            FocusedPane::Tree => {
                let current = state.tree_state.selected().unwrap_or(0);
                // MAX_TREE_ITEMS is placeholder, will be replaced with actual data length
                if current < MAX_TREE_ITEMS.saturating_sub(1) {
                    state.tree_state.select(Some(current + 1));
                    state.selected_index = current + 1;
                    true
                } else {
                    false
                }
            }
            FocusedPane::Detail => {
                state.detail_scroll = state.detail_scroll.saturating_add(1);
                true
            }
        },

        Message::NavigateLeft => {
            if state.focused_pane == FocusedPane::Detail {
                state.focused_pane = FocusedPane::Tree;
                true
            } else {
                // In tree: would collapse node (future)
                false
            }
        }

        Message::NavigateRight => {
            if state.focused_pane == FocusedPane::Tree {
                state.focused_pane = FocusedPane::Detail;
                true
            } else {
                // In detail: no action
                false
            }
        }

        Message::Select => {
            // Enter key: expand/collapse in tree (future) or switch to detail
            if state.focused_pane == FocusedPane::Tree {
                state.focused_pane = FocusedPane::Detail;
                true
            } else {
                false
            }
        }

        Message::SwitchPane => {
            state.focused_pane = match state.focused_pane {
                FocusedPane::Tree => FocusedPane::Detail,
                FocusedPane::Detail => FocusedPane::Tree,
            };
            true
        }

        Message::ScrollUp => {
            if state.detail_scroll > 0 {
                state.detail_scroll = state.detail_scroll.saturating_sub(3);
                true
            } else {
                false
            }
        }

        Message::ScrollDown => {
            state.detail_scroll = state.detail_scroll.saturating_add(3);
            true
        }

        Message::Tick => {
            // Future: refresh data, animations
            false
        }
    }
}

/// Convert keyboard event to Message
pub fn key_to_message(key: crossterm::event::KeyEvent) -> Option<Message> {
    use crossterm::event::KeyCode;

    match key.code {
        // Quit
        KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),

        // Navigation - vim style
        KeyCode::Char('j') | KeyCode::Down => Some(Message::NavigateDown),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::NavigateUp),
        KeyCode::Char('h') | KeyCode::Left => Some(Message::NavigateLeft),
        KeyCode::Char('l') | KeyCode::Right => Some(Message::NavigateRight),

        // Selection
        KeyCode::Enter => Some(Message::Select),
        KeyCode::Tab => Some(Message::SwitchPane),

        // Scrolling
        KeyCode::PageUp => Some(Message::ScrollUp),
        KeyCode::PageDown => Some(Message::ScrollDown),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quit_sets_flag() {
        let mut state = AppState::new();
        assert!(!state.should_quit);

        update(&mut state, Message::Quit);

        assert!(state.should_quit);
    }

    #[test]
    fn test_navigate_down_increments_selection() {
        let mut state = AppState::new();
        assert_eq!(state.tree_state.selected(), Some(0));

        update(&mut state, Message::NavigateDown);

        assert_eq!(state.tree_state.selected(), Some(1));
    }

    #[test]
    fn test_navigate_up_at_zero_stays() {
        let mut state = AppState::new();
        assert_eq!(state.tree_state.selected(), Some(0));

        let changed = update(&mut state, Message::NavigateUp);

        assert!(!changed);
        assert_eq!(state.tree_state.selected(), Some(0));
    }

    #[test]
    fn test_switch_pane_toggles() {
        let mut state = AppState::new();
        assert_eq!(state.focused_pane, FocusedPane::Tree);

        update(&mut state, Message::SwitchPane);
        assert_eq!(state.focused_pane, FocusedPane::Detail);

        update(&mut state, Message::SwitchPane);
        assert_eq!(state.focused_pane, FocusedPane::Tree);
    }
}

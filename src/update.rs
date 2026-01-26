use crate::state::{AppState, FocusedPane, InputMode, Message};

/// TEA Update function - handles all state transitions
///
/// Takes the current state, a message, and the current tree length for bounds checking.
/// Returns whether state changed. This is the ONLY place state mutations happen (TEA pattern).
pub fn update(state: &mut AppState, message: Message, tree_len: usize) -> bool {
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
                if tree_len > 0 && current < tree_len.saturating_sub(1) {
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
                // In tree: collapse handled by app layer via maybe_convert_to_expand_message
                false
            }
        }

        Message::NavigateRight => {
            if state.focused_pane == FocusedPane::Tree {
                state.focused_pane = FocusedPane::Detail;
                true
            } else {
                false
            }
        }

        Message::Select => {
            // Enter key in tree: expand/collapse handled by app layer, fallback switches to detail
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

        Message::ToggleExpand(phase_num) => {
            state.toggle_expansion(phase_num);
            true
        }

        Message::ShowHelp => {
            state.input_mode = InputMode::Help;
            true
        }

        Message::HideHelp => {
            state.input_mode = InputMode::Normal;
            true
        }

        Message::EnterSearchMode => {
            state.input_mode = InputMode::Search;
            state.search_query.clear();
            state.search_matches.clear();
            state.current_match = 0;
            true
        }

        Message::ExitSearchMode => {
            state.input_mode = InputMode::Normal;
            // Preserve search_query for status display; cleared on next EnterSearchMode
            true
        }

        Message::SearchInput(c) => {
            state.search_query.push(c);
            // Matches recomputed in app layer (has access to tree_items for fuzzy matching)
            true
        }

        Message::SearchBackspace => {
            state.search_query.pop();
            true
        }

        Message::ConfirmSearch => {
            if let Some(&match_idx) = state.search_matches.get(state.current_match) {
                state.tree_state.select(Some(match_idx));
                state.selected_index = match_idx;
            }
            state.input_mode = InputMode::Normal;
            true
        }

        Message::NextMatch => {
            if !state.search_matches.is_empty() {
                state.current_match = (state.current_match + 1) % state.search_matches.len();
                if let Some(&match_idx) = state.search_matches.get(state.current_match) {
                    state.tree_state.select(Some(match_idx));
                    state.selected_index = match_idx;
                }
            }
            true
        }

        Message::PrevMatch => {
            if !state.search_matches.is_empty() {
                state.current_match = if state.current_match == 0 {
                    state.search_matches.len().saturating_sub(1)
                } else {
                    state.current_match - 1
                };
                if let Some(&match_idx) = state.search_matches.get(state.current_match) {
                    state.tree_state.select(Some(match_idx));
                    state.selected_index = match_idx;
                }
            }
            true
        }

        Message::Tick => {
            // TODO(Phase 3): Periodic data refresh from .planning/ files
            false
        }
    }
}

/// Convert keyboard event to Message based on current input mode
pub fn key_to_message(key: crossterm::event::KeyEvent, input_mode: InputMode) -> Option<Message> {
    use crossterm::event::KeyCode;

    match input_mode {
        InputMode::Normal => match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => Some(Message::Quit),

            // Help
            KeyCode::Char('?') => Some(Message::ShowHelp),

            // Search
            KeyCode::Char('/') => Some(Message::EnterSearchMode),

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
        },
        InputMode::Help => match key.code {
            KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => Some(Message::HideHelp),
            _ => None, // Ignore other keys in help mode
        },
        InputMode::Search => match key.code {
            KeyCode::Esc => Some(Message::ExitSearchMode),
            KeyCode::Enter => Some(Message::ConfirmSearch),
            KeyCode::Backspace => Some(Message::SearchBackspace),
            KeyCode::Char(c) => Some(Message::SearchInput(c)),
            KeyCode::Down | KeyCode::Tab => Some(Message::NextMatch),
            KeyCode::Up | KeyCode::BackTab => Some(Message::PrevMatch),
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TREE_LEN: usize = 10;

    #[test]
    fn test_quit_sets_flag() {
        let mut state = AppState::new();
        assert!(!state.should_quit);

        update(&mut state, Message::Quit, TEST_TREE_LEN);

        assert!(state.should_quit);
    }

    #[test]
    fn test_navigate_down_increments_selection() {
        let mut state = AppState::new();
        assert_eq!(state.tree_state.selected(), Some(0));

        update(&mut state, Message::NavigateDown, TEST_TREE_LEN);

        assert_eq!(state.tree_state.selected(), Some(1));
    }

    #[test]
    fn test_navigate_up_at_zero_stays() {
        let mut state = AppState::new();
        assert_eq!(state.tree_state.selected(), Some(0));

        let changed = update(&mut state, Message::NavigateUp, TEST_TREE_LEN);

        assert!(!changed);
        assert_eq!(state.tree_state.selected(), Some(0));
    }

    #[test]
    fn test_switch_pane_toggles() {
        let mut state = AppState::new();
        assert_eq!(state.focused_pane, FocusedPane::Tree);

        update(&mut state, Message::SwitchPane, TEST_TREE_LEN);
        assert_eq!(state.focused_pane, FocusedPane::Detail);

        update(&mut state, Message::SwitchPane, TEST_TREE_LEN);
        assert_eq!(state.focused_pane, FocusedPane::Tree);
    }

    #[test]
    fn test_navigate_down_respects_tree_bounds() {
        let mut state = AppState::new();
        state.tree_state.select(Some(2)); // At position 2
        state.selected_index = 2;

        // With tree_len=3, can't go past index 2
        let changed = update(&mut state, Message::NavigateDown, 3);

        assert!(!changed);
        assert_eq!(state.tree_state.selected(), Some(2));
    }

    #[test]
    fn test_navigate_down_empty_tree() {
        let mut state = AppState::new();

        let changed = update(&mut state, Message::NavigateDown, 0);

        assert!(!changed);
    }

    #[test]
    fn test_search_backspace_on_empty_is_safe() {
        let mut state = AppState::new();
        update(&mut state, Message::EnterSearchMode, TEST_TREE_LEN);

        // Backspace on empty string should not panic
        let changed = update(&mut state, Message::SearchBackspace, TEST_TREE_LEN);

        assert!(changed);
        assert!(state.search_query.is_empty());
    }

    #[test]
    fn test_confirm_search_with_no_matches() {
        let mut state = AppState::new();
        state.input_mode = InputMode::Search;
        state.search_matches = vec![]; // No matches

        let changed = update(&mut state, Message::ConfirmSearch, TEST_TREE_LEN);

        assert!(changed);
        assert_eq!(state.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_next_match_wraps_around() {
        let mut state = AppState::new();
        state.search_matches = vec![2, 5, 8];
        state.current_match = 2; // Last match

        update(&mut state, Message::NextMatch, TEST_TREE_LEN);

        assert_eq!(state.current_match, 0); // Wrapped to first
    }

    #[test]
    fn test_prev_match_wraps_from_zero() {
        let mut state = AppState::new();
        state.search_matches = vec![2, 5, 8];
        state.current_match = 0;

        update(&mut state, Message::PrevMatch, TEST_TREE_LEN);

        assert_eq!(state.current_match, 2); // Wrapped to last
    }

    #[test]
    fn test_show_hide_help() {
        let mut state = AppState::new();
        assert_eq!(state.input_mode, InputMode::Normal);

        update(&mut state, Message::ShowHelp, TEST_TREE_LEN);
        assert_eq!(state.input_mode, InputMode::Help);

        update(&mut state, Message::HideHelp, TEST_TREE_LEN);
        assert_eq!(state.input_mode, InputMode::Normal);
    }

    #[test]
    fn test_enter_search_clears_previous_state() {
        let mut state = AppState::new();
        state.search_query = "old query".to_string();
        state.search_matches = vec![1, 2, 3];
        state.current_match = 2;

        update(&mut state, Message::EnterSearchMode, TEST_TREE_LEN);

        assert!(state.search_query.is_empty());
        assert!(state.search_matches.is_empty());
        assert_eq!(state.current_match, 0);
        assert_eq!(state.input_mode, InputMode::Search);
    }
}

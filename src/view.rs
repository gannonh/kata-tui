use std::collections::HashSet;

use ratatui::Frame;

use crate::components::tree_view::{TreeItem, TreeView};
use crate::components::{DetailPane, HelpOverlay, SearchInput, StatusBar};
use crate::data::PlanningData;
use crate::layout::{compute_layout, is_terminal_too_small};
use crate::state::InputMode;
use crate::state::{AppState, FocusedPane};

/// Render the entire UI
///
/// This is the TEA View function - it renders current state to the terminal.
pub fn view(
    frame: &mut Frame,
    state: &mut AppState,
    data: &PlanningData,
    tree_items: &[TreeItem],
    phases_with_children: &HashSet<u8>,
) {
    let area = frame.area();

    // Check terminal size
    if is_terminal_too_small(area) {
        render_size_warning(frame);
        return;
    }

    let layout = compute_layout(area);

    // Render tree view (left pane)
    let tree_focused = state.focused_pane == FocusedPane::Tree;
    let tree_view = TreeView::new(
        tree_items,
        tree_focused,
        &state.expanded_phases,
        phases_with_children,
    );
    frame.render_stateful_widget(tree_view, layout.tree, &mut state.tree_state);

    // Get selected item for detail pane
    let selected_item = state.tree_state.selected().and_then(|i| tree_items.get(i));

    // Render detail pane (right pane)
    let detail_focused = state.focused_pane == FocusedPane::Detail;
    let detail_pane = DetailPane::new(selected_item, data, detail_focused, state.detail_scroll);
    frame.render_widget(detail_pane, layout.detail);

    // Render status bar (bottom)
    let status_bar = StatusBar::new(&data.state, state.focused_pane);
    frame.render_widget(status_bar, layout.status_bar);

    // Render overlays based on input mode
    match state.input_mode {
        InputMode::Help => {
            frame.render_widget(HelpOverlay::new(), area);
        }
        InputMode::Search => {
            // Render search input over the status bar area
            let search_input = SearchInput::new(
                &state.search_query,
                state.search_matches.len(),
                state.current_match,
            );
            frame.render_widget(search_input, layout.status_bar);
        }
        InputMode::Normal => {}
    }
}

/// Render terminal size warning
fn render_size_warning(frame: &mut Frame) {
    use ratatui::style::{Color, Style};
    use ratatui::widgets::Paragraph;

    let warning = Paragraph::new("Terminal too small. Please resize to at least 60x16.")
        .style(Style::default().fg(Color::Red));

    frame.render_widget(warning, frame.area());
}

// Re-export for use by app.rs
pub use crate::components::tree_view::{build_tree_items, phases_with_requirements};

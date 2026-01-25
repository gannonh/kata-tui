use ratatui::layout::{Constraint, Direction, Layout as RatatuiLayout, Rect};

/// Computed layout areas
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    /// Left pane (tree view) - 30% width
    pub tree: Rect,
    /// Right pane (detail) - 70% width
    pub detail: Rect,
    /// Bottom status bar - 1 line
    pub status_bar: Rect,
}

/// Minimum terminal size for proper display
pub const MIN_WIDTH: u16 = 60;
pub const MIN_HEIGHT: u16 = 16;

/// Narrow terminal threshold (reduce tree to 25%)
pub const NARROW_WIDTH: u16 = 80;

/// Compute layout for the given terminal area
pub fn compute_layout(area: Rect) -> Layout {
    // First split: main area and status bar
    let vertical = RatatuiLayout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(area);

    let main_area = vertical[0];
    let status_bar = vertical[1];

    // Determine tree width percentage based on terminal width
    let tree_percent = if area.width < NARROW_WIDTH { 25 } else { 30 };

    // Second split: tree and detail panes
    let horizontal = RatatuiLayout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(tree_percent),
            Constraint::Percentage(100 - tree_percent),
        ])
        .split(main_area);

    Layout {
        tree: horizontal[0],
        detail: horizontal[1],
        status_bar,
    }
}

/// Check if terminal is too small
pub fn is_terminal_too_small(area: Rect) -> bool {
    area.width < MIN_WIDTH || area.height < MIN_HEIGHT
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_layout_standard() {
        let area = Rect::new(0, 0, 100, 30);
        let layout = compute_layout(area);

        // Tree should be ~30%
        assert!(layout.tree.width >= 29 && layout.tree.width <= 31);
        // Detail should be ~70%
        assert!(layout.detail.width >= 69 && layout.detail.width <= 71);
        // Status bar should be 1 line
        assert_eq!(layout.status_bar.height, 1);
    }

    #[test]
    fn test_compute_layout_narrow() {
        let area = Rect::new(0, 0, 70, 20);
        let layout = compute_layout(area);

        // Tree should be ~25% for narrow terminals
        assert!(layout.tree.width >= 16 && layout.tree.width <= 19);
    }

    #[test]
    fn test_terminal_too_small() {
        assert!(is_terminal_too_small(Rect::new(0, 0, 50, 20)));
        assert!(is_terminal_too_small(Rect::new(0, 0, 80, 10)));
        assert!(!is_terminal_too_small(Rect::new(0, 0, 80, 20)));
    }
}

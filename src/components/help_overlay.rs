use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

/// Help overlay showing all keybindings
pub struct HelpOverlay;

impl HelpOverlay {
    pub fn new() -> Self {
        Self
    }

    /// Calculate centered popup area
    fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }
}

impl Default for HelpOverlay {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for HelpOverlay {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let popup_area = Self::popup_area(area, 60, 70);
        Clear.render(popup_area, buf);

        let help_lines = vec![
            Line::from(Span::styled(
                "Keybindings",
                Style::default().add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            )),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Navigation",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from("  j / Down      Move down"),
            Line::from("  k / Up        Move up"),
            Line::from("  h / Left      Collapse / Move to tree"),
            Line::from("  l / Right     Expand / Move to detail"),
            Line::from("  Enter         Toggle expand / Select"),
            Line::from("  Tab           Switch pane focus"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Scrolling",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from("  PageUp        Scroll detail up"),
            Line::from("  PageDown      Scroll detail down"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Actions",
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from("  /             Search / Filter"),
            Line::from("  ?             Toggle this help"),
            Line::from("  q / Esc       Quit (or close overlay)"),
            Line::from(""),
            Line::from(Span::styled(
                "Press ? or Esc to close",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let help = Paragraph::new(help_lines)
            .block(
                Block::default()
                    .title(" Help ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Cyan)),
            )
            .style(Style::default().bg(Color::Black));

        help.render(popup_area, buf);
    }
}

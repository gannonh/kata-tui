use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Search input widget displayed at bottom of screen
pub struct SearchInput<'a> {
    query: &'a str,
    match_count: usize,
    current_match: usize,
}

impl<'a> SearchInput<'a> {
    pub fn new(query: &'a str, match_count: usize, current_match: usize) -> Self {
        Self {
            query,
            match_count,
            current_match,
        }
    }
}

impl Widget for SearchInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let match_info = if self.match_count > 0 {
            format!(" [{}/{}]", self.current_match + 1, self.match_count)
        } else if !self.query.is_empty() {
            " [no matches]".to_string()
        } else {
            String::new()
        };

        let content = format!("/{}{}", self.query, match_info);

        let style = if self.match_count > 0 || self.query.is_empty() {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::Red)
        };

        let search_bar = Paragraph::new(content).style(style).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" Search (Esc to cancel, Enter to confirm) "),
        );

        search_bar.render(area, buf);
    }
}

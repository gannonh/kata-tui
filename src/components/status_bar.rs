use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use crate::data::PlanningState;
use crate::state::FocusedPane;

/// Status bar widget showing current state and keybinding hints
pub struct StatusBar<'a> {
    state: &'a PlanningState,
    focused_pane: FocusedPane,
}

impl<'a> StatusBar<'a> {
    pub fn new(state: &'a PlanningState, focused_pane: FocusedPane) -> Self {
        Self {
            state,
            focused_pane,
        }
    }
}

impl Widget for StatusBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let pane_indicator = match self.focused_pane {
            FocusedPane::Tree => "Tree",
            FocusedPane::Detail => "Detail",
        };

        let phase_info = if self.state.current_phase > 0 {
            format!(
                "Phase {} | {} | {}",
                self.state.current_phase, self.state.current_phase_name, self.state.status
            )
        } else {
            "No project loaded".to_string()
        };

        let line = Line::from(vec![
            Span::styled(
                format!(" {} ", pane_indicator),
                Style::default().fg(Color::Black).bg(Color::Cyan),
            ),
            Span::raw(" "),
            Span::styled(phase_info, Style::default().fg(Color::White)),
            Span::raw(" | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(":quit "),
            Span::styled("j/k", Style::default().fg(Color::Yellow)),
            Span::raw(":nav "),
            Span::styled("Tab", Style::default().fg(Color::Yellow)),
            Span::raw(":switch "),
        ]);

        let paragraph = Paragraph::new(line).style(Style::default().bg(Color::DarkGray));

        paragraph.render(area, buf);
    }
}

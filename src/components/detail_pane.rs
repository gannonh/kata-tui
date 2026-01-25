use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::components::tree_view::TreeItem;
use crate::data::PlanningData;

/// Detail pane widget showing selected item content
pub struct DetailPane<'a> {
    selected_item: Option<&'a TreeItem>,
    data: &'a PlanningData,
    focused: bool,
    scroll: u16,
}

impl<'a> DetailPane<'a> {
    pub fn new(
        selected_item: Option<&'a TreeItem>,
        data: &'a PlanningData,
        focused: bool,
        scroll: u16,
    ) -> Self {
        Self {
            selected_item,
            data,
            focused,
            scroll,
        }
    }

    fn build_content(&self) -> Text<'static> {
        match self.selected_item {
            None => Text::raw("No item selected"),
            Some(TreeItem::Project(_)) => self.build_project_content(),
            Some(TreeItem::Phase(phase)) => self.build_phase_content(phase),
            Some(TreeItem::Requirement {
                requirement,
                phase_num,
            }) => self.build_requirement_content(requirement, *phase_num),
        }
    }

    fn build_project_content(&self) -> Text<'static> {
        let project = &self.data.project;
        let mut lines = vec![
            Line::from(vec![Span::styled(
                project.name.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
        ];

        if !project.description.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Core Value: ",
                Style::default().fg(Color::Yellow),
            )]));
            lines.push(Line::from(project.description.clone()));
            lines.push(Line::from(""));
        }

        if !project.problem.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Problem: ",
                Style::default().fg(Color::Yellow),
            )]));
            lines.push(Line::from(project.problem.clone()));
            lines.push(Line::from(""));
        }

        if !project.solution.is_empty() {
            lines.push(Line::from(vec![Span::styled(
                "Solution: ",
                Style::default().fg(Color::Yellow),
            )]));
            lines.push(Line::from(project.solution.clone()));
        }

        Text::from(lines)
    }

    fn build_phase_content(&self, phase: &crate::data::Phase) -> Text<'static> {
        let percentage = phase.completion_percentage();
        let progress_color = if percentage >= 100.0 {
            Color::Green
        } else if percentage > 0.0 {
            Color::Yellow
        } else {
            Color::DarkGray
        };

        let mut lines = vec![
            Line::from(vec![Span::styled(
                format!("Phase {}: {}", phase.number, phase.name),
                Style::default().add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
        ];

        // Add visual progress bar
        let bar_width: usize = 20;
        let filled = ((percentage / 100.0) * bar_width as f32).round() as usize;
        let empty = bar_width.saturating_sub(filled);
        let progress_bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        lines.push(Line::from(vec![
            Span::styled("Progress: ", Style::default().fg(Color::Yellow)),
            Span::styled(progress_bar, Style::default().fg(progress_color)),
            Span::raw(" "),
            Span::styled(format!("{:.0}%", percentage), Style::default().fg(progress_color)),
        ]));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![Span::styled(
            "Goal: ",
            Style::default().fg(Color::Yellow),
        )]));
        lines.push(Line::from(phase.goal.clone()));
        lines.push(Line::from(""));
        lines.push(Line::from(vec![Span::styled(
            "Requirements:",
            Style::default().fg(Color::Yellow),
        )]));

        for req in &phase.requirements {
            let status = match req.status {
                crate::data::RequirementStatus::Complete => "[x]",
                crate::data::RequirementStatus::InProgress => "[~]",
                crate::data::RequirementStatus::Pending => "[ ]",
            };
            let status_color = req.status.color();
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(status, Style::default().fg(status_color)),
                Span::raw(" "),
                Span::styled(req.id.clone(), Style::default().fg(Color::Cyan)),
                Span::raw(": "),
                Span::raw(req.description.clone()),
            ]));
        }

        // Add completion stats
        let complete = phase
            .requirements
            .iter()
            .filter(|r| r.status == crate::data::RequirementStatus::Complete)
            .count();
        let total = phase.requirements.len();
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("Summary: ", Style::default().fg(Color::Yellow)),
            Span::raw(format!("{}/{} requirements complete", complete, total)),
        ]));

        Text::from(lines)
    }

    fn build_requirement_content(
        &self,
        req: &crate::data::Requirement,
        phase_num: u8,
    ) -> Text<'static> {
        let status_text = match req.status {
            crate::data::RequirementStatus::Complete => "Complete",
            crate::data::RequirementStatus::InProgress => "In Progress",
            crate::data::RequirementStatus::Pending => "Pending",
        };

        let status_color = match req.status {
            crate::data::RequirementStatus::Complete => Color::Green,
            crate::data::RequirementStatus::InProgress => Color::Yellow,
            crate::data::RequirementStatus::Pending => Color::Gray,
        };

        Text::from(vec![
            Line::from(vec![Span::styled(
                req.id.clone(),
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .fg(Color::Cyan),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(status_text, Style::default().fg(status_color)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Phase: ", Style::default().fg(Color::Yellow)),
                Span::raw(format!("{}", phase_num)),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Description:",
                Style::default().fg(Color::Yellow),
            )]),
            Line::from(req.description.clone()),
        ])
    }
}

impl Widget for DetailPane<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let title: &str = match self.selected_item {
            Some(TreeItem::Project(_)) => " Project Details ",
            Some(TreeItem::Phase(p)) => {
                // Use a static string since we can't create dynamic titles easily
                match p.number {
                    1 => " Phase 1 ",
                    2 => " Phase 2 ",
                    3 => " Phase 3 ",
                    4 => " Phase 4 ",
                    5 => " Phase 5 ",
                    _ => " Phase ",
                }
            }
            Some(TreeItem::Requirement { requirement, .. }) => {
                // For simplicity, use a generic title
                if requirement.id.starts_with("DISP") {
                    " DISP Requirement "
                } else if requirement.id.starts_with("NAV") {
                    " NAV Requirement "
                } else if requirement.id.starts_with("PLAT") {
                    " PLAT Requirement "
                } else if requirement.id.starts_with("REAL") {
                    " REAL Requirement "
                } else if requirement.id.starts_with("REND") {
                    " REND Requirement "
                } else if requirement.id.starts_with("CMD") {
                    " CMD Requirement "
                } else if requirement.id.starts_with("DIST") {
                    " DIST Requirement "
                } else {
                    " Requirement "
                }
            }
            None => " Details ",
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(border_style);

        let content = self.build_content();
        let paragraph = Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: true })
            .scroll((self.scroll, 0));

        paragraph.render(area, buf);
    }
}

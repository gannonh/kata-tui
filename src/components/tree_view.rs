use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, StatefulWidget},
};

use crate::data::{Phase, PlanningData, Requirement};

/// Tree item types for the hierarchical view
#[derive(Debug, Clone)]
pub enum TreeItem {
    Project(String),
    Phase(Phase),
    Requirement {
        phase_num: u8,
        requirement: Requirement,
    },
}

impl TreeItem {
    pub fn to_list_item(&self) -> ListItem<'static> {
        match self {
            TreeItem::Project(name) => ListItem::new(Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(name.clone(), Style::default().add_modifier(Modifier::BOLD)),
            ])),
            TreeItem::Phase(phase) => {
                let status_icon = match phase.status {
                    crate::data::PhaseStatus::Complete => "[x]",
                    crate::data::PhaseStatus::InProgress => "[~]",
                    crate::data::PhaseStatus::Pending => "[ ]",
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("  {} ", status_icon)),
                    Span::styled(
                        format!("Phase {}: {}", phase.number, phase.name),
                        Style::default().add_modifier(Modifier::BOLD),
                    ),
                ]))
            }
            TreeItem::Requirement { requirement, .. } => {
                let status_icon = match requirement.status {
                    crate::data::RequirementStatus::Complete => "[x]",
                    crate::data::RequirementStatus::InProgress => "[~]",
                    crate::data::RequirementStatus::Pending => "[ ]",
                };
                ListItem::new(Line::from(vec![
                    Span::raw(format!("    {} ", status_icon)),
                    Span::styled(requirement.id.clone(), Style::default().fg(Color::Cyan)),
                    Span::raw(": "),
                    Span::raw(truncate_text(&requirement.description, 30)),
                ]))
            }
        }
    }
}

/// Truncate text to max_len with ellipsis
fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len.saturating_sub(3)])
    }
}

/// Build flat list of tree items from planning data
pub fn build_tree_items(data: &PlanningData) -> Vec<TreeItem> {
    let mut items = Vec::new();

    // Add project as root
    if !data.project.name.is_empty() {
        items.push(TreeItem::Project(data.project.name.clone()));
    }

    // Add phases and their requirements
    for phase in &data.roadmap.phases {
        items.push(TreeItem::Phase(phase.clone()));

        for req in &phase.requirements {
            items.push(TreeItem::Requirement {
                phase_num: phase.number,
                requirement: req.clone(),
            });
        }
    }

    items
}

/// Tree view widget
pub struct TreeView<'a> {
    items: &'a [TreeItem],
    focused: bool,
}

impl<'a> TreeView<'a> {
    pub fn new(items: &'a [TreeItem], focused: bool) -> Self {
        Self { items, focused }
    }
}

impl<'a> StatefulWidget for TreeView<'a> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let border_style = if self.focused {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(" Project ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let list_items: Vec<ListItem> = self.items.iter().map(|i| i.to_list_item()).collect();

        let list = List::new(list_items)
            .block(block)
            .highlight_style(
                Style::default()
                    .bg(Color::DarkGray)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        StatefulWidget::render(list, area, buf, state);
    }
}

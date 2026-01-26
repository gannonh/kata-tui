use std::collections::HashSet;

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
    /// Get the phase number if this item is a Phase
    pub fn phase_number(&self) -> Option<u8> {
        match self {
            TreeItem::Phase(phase) => Some(phase.number),
            _ => None,
        }
    }

    /// Convert to a ratatui ListItem for rendering
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
                    Span::raw("  "),
                    Span::styled(status_icon, Style::default().fg(phase.status.color())),
                    Span::raw(" "),
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
                    Span::raw("    "),
                    Span::styled(status_icon, Style::default().fg(requirement.status.color())),
                    Span::raw(" "),
                    Span::styled(requirement.id.clone(), Style::default().fg(Color::Cyan)),
                    Span::raw(": "),
                    Span::raw(truncate_text(&requirement.description, 30)),
                ]))
            }
        }
    }
}

/// Truncate text to max_len characters with ellipsis (UTF-8 safe)
fn truncate_text(text: &str, max_len: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_len {
        text.to_string()
    } else if max_len <= 3 {
        // Not enough room for text + ellipsis, just take first max_len chars
        text.chars().take(max_len).collect()
    } else {
        let truncated: String = text.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

/// Build flat list of tree items from planning data, respecting expand state
pub fn build_tree_items(data: &PlanningData, expanded: &HashSet<u8>) -> Vec<TreeItem> {
    let mut items = Vec::new();

    // Add project as root
    if !data.project.name.is_empty() {
        items.push(TreeItem::Project(data.project.name.clone()));
    }

    // Add phases and their requirements (only if expanded)
    for phase in &data.roadmap.phases {
        items.push(TreeItem::Phase(phase.clone()));

        // Only include requirements if this phase is expanded
        if expanded.contains(&phase.number) {
            for req in &phase.requirements {
                items.push(TreeItem::Requirement {
                    phase_num: phase.number,
                    requirement: req.clone(),
                });
            }
        }
    }

    items
}

/// Get set of phase numbers that have requirements
pub fn phases_with_requirements(data: &PlanningData) -> HashSet<u8> {
    data.roadmap
        .phases
        .iter()
        .filter(|p| !p.requirements.is_empty())
        .map(|p| p.number)
        .collect()
}

/// Tree view widget
pub struct TreeView<'a> {
    items: &'a [TreeItem],
    focused: bool,
    expanded: &'a HashSet<u8>,
    phases_with_children: &'a HashSet<u8>,
}

impl<'a> TreeView<'a> {
    pub fn new(
        items: &'a [TreeItem],
        focused: bool,
        expanded: &'a HashSet<u8>,
        phases_with_children: &'a HashSet<u8>,
    ) -> Self {
        Self {
            items,
            focused,
            expanded,
            phases_with_children,
        }
    }

    /// Get the expand indicator for a phase
    fn expand_icon(&self, phase_num: u8) -> &'static str {
        if !self.phases_with_children.contains(&phase_num) {
            "  " // No children, no indicator
        } else if self.expanded.contains(&phase_num) {
            "▼ " // Expanded
        } else {
            "▶ " // Collapsed
        }
    }
}

impl StatefulWidget for TreeView<'_> {
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

        // Build list items with expand awareness for phases
        let list_items: Vec<ListItem> = self
            .items
            .iter()
            .map(|item| match item {
                TreeItem::Phase(phase) => {
                    let status_icon = match phase.status {
                        crate::data::PhaseStatus::Complete => "[x]",
                        crate::data::PhaseStatus::InProgress => "[~]",
                        crate::data::PhaseStatus::Pending => "[ ]",
                    };
                    let expand_icon = self.expand_icon(phase.number);

                    // Progress percentage with color coding
                    let percentage = phase.completion_percentage();
                    let progress_color = if percentage >= 100.0 {
                        Color::Green
                    } else if percentage > 0.0 {
                        Color::Yellow
                    } else {
                        Color::DarkGray
                    };

                    ListItem::new(Line::from(vec![
                        Span::raw(expand_icon),
                        Span::styled(status_icon, Style::default().fg(phase.status.color())),
                        Span::raw(" "),
                        Span::styled(
                            format!("Phase {}: {}", phase.number, phase.name),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                        Span::raw(" "),
                        Span::styled(
                            format!("[{:3.0}%]", percentage),
                            Style::default().fg(progress_color),
                        ),
                    ]))
                }
                _ => item.to_list_item(),
            })
            .collect();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::{PhaseStatus, Project, Requirement, RequirementStatus, Roadmap};

    fn mock_data() -> PlanningData {
        PlanningData {
            project: Project {
                name: "Test".to_string(),
                ..Default::default()
            },
            roadmap: Roadmap {
                phases: vec![
                    Phase {
                        number: 1,
                        name: "Phase One".to_string(),
                        goal: "Test goal".to_string(),
                        requirements: vec![Requirement {
                            id: "REQ-01".to_string(),
                            description: "Test req".to_string(),
                            status: RequirementStatus::Pending,
                        }],
                        status: PhaseStatus::Pending,
                        dependencies: vec![],
                    },
                    Phase {
                        number: 2,
                        name: "Phase Two".to_string(),
                        goal: "Test goal 2".to_string(),
                        requirements: vec![],
                        status: PhaseStatus::Pending,
                        dependencies: vec![],
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_build_tree_collapsed_hides_requirements() {
        let data = mock_data();
        let expanded = HashSet::new(); // All collapsed

        let items = build_tree_items(&data, &expanded);

        // Should have: Project + 2 Phases = 3 items (no requirements)
        assert_eq!(items.len(), 3);
    }

    #[test]
    fn test_build_tree_expanded_shows_requirements() {
        let data = mock_data();
        let mut expanded = HashSet::new();
        expanded.insert(1); // Expand phase 1

        let items = build_tree_items(&data, &expanded);

        // Should have: Project + Phase1 + Req + Phase2 = 4 items
        assert_eq!(items.len(), 4);
    }

    #[test]
    fn test_phases_with_requirements() {
        let data = mock_data();

        let phases = phases_with_requirements(&data);

        // Only phase 1 has requirements
        assert!(phases.contains(&1));
        assert!(!phases.contains(&2));
    }
}

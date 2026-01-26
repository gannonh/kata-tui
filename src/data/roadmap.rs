use ratatui::style::Color;

/// Status of a requirement
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RequirementStatus {
    #[default]
    Pending,
    InProgress,
    Complete,
}

impl RequirementStatus {
    pub fn from_checkbox(checked: bool) -> Self {
        if checked {
            Self::Complete
        } else {
            Self::Pending
        }
    }

    /// Get the display color for this status
    pub fn color(&self) -> Color {
        match self {
            RequirementStatus::Complete => Color::Green,
            RequirementStatus::InProgress => Color::Yellow,
            RequirementStatus::Pending => Color::DarkGray,
        }
    }
}

/// A single requirement (e.g., DISP-01)
#[derive(Debug, Clone, Default)]
pub struct Requirement {
    /// Requirement ID (e.g., "DISP-01")
    pub id: String,
    /// Description text
    pub description: String,
    /// Current status
    pub status: RequirementStatus,
}

/// Status of a phase
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PhaseStatus {
    #[default]
    Pending,
    InProgress,
    Complete,
}

impl PhaseStatus {
    /// Get the display color for this status
    pub fn color(&self) -> Color {
        match self {
            PhaseStatus::Complete => Color::Green,
            PhaseStatus::InProgress => Color::Yellow,
            PhaseStatus::Pending => Color::DarkGray,
        }
    }
}

/// A project phase
#[derive(Debug, Clone, Default)]
pub struct Phase {
    /// Phase number (1, 2, 3...)
    pub number: u8,
    /// Phase name
    pub name: String,
    /// Phase goal
    pub goal: String,
    /// Requirements in this phase
    pub requirements: Vec<Requirement>,
    /// Phase status
    pub status: PhaseStatus,
    /// Dependency phase numbers
    pub dependencies: Vec<u8>,
}

impl Phase {
    /// Calculate completion percentage based on requirements
    pub fn completion_percentage(&self) -> f32 {
        if self.requirements.is_empty() {
            return 0.0;
        }
        let complete = self
            .requirements
            .iter()
            .filter(|r| r.status == RequirementStatus::Complete)
            .count();
        (complete as f32 / self.requirements.len() as f32) * 100.0
    }
}

/// Project roadmap containing all phases
#[derive(Debug, Clone, Default)]
pub struct Roadmap {
    /// Overview text
    pub overview: String,
    /// All phases
    pub phases: Vec<Phase>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_percentage_empty_requirements() {
        let phase = Phase::default();
        assert_eq!(phase.completion_percentage(), 0.0);
    }

    #[test]
    fn test_completion_percentage_all_complete() {
        let phase = Phase {
            requirements: vec![
                Requirement {
                    status: RequirementStatus::Complete,
                    ..Default::default()
                },
                Requirement {
                    status: RequirementStatus::Complete,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(phase.completion_percentage(), 100.0);
    }

    #[test]
    fn test_completion_percentage_none_complete() {
        let phase = Phase {
            requirements: vec![
                Requirement {
                    status: RequirementStatus::Pending,
                    ..Default::default()
                },
                Requirement {
                    status: RequirementStatus::InProgress,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        assert_eq!(phase.completion_percentage(), 0.0);
    }

    #[test]
    fn test_completion_percentage_partial() {
        let phase = Phase {
            requirements: vec![
                Requirement {
                    status: RequirementStatus::Complete,
                    ..Default::default()
                },
                Requirement {
                    status: RequirementStatus::Pending,
                    ..Default::default()
                },
                Requirement {
                    status: RequirementStatus::InProgress,
                    ..Default::default()
                },
            ],
            ..Default::default()
        };
        // 1 complete out of 3 = 33.33%
        let percentage = phase.completion_percentage();
        assert!((percentage - 33.33).abs() < 0.01);
    }

    #[test]
    fn test_status_colors() {
        assert_eq!(PhaseStatus::Complete.color(), Color::Green);
        assert_eq!(PhaseStatus::InProgress.color(), Color::Yellow);
        assert_eq!(PhaseStatus::Pending.color(), Color::DarkGray);

        assert_eq!(RequirementStatus::Complete.color(), Color::Green);
        assert_eq!(RequirementStatus::InProgress.color(), Color::Yellow);
        assert_eq!(RequirementStatus::Pending.color(), Color::DarkGray);
    }
}

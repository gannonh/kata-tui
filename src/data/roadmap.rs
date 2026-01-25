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
}

/// A single requirement (e.g., DISP-01)
#[derive(Debug, Clone)]
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

/// A project phase
#[derive(Debug, Clone)]
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

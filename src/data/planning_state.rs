/// Current project state from STATE.md
#[derive(Debug, Clone, Default)]
pub struct PlanningState {
    /// Current phase number
    pub current_phase: u8,
    /// Current phase name
    pub current_phase_name: String,
    /// Current plan within phase
    pub current_plan: Option<String>,
    /// Status (Planning, Executing, etc.)
    pub status: String,
    /// Progress percentage
    pub progress: u8,
    /// Total phases count
    pub total_phases: u8,
    /// Phases complete count
    pub phases_complete: u8,
    /// Total requirements count
    pub total_requirements: u16,
    /// Requirements complete count
    pub requirements_complete: u16,
}

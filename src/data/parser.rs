use std::path::Path;

use color_eyre::Result;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};

use super::{Phase, PhaseStatus, PlanningState, Project, Requirement, RequirementStatus, Roadmap};

/// Combined planning data from all files
#[derive(Debug, Clone, Default)]
pub struct PlanningData {
    pub project: Project,
    pub roadmap: Roadmap,
    pub state: PlanningState,
}

/// Load all planning data from a .planning/ directory
pub fn load_planning_data(planning_dir: &Path) -> Result<PlanningData> {
    let project = load_project(&planning_dir.join("PROJECT.md")).unwrap_or_default();
    let roadmap = load_roadmap(&planning_dir.join("ROADMAP.md")).unwrap_or_default();
    let state = load_state(&planning_dir.join("STATE.md")).unwrap_or_default();

    Ok(PlanningData {
        project,
        roadmap,
        state,
    })
}

/// Parse PROJECT.md
fn load_project(path: &Path) -> Result<Project> {
    let content = std::fs::read_to_string(path)?;
    let mut project = Project::default();

    let parser = Parser::new(&content);
    let mut current_section = String::new();
    let mut _in_heading = false;
    let mut heading_level = 0u8;
    let mut text_buffer = String::new();

    for event in parser {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                // Save previous section content
                if !current_section.is_empty() && !text_buffer.is_empty() {
                    match current_section.to_lowercase().as_str() {
                        s if s.contains("core value") => {
                            project.description = text_buffer.trim().to_string()
                        }
                        s if s.contains("problem") => {
                            project.problem = text_buffer.trim().to_string()
                        }
                        s if s.contains("solution") => {
                            project.solution = text_buffer.trim().to_string()
                        }
                        _ => {}
                    }
                }
                text_buffer.clear();
                _in_heading = true;
                heading_level = level as u8;
            }
            Event::End(TagEnd::Heading(_)) => {
                if heading_level == 1 && project.name.is_empty() {
                    project.name = text_buffer.trim().to_string();
                } else {
                    current_section = text_buffer.trim().to_string();
                }
                text_buffer.clear();
                _in_heading = false;
            }
            Event::Text(text) => {
                text_buffer.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                text_buffer.push('\n');
            }
            _ => {}
        }
    }

    // Handle last section
    if !current_section.is_empty() && !text_buffer.is_empty() {
        match current_section.to_lowercase().as_str() {
            s if s.contains("core value") => project.description = text_buffer.trim().to_string(),
            s if s.contains("problem") => project.problem = text_buffer.trim().to_string(),
            s if s.contains("solution") => project.solution = text_buffer.trim().to_string(),
            _ => {}
        }
    }

    Ok(project)
}

/// Parse ROADMAP.md
fn load_roadmap(path: &Path) -> Result<Roadmap> {
    let content = std::fs::read_to_string(path)?;
    let mut roadmap = Roadmap::default();

    // Simple line-by-line parsing for phase structure
    let mut current_phase: Option<Phase> = None;
    let mut in_requirements = false;
    let mut in_goal = false;
    let mut goal_buffer = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Phase header: ### Phase N: Name
        if trimmed.starts_with("### Phase") {
            // Save previous phase
            if let Some(mut phase) = current_phase.take() {
                if in_goal && !goal_buffer.is_empty() {
                    phase.goal = goal_buffer.trim().to_string();
                }
                roadmap.phases.push(phase);
            }

            // Parse new phase
            if let Some(rest) = trimmed.strip_prefix("### Phase") {
                let rest = rest.trim();
                // Format: "N: Name" or just "N"
                let parts: Vec<&str> = rest.splitn(2, ':').collect();
                let number = parts[0].trim().parse().unwrap_or(0);
                let name = parts
                    .get(1)
                    .map(|s| s.trim().to_string())
                    .unwrap_or_default();

                current_phase = Some(Phase {
                    number,
                    name,
                    goal: String::new(),
                    requirements: Vec::new(),
                    status: PhaseStatus::Pending,
                    dependencies: Vec::new(),
                });
                in_requirements = false;
                in_goal = false;
                goal_buffer.clear();
            }
        }
        // Goal line
        else if trimmed.starts_with("**Goal:**") {
            if let Some(ref mut phase) = current_phase {
                let goal_text = trimmed.strip_prefix("**Goal:**").unwrap_or("").trim();
                if goal_text.is_empty() {
                    in_goal = true;
                } else {
                    phase.goal = goal_text.to_string();
                }
            }
        }
        // Requirements section
        else if trimmed.starts_with("**Requirements:**") {
            in_requirements = true;
            in_goal = false;
            if !goal_buffer.is_empty() {
                if let Some(ref mut phase) = current_phase {
                    phase.goal = goal_buffer.trim().to_string();
                }
            }
            goal_buffer.clear();
        }
        // Requirement line: - REQ-ID: Description
        else if in_requirements && trimmed.starts_with("- ") {
            if let Some(ref mut phase) = current_phase {
                let req_text = trimmed.strip_prefix("- ").unwrap_or("");
                // Format: "REQ-ID: Description"
                if let Some(colon_pos) = req_text.find(':') {
                    let id = req_text[..colon_pos].trim().to_string();
                    let desc = req_text[colon_pos + 1..].trim().to_string();
                    phase.requirements.push(Requirement {
                        id,
                        description: desc,
                        status: RequirementStatus::Pending,
                    });
                }
            }
        }
        // Section divider ends requirements
        else if trimmed == "---" {
            in_requirements = false;
            in_goal = false;
        }
        // Collect goal text
        else if in_goal && !trimmed.is_empty() {
            goal_buffer.push_str(trimmed);
            goal_buffer.push(' ');
        }
    }

    // Save last phase
    if let Some(mut phase) = current_phase.take() {
        if !goal_buffer.is_empty() {
            phase.goal = goal_buffer.trim().to_string();
        }
        roadmap.phases.push(phase);
    }

    Ok(roadmap)
}

/// Parse STATE.md
fn load_state(path: &Path) -> Result<PlanningState> {
    let content = std::fs::read_to_string(path)?;
    let mut state = PlanningState::default();

    for line in content.lines() {
        let trimmed = line.trim();

        // **Phase:** N - Name
        if trimmed.starts_with("**Phase:**") {
            let rest = trimmed.strip_prefix("**Phase:**").unwrap_or("").trim();
            // Format: "N - Name"
            let parts: Vec<&str> = rest.splitn(2, '-').collect();
            state.current_phase = parts[0].trim().parse().unwrap_or(0);
            state.current_phase_name = parts
                .get(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
        }
        // **Plan:** value
        else if trimmed.starts_with("**Plan:**") {
            let value = trimmed.strip_prefix("**Plan:**").unwrap_or("").trim();
            if !value.is_empty() && value != "Not yet created" {
                state.current_plan = Some(value.to_string());
            }
        }
        // **Status:** value
        else if trimmed.starts_with("**Status:**") {
            state.status = trimmed
                .strip_prefix("**Status:**")
                .unwrap_or("")
                .trim()
                .to_string();
        }
        // Table row parsing for metrics
        else if trimmed.starts_with("| Total Phases |") {
            if let Some(value) = extract_table_value(trimmed) {
                state.total_phases = value.parse().unwrap_or(0);
            }
        } else if trimmed.starts_with("| Phases Complete |") {
            if let Some(value) = extract_table_value(trimmed) {
                state.phases_complete = value.parse().unwrap_or(0);
            }
        } else if trimmed.starts_with("| v1 Requirements |")
            || trimmed.starts_with("| Total Requirements |")
        {
            if let Some(value) = extract_table_value(trimmed) {
                state.total_requirements = value.parse().unwrap_or(0);
            }
        } else if trimmed.starts_with("| Requirements Complete |") {
            if let Some(value) = extract_table_value(trimmed) {
                state.requirements_complete = value.parse().unwrap_or(0);
            }
        }
    }

    Ok(state)
}

/// Extract value from markdown table row: | Key | Value |
fn extract_table_value(line: &str) -> Option<&str> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() >= 3 {
        Some(parts[2].trim())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_load_project_basic() {
        let dir = tempdir().unwrap();
        let project_path = dir.path().join("PROJECT.md");
        std::fs::write(
            &project_path,
            "# Test Project\n\n## Core Value\n\nA test project.\n",
        )
        .unwrap();

        let project = load_project(&project_path).unwrap();

        assert_eq!(project.name, "Test Project");
        assert_eq!(project.description, "A test project.");
    }

    #[test]
    fn test_load_roadmap_phases() {
        let dir = tempdir().unwrap();
        let roadmap_path = dir.path().join("ROADMAP.md");
        std::fs::write(
            &roadmap_path,
            r#"# Roadmap

## Overview

Test roadmap.

### Phase 1: Foundation

**Goal:** Build the foundation.

**Requirements:**
- DISP-01: Display stuff
- NAV-01: Navigate stuff

---

### Phase 2: Features

**Goal:** Add features.

**Requirements:**
- FEAT-01: Add feature

---
"#,
        )
        .unwrap();

        let roadmap = load_roadmap(&roadmap_path).unwrap();

        assert_eq!(roadmap.phases.len(), 2);
        assert_eq!(roadmap.phases[0].number, 1);
        assert_eq!(roadmap.phases[0].name, "Foundation");
        assert_eq!(roadmap.phases[0].requirements.len(), 2);
        assert_eq!(roadmap.phases[0].requirements[0].id, "DISP-01");
    }

    #[test]
    fn test_load_state_metrics() {
        let dir = tempdir().unwrap();
        let state_path = dir.path().join("STATE.md");
        std::fs::write(
            &state_path,
            r#"# Project State

**Phase:** 1 - Foundation
**Status:** Planning

| Metric | Value |
|--------|-------|
| Total Phases | 5 |
| Phases Complete | 1 |
"#,
        )
        .unwrap();

        let state = load_state(&state_path).unwrap();

        assert_eq!(state.current_phase, 1);
        assert_eq!(state.current_phase_name, "Foundation");
        assert_eq!(state.total_phases, 5);
        assert_eq!(state.phases_complete, 1);
    }

    #[test]
    fn test_missing_file_returns_default() {
        let dir = tempdir().unwrap();
        let data = load_planning_data(dir.path()).unwrap();

        // Should return defaults, not error
        assert!(data.project.name.is_empty());
        assert!(data.roadmap.phases.is_empty());
    }
}

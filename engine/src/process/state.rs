// These are the only lifecycle states an agent execution can occupy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    // Config is being loaded and validated.
    Loading,
    // Agent is valid, but not executing yet.
    Ready,
    // Agent may consume tools, memory, tokens, and runtime resources.
    Running,
    // Agent keeps its state, but active work should stop.
    Paused,
    // Successful terminal state. Runtime resources should be cleaned up.
    Done,
    // Failed terminal state. Error context should be preserved.
    Failed,
}

// Central gate for lifecycle movement and capability cleanup.
pub fn is_valid_transition(from: AgentState, to: AgentState) -> bool {
    // Terminal states intentionally have no outgoing transitions.
    matches!(
        (from, to),
        (AgentState::Loading, AgentState::Ready)
            | (AgentState::Ready, AgentState::Running)
            | (AgentState::Running, AgentState::Paused)
            | (AgentState::Paused, AgentState::Running)
            | (AgentState::Running, AgentState::Done)
            | (AgentState::Running, AgentState::Failed)
            | (AgentState::Paused, AgentState::Failed)
    )
}

#[cfg(test)]
// These tests document which lifecycle jumps are allowed.
mod tests {
    use super::{AgentState, is_valid_transition};

    #[test]
    fn allows_normal_lifecycle_transitions() {
        assert!(is_valid_transition(AgentState::Loading, AgentState::Ready));
        assert!(is_valid_transition(AgentState::Ready, AgentState::Running));
        assert!(is_valid_transition(AgentState::Running, AgentState::Paused));
        assert!(is_valid_transition(AgentState::Paused, AgentState::Running));
        assert!(is_valid_transition(AgentState::Running, AgentState::Done));
        assert!(is_valid_transition(AgentState::Running, AgentState::Failed));
        assert!(is_valid_transition(AgentState::Paused, AgentState::Failed));
    }

    #[test]
    fn keeps_terminal_states_terminal() {
        assert!(!is_valid_transition(AgentState::Done, AgentState::Running));
        assert!(!is_valid_transition(AgentState::Done, AgentState::Failed));
        assert!(!is_valid_transition(AgentState::Failed, AgentState::Running));
        assert!(!is_valid_transition(AgentState::Failed, AgentState::Done));
    }

    #[test]
    fn rejects_skipped_lifecycle_transitions() {
        assert!(!is_valid_transition(AgentState::Loading, AgentState::Running));
        assert!(!is_valid_transition(AgentState::Ready, AgentState::Done));
        assert!(!is_valid_transition(AgentState::Paused, AgentState::Done));
    }
}

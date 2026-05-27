use crate::process::{AgentProcess, AgentState};

// Control actions change an already-started process without ending it.
impl AgentProcess {
    pub fn pause(&mut self) -> Result<(), String> {
        // Paused keeps process state, but active work should stop.
        self.transition_to(AgentState::Paused)
    }

    pub fn resume(&mut self) -> Result<(), String> {
        // Resume returns a paused process back to active execution.
        self.transition_to(AgentState::Running)
    }
}

#[cfg(test)]
// Control tests prove pause/resume only work at the right stages.
mod tests {
    use crate::process::{AgentProcess, AgentState};
    use crate::test_support::config_factory::{running_agent, test_config};

    #[test]
    fn pause_moves_running_process_to_paused() {
        let mut agent = running_agent();

        let result = agent.pause();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Paused);
    }

    #[test]
    fn pause_rejects_process_that_is_not_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.pause();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Loading);
    }

    #[test]
    fn resume_moves_paused_process_to_running() {
        let mut agent = running_agent();
        agent.pause().expect("pause should work from running");

        let result = agent.resume();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[test]
    fn resume_rejects_process_that_is_not_paused() {
        let mut agent = running_agent();

        let result = agent.resume();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Running);
    }
}

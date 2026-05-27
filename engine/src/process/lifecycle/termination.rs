use crate::process::{AgentProcess, AgentState};

// Termination actions close an execution path.
impl AgentProcess {
    pub fn complete(&mut self) -> Result<(), String> {
        // Done means successful completion and later resource cleanup.
        self.transition_to(AgentState::Done)
    }

    pub fn fail(&mut self) -> Result<(), String> {
        // Failed preserves error context for logs and restart decisions.
        self.transition_to(AgentState::Failed)
    }
}

#[cfg(test)]
// Termination tests prove terminal states cannot be revived.
mod tests {
    use crate::process::AgentState;
    use crate::test_support::config_factory::running_agent;

    #[test]
    fn complete_moves_running_process_to_done() {
        let mut agent = running_agent();

        let result = agent.complete();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Done);
    }

    #[test]
    fn complete_rejects_paused_process() {
        let mut agent = running_agent();
        agent.pause().expect("pause should work from running");

        let result = agent.complete();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Paused);
    }

    #[test]
    fn fail_moves_running_process_to_failed() {
        let mut agent = running_agent();

        let result = agent.fail();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Failed);
    }

    #[test]
    fn fail_moves_paused_process_to_failed() {
        let mut agent = running_agent();
        agent.pause().expect("pause should work from running");

        let result = agent.fail();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Failed);
    }

    #[test]
    fn terminal_process_cannot_resume() {
        let mut agent = running_agent();
        agent.complete().expect("complete should work from running");

        let result = agent.resume();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Done);
    }
}

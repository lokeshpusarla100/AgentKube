use crate::process::{AgentProcess, AgentState};

// Startup moves a validated process toward active execution.
impl AgentProcess {
    pub fn load(&mut self) -> Result<(), String> {
        // Loading finishes by making the process ready to run.
        self.transition_to(AgentState::Ready)
    }

    pub fn start(&mut self) -> Result<(), String> {
        // Running is where tools, memory, and token budgets become active.
        self.transition_to(AgentState::Running)
    }
}

#[cfg(test)]
// Startup tests prove callers cannot skip required stages.
mod tests {
    use crate::process::{AgentProcess, AgentState};
    use crate::test_support::config_factory::test_config;

    #[test]
    fn load_moves_process_to_ready() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.load();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Ready);
    }

    #[test]
    fn start_moves_ready_process_to_running() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");
        agent.load().expect("load should move to ready");

        let result = agent.start();

        assert!(result.is_ok());
        assert_eq!(agent.state(), AgentState::Running);
    }

    #[test]
    fn start_rejects_process_that_is_not_ready() {
        let mut agent = AgentProcess::from_config(test_config()).expect("config should be valid");

        let result = agent.start();

        assert!(result.is_err());
        assert_eq!(agent.state(), AgentState::Loading);
    }
}
